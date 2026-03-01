use std::collections::BTreeMap;
use crate::ast::{Program, Module, Stmt, Expr, Literal, BinaryOp, UnaryOp, MatchArm, Pattern};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Unit,
    Closure(Vec<String>, Box<Expr>, BTreeMap<String, Value>),
    OracleFunction(crate::ast::FunctionDecl),
    MeshFunction(crate::ast::FunctionDecl),
    List(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Identifier(String),
    Port(String),   // Connection ID/URI
    Stream(String), // Stream URI
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Unit, Value::Unit) => true,
            (Value::OracleFunction(a), Value::OracleFunction(b)) => a.name == b.name,
            (Value::MeshFunction(a), Value::MeshFunction(b)) => a.name == b.name,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            (Value::Identifier(a), Value::Identifier(b)) => a == b,
            (Value::Port(a), Value::Port(b)) => a == b,
            (Value::Stream(a), Value::Stream(b)) => a == b,
            _ => false,
        }
    }
}

pub struct ProgramEvaluator {
    pub globals: BTreeMap<String, Value>,
    pub environment: BTreeMap<String, Value>,
    pub return_signal: Option<Value>,
    pub is_worker_mode: bool,
}

impl Default for ProgramEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramEvaluator {
    pub fn new() -> Self {
        ProgramEvaluator {
            globals: BTreeMap::new(),
            environment: BTreeMap::new(),
            return_signal: None,
            is_worker_mode: false,
        }
    }

    pub fn evaluate_program(&mut self, program: &Program) -> Result<Value, String> {
        for module in &program.modules {
            self.evaluate_module(module)?;
        }
        Ok(Value::Unit)
    }

    pub fn call_function_by_name(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        if let Some(func) = self.globals.get(name).cloned() {
            self.apply_closure_value(&func, args)
        } else {
            Err(format!("Function {} not found securely mapped", name))
        }
    }

    fn evaluate_module(&mut self, module: &Module) -> Result<(), String> {
        for item in &module.items {
            match item {
                crate::ast::Item::Function(func) => {
                    let is_oracle = func.decorators.iter().any(|d| d.name == "oracle");
                    let is_mesh = func.decorators.iter().any(|d| d.name == "mesh");
                    
                    if is_oracle {
                        self.globals.insert(func.name.clone(), Value::OracleFunction(func.clone()));
                    } else if is_mesh {
                        self.globals.insert(func.name.clone(), Value::MeshFunction(func.clone()));
                    } else if let Some(body) = &func.body {
                        let closure = Value::Closure(
                            func.params.iter().map(|p| p.name.clone()).collect(),
                            Box::new(Expr::Block(body.clone())),
                            BTreeMap::new(), 
                        );
                        self.globals.insert(func.name.clone(), closure);
                    }
                }
                crate::ast::Item::Const(c) => {
                    let val = self.evaluate_expression(&c.value)?;
                    self.globals.insert(c.name.clone(), val);
                }
                _ => {} // Structs are currently types only, not runtime values
            }
        }
        
        // After registering all items, if there's a "main" constant or function, we can execute it if needed.
        // But main() is handled by the caller (CLI) by looking at the evaluated result or specific main item.
        Ok(())
    }

