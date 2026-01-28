use std::collections::BTreeMap;
use crate::ast::{Program, Module, Stmt, Expr, Literal, BinaryOp, UnaryOp, MatchArm, Pattern};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Unit,
    Closure(Vec<String>, Box<Expr>, BTreeMap<String, Value>),
    List(Vec<Value>),
    // Future: Function(FunctionDecl), Struct(StructInstance)
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Unit, Value::Unit) => true,
            (Value::List(a), Value::List(b)) => a == b,
            _ => false,
        }
    }
}

pub struct ProgramEvaluator {
    pub environment: BTreeMap<String, Value>,
}

impl Default for ProgramEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramEvaluator {
    pub fn new() -> Self {
        ProgramEvaluator {
            environment: BTreeMap::new(),
        }
    }

    pub fn evaluate_program(&mut self, program: &Program) -> Result<Value, String> {
        for module in &program.modules {
            self.evaluate_module(module)?;
        }
        Ok(Value::Unit)
    }

    fn evaluate_module(&mut self, _: &Module) -> Result<(), String> {
        // For MVP, just evaluate items that are statements or expressions if any?
        // Actually AST Module has `items: Vec<Item>`, not statements.
        // We might need an entry point like "main".
        // For now, let's assume we are evaluating a BlockExpr or similar in tests.
        // But for match expression testing, we just need evaluate_expression.
        Ok(())
    }

    // Helper for testing expression evaluation directly
    pub fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => self.evaluate_literal(lit),
            Expr::Identifier(name) => {
                self.environment.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
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
            match stmt {
                Stmt::Let(let_stmt) => {
                    let val = self.evaluate_expression(&let_stmt.value)?;
                    self.environment.insert(let_stmt.name.clone(), val);
                }
                Stmt::Expr(e) => {
                    self.evaluate_expression(e)?;
                }
                Stmt::Return(e) => {
                    return self.evaluate_expression(e);
                }
            }
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
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match op {
                BinaryOp::Add => Ok(Value::Number(l + r)),
                BinaryOp::Sub => Ok(Value::Number(l - r)),
                BinaryOp::Mul => Ok(Value::Number(l * r)),
                BinaryOp::Div => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                BinaryOp::Rem => {
                     if r == 0.0 {
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
                _ => Err(format!("Unsupported op {:?} for bools", op)),
            },
             (Value::String(l), Value::String(r)) => match op {
                BinaryOp::Eq => Ok(Value::Bool(l == r)),
                BinaryOp::Neq => Ok(Value::Bool(l != r)),
                BinaryOp::Add => Ok(Value::String(l + &r)),
                _ => Err(format!("Unsupported op {:?} for strings", op)),
            },
            _ => Err("Type mismatch".to_string()),
        }
    }

    fn evaluate_unary_op(&self, op: &UnaryOp, val: Value) -> Result<Value, String> {
        match (op, val) {
            (UnaryOp::Neg, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            _ => Err(format!("Unsupported unary op {:?}", op)),
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
                _ => {} // Fallthrough to variable lookup
            }
        }

        let func_val = self.evaluate_expression(func)?;

        match func_val {
            Value::Closure(params, body, captured_env) => {
                if args.len() != params.len() {
                    return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
                }
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate_expression(arg)?);
                }

                // Swap environment
                let previous_env = std::mem::replace(&mut self.environment, captured_env);

                // Bind args
                for (param, val) in params.iter().zip(arg_vals) {
                    self.environment.insert(param.clone(), val);
                }

                // Execute body
                let result = self.evaluate_expression(&body);

                // Restore environment
                self.environment = previous_env;

                result
            }
            _ => Err(format!("Expression is not callable: {:?}", func_val))
        }
    }

    fn apply_closure_value(&mut self, func: &Value, args: Vec<Value>) -> Result<Value, String> {
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
             _ => Err(format!("Not a closure: {:?}", func))
        }
    }
}