    // Helper for testing expression evaluation directly
    pub fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => self.evaluate_literal(lit),
            Expr::Identifier(name) => {
                self.environment.get(name)
                    .or_else(|| self.globals.get(name))
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::BinaryOp(left, op, right) => {
                if *op == BinaryOp::Assign {
                    let val = self.evaluate_expression(right)?;
                    self.assign_to_expr(left, val.clone())?;
                    return Ok(val);
                }
                
                let left_val = self.evaluate_expression(left)?;
                
                // For Dot access, we don't evaluate the right side in the environment
                if *op == BinaryOp::Dot {
                    if let Expr::Identifier(field_name) = &**right {
                        return self.evaluate_binary_op(left_val, op, Value::String(field_name.clone()));
                    } else {
                        return Err("Right side of '.' must be an identifier".to_string());
                    }
                }

                // Short-circuiting for Logical And/Or
                if *op == BinaryOp::And {
                    if let Value::Bool(false) = left_val {
                        return Ok(Value::Bool(false));
                    }
                } else if *op == BinaryOp::Or {
                    if let Value::Bool(true) = left_val {
                        return Ok(Value::Bool(true));
                    }
                }

                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(left_val, op, right_val)
            }
            Expr::UnaryOp(op, expr) => {
                let val = self.evaluate_expression(expr)?;
                self.evaluate_unary_op(op, val)
            }
            Expr::Call(func, args) => {
                // Simplified call handling for built-ins
                self.evaluate_call(func, args)
            }
            Expr::If(if_expr) => {
                let cond_val = self.evaluate_expression(&if_expr.condition)?;
                match cond_val {
                    Value::Bool(true) => self.evaluate_block(&if_expr.then_branch),
                    Value::Bool(false) => {
                        if let Some(else_branch) = &if_expr.else_branch {
                            self.evaluate_expression(else_branch)
                        } else {
                            Ok(Value::Unit)
                        }
                    }
                    _ => Err("Condition must be a boolean".to_string()),
                }
            }
            Expr::Block(block) => self.evaluate_block(block),
            Expr::Match(scrutinee, arms) => self.evaluate_match(scrutinee, arms),
            Expr::Lambda(params, body) => {
                // Capture current environment (simple clone for MVP)
                Ok(Value::Closure(params.clone(), body.clone(), self.environment.clone()))
            }
            Expr::Array(elements) => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.evaluate_expression(e)?);
                }
                Ok(Value::List(vals))
            }
            Expr::StructInit(name, fields) => self.evaluate_struct_init(name, fields),
            Expr::Index(array_expr, index_expr) => {
                let array_val = self.evaluate_expression(array_expr)?;
                let index_val = self.evaluate_expression(index_expr)?;

                if let Value::List(elements) = array_val {
                     if let Value::Number(idx) = index_val {
                         let i = idx as usize;
                         if i < elements.len() {
                             return Ok(elements[i].clone());
                         } else {
                             return Err(format!("Index out of bounds: {} (len {})", i, elements.len()));
                         }
                     }
                     return Err("Array index must be a number".to_string());
                }
                Err("Indexing requires a List".to_string())
            }
        }
    }

    fn evaluate_match(&mut self, scrutinee: &Expr, arms: &[MatchArm]) -> Result<Value, String> {
        let value = self.evaluate_expression(scrutinee)?;

        for arm in arms {
            if self.check_pattern(&arm.pattern, &value)? {
                // If pattern matches, execute body
                // TODO: Handle variable binding in pattern (e.g. identifier binding)
                // For now, simple matching
                if let Pattern::Identifier(name) = &arm.pattern {
                     self.environment.insert(name.clone(), value.clone());
                }
                
                return self.evaluate_expression(&arm.body);
            }
        }

        Err("Non-exhaustive match or no match found".to_string())
    }

    fn evaluate_struct_init(&mut self, _: &str, fields: &[(String, Expr)]) -> Result<Value, String> {
        let mut map = BTreeMap::new();
        for (name, expr) in fields {
            let val = self.evaluate_expression(expr)?;
            map.insert(name.clone(), val);
        }
        Ok(Value::Object(map))
    }

    fn assign_to_expr(&mut self, target: &Expr, value: Value) -> Result<(), String> {
        match target {
            Expr::Identifier(name) => {
                if self.environment.contains_key(name) {
                    self.environment.insert(name.clone(), value);
                    Ok(())
                } else if self.globals.contains_key(name) {
                    self.globals.insert(name.clone(), value);
                    Ok(())
                } else {
                    Err(format!("Undefined variable: {}", name))
                }
            }
            Expr::BinaryOp(left, BinaryOp::Dot, right) => {
                let obj = self.evaluate_expression(left)?;
                if let Expr::Identifier(prop) = &**right {
                    if let Value::Object(mut map) = obj {
                        map.insert(prop.clone(), value);
                        self.assign_to_expr(left, Value::Object(map))?;
                        Ok(())
                    } else {
                        Err("Cannot assign to property of non-object".to_string())
                    }
                } else {
                    Err("Expected identifier after '.'".to_string())
                }
            }
            _ => Err("Invalid assignment target".to_string())
        }
    }

    fn check_pattern(&self, pattern: &Pattern, value: &Value) -> Result<bool, String> {
        match (pattern, value) {
            (Pattern::Wildcard, _) => Ok(true),
            (Pattern::Literal(lit), val) => {
                let lit_val = self.evaluate_literal(lit)?;
                Ok(lit_val == *val)
            }
            (Pattern::Identifier(_), _) => Ok(true), // Always matches and binds
            _ => Ok(false), // Tuple not supported yet
        }
    }

    fn evaluate_block(&mut self, block: &crate::ast::BlockExpr) -> Result<Value, String> {
        for stmt in &block.statements {
            if self.return_signal.is_some() {
                return Ok(Value::Unit);
            }
            match stmt {
                Stmt::Let(let_stmt) => {
                    let val = self.evaluate_expression(&let_stmt.value)?;
                    self.environment.insert(let_stmt.name.clone(), val);
                }
                Stmt::Expr(e) => {
                    self.evaluate_expression(e)?;
                }
                Stmt::Return(e) => {
                    let val = self.evaluate_expression(e)?;
                    self.return_signal = Some(val);
                    return Ok(Value::Unit);
                }
                Stmt::While(while_stmt) => {
                    while self.return_signal.is_none() {
                        let cond = self.evaluate_expression(&while_stmt.condition)?;
                        if let Value::Bool(true) = cond {
                            self.evaluate_block(&while_stmt.body)?;
                        } else {
                            break;
                        }
                    }
                }
                Stmt::For(for_stmt) => {
                    let collection = self.evaluate_expression(&for_stmt.collection)?;
                    if let Value::List(items) = collection {
                        for item in items {
                            if self.return_signal.is_some() { break; }
                            self.environment.insert(for_stmt.iterator.clone(), item);
                            self.evaluate_block(&for_stmt.body)?;
                        }
                    } else {
                        return Err(format!("Expected list for iteration, found {:?}", collection));
                    }
                }
            }
        }
        if self.return_signal.is_some() {
            return Ok(Value::Unit);
        }
        if let Some(final_expr) = &block.final_expr {
            self.evaluate_expression(final_expr)
        } else {
            Ok(Value::Unit)
        }
    }

    fn evaluate_literal(&self, lit: &Literal) -> Result<Value, String> {
        match lit {
            Literal::Int(n) => Ok(Value::Number(*n as f64)),
            Literal::Float(n) => Ok(Value::Number(*n)),
            Literal::Str(s) => Ok(Value::String(s.clone())),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
        }
    }

    fn evaluate_binary_op(&self, left: Value, op: &BinaryOp, right: Value) -> Result<Value, String> {
        match (&left, &right) {
            (Value::Number(l), Value::Number(r)) => match op {
                BinaryOp::Add => Ok(Value::Number(l + r)),
                BinaryOp::Sub => Ok(Value::Number(l - r)),
                BinaryOp::Mul => Ok(Value::Number(l * r)),
                BinaryOp::Div => {
                    if *r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                BinaryOp::Rem => {
                     if *r == 0.0 {
                        Err("Modulo by zero".to_string())
                    } else {
                        Ok(Value::Number(l % r))
                    }
                }
                BinaryOp::Eq => Ok(Value::Bool(l == r)),
                BinaryOp::Neq => Ok(Value::Bool(l != r)),
                BinaryOp::Lt => Ok(Value::Bool(l < r)),
                BinaryOp::Lte => Ok(Value::Bool(l <= r)),
                BinaryOp::Gt => Ok(Value::Bool(l > r)),
                BinaryOp::Gte => Ok(Value::Bool(l >= r)),
                _ => Err(format!("Unsupported op {:?} for numbers", op)),
            },
            (Value::Bool(l), Value::Bool(r)) => match op {
                BinaryOp::Eq => Ok(Value::Bool(l == r)),
                BinaryOp::Neq => Ok(Value::Bool(l != r)),
                BinaryOp::And => Ok(Value::Bool(*l && *r)),
                BinaryOp::Or => Ok(Value::Bool(*l || *r)),
                _ => Err(format!("Unsupported op {:?} for bools", op)),
            },
             (Value::String(l), Value::String(r)) => match op {
                BinaryOp::Eq => Ok(Value::Bool(l == r)),
                BinaryOp::Neq => Ok(Value::Bool(l != r)),
                BinaryOp::Add => Ok(Value::String(l.clone() + r)),
                _ => Err(format!("Unsupported op {:?} for strings", op)),
            },
            (Value::Object(map), Value::String(prop)) if *op == BinaryOp::Dot => {
                map.get(prop).cloned().ok_or_else(|| format!("Property '{}' not found on object", prop))
            }
            (Value::Object(map), Value::Identifier(prop)) if *op == BinaryOp::Dot => {
                // This case handles when the right side is an identifier (internal representation)
                map.get(prop).cloned().ok_or_else(|| format!("Property '{}' not found on object", prop))
            }
            (left, right) if *op == BinaryOp::Add => {
                // Implicit String Concatenation Runtime Support
                if let Value::String(s) = left {
                    let r_str = match right {
                        Value::String(s2) => s2.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Port(p) => p.clone(),
                        Value::Stream(s) => s.clone(),
                        _ => format!("{:?}", right),
                    };
                    return Ok(Value::String(s.clone() + &r_str));
                }
                if let Value::String(s) = right {
                     let l_str = match left {
                        Value::String(s2) => s2.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => format!("{:?}", left),
                    };
                    return Ok(Value::String(l_str + s));
                }
                Err(format!("Type mismatch or unsupported op {:?} for {:?} and {:?}", op, left, right))
            }
            _ => Err(format!("Type mismatch or unsupported op {:?} for {:?} and {:?}", op, left, right)),
        }
    }

    fn evaluate_unary_op(&self, op: &UnaryOp, val: Value) -> Result<Value, String> {
        match (op, &val) {
            (UnaryOp::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            _ => Err(format!("Unsupported unary op {:?} for {:?}", op, val)),
        }
    }

    fn evaluate_call(&mut self, func: &Expr, args: &[Expr]) -> Result<Value, String> {
        if let Expr::Identifier(name) = func {
            match name.as_str() {
                "print" => {
                    for arg in args {
                        let val = self.evaluate_expression(arg)?;
                        match val {
                            Value::Number(n) => print!("{}", n),
                            Value::String(s) => print!("{}", s),
                            Value::Bool(b) => print!("{}", b),
                            Value::Unit => print!("()"),
                            _ => print!("{:?}", val),
                        }
                    }
                    println!();
                    return Ok(Value::Unit);
                }
                "assert" => {
                    if args.len() != 1 {
                        return Err("assert expects exactly 1 argument".to_string());
                    }
                    let val = self.evaluate_expression(&args[0])?;
                    match val {
                        Value::Bool(true) => return Ok(Value::Unit),
                        Value::Bool(false) => return Err("Assertion failed".to_string()),
                        _ => return Err("assert expects a boolean".to_string()),
                    }
                }
                "assert_eq" => {
                    if args.len() != 2 {
                        return Err("assert_eq expects exactly 2 arguments".to_string());
                    }
                    let left = self.evaluate_expression(&args[0])?;
                    let right = self.evaluate_expression(&args[1])?;
                    if left == right {
                        return Ok(Value::Unit);
                    } else {
                        return Err(format!("Assertion failed: {:?} != {:?}", left, right));
                    }
                }
                "map" => {
                    if args.len() != 2 { return Err("map expects 2 arguments: list, func".to_string()); }
                    let list_val = self.evaluate_expression(&args[0])?;
                    let func_val = self.evaluate_expression(&args[1])?;
                    
                    if let Value::List(elements) = list_val {
                       let mut new_elements = Vec::new();
                       for elem in elements {
                           // Apply closure to elem
                           let res = self.apply_closure_value(&func_val, vec![elem])?;
                           new_elements.push(res);
                       }
                       return Ok(Value::List(new_elements));
                    } else {
                        return Err("map expects a list as first argument".to_string());
                    }
                }
                "filter" => {
                    if args.len() != 2 { return Err("filter expects 2 arguments: list, func".to_string()); }
                    let list_val = self.evaluate_expression(&args[0])?;
                    let func_val = self.evaluate_expression(&args[1])?;

                    if let Value::List(elements) = list_val {
                       let mut new_elements = Vec::new();
                       for elem in elements {
                           let res = self.apply_closure_value(&func_val, vec![elem.clone()])?;
                           if let Value::Bool(true) = res {
                               new_elements.push(elem);
                           }
                       }
                       return Ok(Value::List(new_elements));
                    } else {
                        return Err("filter expects a list as first argument".to_string());
                    }
                }
                "reduce" => {
                     if args.len() != 3 { return Err("reduce expects 3 arguments: list, func, init".to_string()); }
                     let list_val = self.evaluate_expression(&args[0])?;
                     let func_val = self.evaluate_expression(&args[1])?;
                     let mut acc = self.evaluate_expression(&args[2])?;

                     if let Value::List(elements) = list_val {
                        for elem in elements {
                            acc = self.apply_closure_value(&func_val, vec![acc, elem])?;
                        }
                        return Ok(acc);
                     } else {
                         return Err("reduce expects a list as first argument".to_string());
                     }
                }
                "io_open" => {
                    let uri = self.evaluate_expression(&args[0])?;
                    if let Value::String(s) = uri {
                        return Ok(Value::Port(s));
                    }
                    return Err("io_open expects a string URI".to_string());
                }
                "io_write" => {
                    let port = self.evaluate_expression(&args[0])?;
                    let data = self.evaluate_expression(&args[1])?;
                    if let Value::Port(p) = port {
                        println!("[IO] Writing to {}: {:?}", p, data);
                        return Ok(Value::Bool(true));
                    }
                    return Err("io_write expects a Port as first argument".to_string());
                }
                "io_read" => {
                    let source = self.evaluate_expression(&args[0])?;
                    match source {
                        Value::Port(p) => {
                            // Simulated data based on port name
                            let mock_data = if p.starts_with("serial") { "CMD_OK" } else { "ACK" };
                            return Ok(Value::String(mock_data.to_string()));
                        }
                        Value::Stream(_) => {
                            return Ok(Value::String("[Frame Data]".to_string()));
                        }
                        _ => return Err("io_read expects a Port or Stream".to_string()),
                    }
                }
                "io_poll" => {
                    return Ok(Value::Bool(true)); // Always has data in simulation
                }
                "hid_get_key" => {
                    return Ok(Value::Number(13.0)); // Mock Enter key
                }
                "cam_capture" => {
                    return Ok(Value::Stream("cam://0".to_string()));
                }
                // Math & AI Helpers
                "math_exp" => { // for Sigmoid
                    let val = self.evaluate_expression(&args[0])?;
                    if let Value::Number(n) = val { return Ok(Value::Number(n.exp())); }
                    return Err("math_exp expects a number".to_string());
                }
                "math_sqrt" => {
                    let val = self.evaluate_expression(&args[0])?;
                    if let Value::Number(n) = val { return Ok(Value::Number(n.sqrt())); }
                    return Err("math_sqrt expects a number".to_string());
                }
                "math_sin" => {
                    let val = self.evaluate_expression(&args[0])?;
                    if let Value::Number(n) = val { return Ok(Value::Number(n.sin())); }
                    return Err("math_sin expects a number".to_string());
                }
                "math_cos" => {
                    let val = self.evaluate_expression(&args[0])?;
                    if let Value::Number(n) = val { return Ok(Value::Number(n.cos())); }
                    return Err("math_cos expects a number".to_string());
                }
                "math_random" => {
                     // Simple pseudo-random for simulation
                     return Ok(Value::Number(0.5)); // Fixed for determinism in validation
                }
                // v1.6.0: Future Tech Helpers
                "crypto_hash" => {
                    let val = self.evaluate_expression(&args[0])?;
                    if let Value::String(s) = val {
                        // Mock Hash: Simple visual transformation for demo
                        // In real impl, use sha2 crate. Here just reverse + length + salt
                        let reversed: String = s.chars().rev().collect();
                        let hash = format!("HASH_{}_{}", s.len(), reversed); 
                        return Ok(Value::String(hash));
                    }
                    return Err("crypto_hash expects a string".to_string());
                }
                "time_now" => {
                    // Mock timestamp for deterministic testing
                    return Ok(Value::Number(1678886400.0)); 
                }
                "str_len" => {
                    let val = self.evaluate_expression(&args[0])?;
                    if let Value::String(s) = val { return Ok(Value::Number(s.len() as f64)); }
                    return Err("str_len expects a string".to_string());
                }
                "str_sub" => {
                    let str_val = self.evaluate_expression(&args[0])?;
                    let start_val = self.evaluate_expression(&args[1])?;
                    let len_val = self.evaluate_expression(&args[2])?;
                    
                    if let (Value::String(s), Value::Number(start), Value::Number(len)) = (str_val, start_val, len_val) {
                        let start_idx = start as usize;
                        let length = len as usize;
                        if start_idx + length <= s.len() {
                             let sub = s[start_idx..start_idx+length].to_string();
                             return Ok(Value::String(sub));
                        }
                        return Ok(Value::String("".to_string())); // Safe fail
                    }
                    return Err("str_sub expects (string, number, number)".to_string());
                }
                "str_replace" => {
                    let str_val = self.evaluate_expression(&args[0])?;
                    let pattern_val = self.evaluate_expression(&args[1])?;
                    let replace_val = self.evaluate_expression(&args[2])?;
                    
                    if let (Value::String(s), Value::String(p), Value::String(r)) = (str_val, pattern_val, replace_val) {
                        return Ok(Value::String(s.replace(&p, &r)));
                    }
                    return Err("str_replace expects (string, string, string)".to_string());
                }
                _ => {} // Fallthrough to variable lookup
            }
        }

        let func_val = self.evaluate_expression(func)?;

        let mut arg_vals = Vec::new();
        for arg in args {
            arg_vals.push(self.evaluate_expression(arg)?);
        }

        self.apply_closure_value(&func_val, arg_vals)
    }

    pub fn apply_closure_value(&mut self, func: &Value, args: Vec<Value>) -> Result<Value, String> {
        match func {
            Value::Closure(params, body, captured_env) => {
                 if args.len() != params.len() {
                    return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
                }
                
                // Swap environment
                // To avoid cloning captured_env repeatedly if possible? No, we need to execute in that env.
                // We typically need to clone the captured env if we want to support recursion or re-entry properly without consuming it? 
                // But Value::Closure owns captured_env.
                // MVP: clone it.
                
                let mut exec_env = captured_env.clone();
                
                 // Bind args
                for (param, val) in params.iter().zip(args) {
                    exec_env.insert(param.clone(), val);
                }
                
                 let previous_env = std::mem::replace(&mut self.environment, exec_env);
                 let result = self.evaluate_expression(&body);
                 self.environment = previous_env;
                 
                 result
            }
            Value::OracleFunction(func) => {
                let oracle_deco = func.decorators.iter().find(|d| d.name == "oracle").unwrap();
                let format = oracle_deco.args.get("format").map(|s| s.as_str()).unwrap_or("");
                let model_path = oracle_deco.args.get("model").map(|s| s.as_str()).ok_or("Oracle decorator missing 'model' arg")?;
                
                if format == "onnx" {
                    let start_time = std::time::Instant::now();
                    
                    let shapes_str = oracle_deco.args.get("shape").map(|s| s.as_str()).unwrap_or("");
                    let shape_parts: Vec<&str> = if shapes_str.is_empty() { Vec::new() } else { shapes_str.split('|').collect() };
                    
                    let mut inputs_data = Vec::new();
                    for (i, input_val) in args.into_iter().enumerate() {
                        let mut flat_data = Vec::new();
                        if let Value::List(list) = input_val {
                            for item in list {
                                if let Value::Number(n) = item {
                                    flat_data.push(n as f32);
                                } else {
                                    return Err("ONNX input must be a list of numbers".to_string());
                                }
                            }
                        } else {
                            return Err("ONNX input must be a list of numbers".to_string());
                        }
                        
                        let shape = if i < shape_parts.len() && !shape_parts[i].is_empty() {
                            shape_parts[i].split(',').map(|dim| dim.trim().parse::<usize>().unwrap_or(0)).collect::<Vec<usize>>()
                        } else {
                            vec![1, flat_data.len()]
                        };
                        inputs_data.push((shape, flat_data));
                    }
                    
                    match crate::onnx_oracle::run_inference(std::path::Path::new(model_path), inputs_data) {
                        Ok(out_data) => {
                            let elapsed = start_time.elapsed();
                            println!("[ORACLE TIMER] Inference ran in {:.2?}", elapsed);
                            
                            let mut rust_outputs = Vec::new();
                            for tensor_out in out_data {
                                let mut vals = Vec::new();
                                for f in tensor_out {
                                    vals.push(Value::Number(f as f64));
                                }
                                rust_outputs.push(Value::List(vals));
                            }
                            
                            Ok(Value::List(rust_outputs))
                        }
                        Err(e) => Err(format!("ONNX Inference Failed: {}", e))
                    }
                } else {
                    Err(format!("Unsupported oracle format: {}", format))
                }
            }
            Value::MeshFunction(func) => {
                if self.is_worker_mode {
                    // Eksekusi fungsi sebagai fungsi lokal
                    let params: Vec<String> = func.params.iter().map(|p| p.name.clone()).collect();
                    if args.len() != params.len() {
                        return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
                    }
                    
                    let exec_env = self.environment.clone();
                    let previous_env = std::mem::replace(&mut self.environment, exec_env);
                    
                    for (param, val) in params.iter().zip(args) {
                        self.environment.insert(param.clone(), val);
                    }
                    
                    let iter_body = func.body.clone();
                    let result = if let Some(b) = iter_body {
                        let body_expr = Expr::Block(b);
                        self.evaluate_expression(&body_expr)
                    } else {
                        Ok(Value::Unit)
                    };
                    let final_res = if let Some(val) = self.return_signal.take() {
                        Ok(val)
                    } else {
                        result
                    };
                    self.environment = previous_env;
                    final_res
                } else {
                    // Forward sebagai klien RPC
                    let mesh_deco = func.decorators.iter().find(|d| d.name == "mesh").unwrap();
                    let target = mesh_deco.args.get("target").map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");
                    
                    let token = self.environment.get("X_CAPABILITY_TOKEN")
                        .or_else(|| self.globals.get("X_CAPABILITY_TOKEN"))
                        .and_then(|v| if let Value::String(s) = v { Some(s.clone()) } else { None });
                    
                    println!("[MESH] Forwarding execution of '{}' to {}", func.name, target);
                    crate::mesh::transport::send_mesh_request(target, &func.name, &args, token)
                }
            }
             _ => Err(format!("Not a closure/callable: {:?}", func))
        }
    }
}
