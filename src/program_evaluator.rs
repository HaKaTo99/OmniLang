use std::collections::HashMap;
use crate::ast::{Program, Module, Stmt, Expr, Literal, BinaryOp, UnaryOp, MatchArm, Pattern};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Unit,
    // Future: Function(FunctionDecl), Struct(StructInstance)
}

pub struct ProgramEvaluator {
    pub environment: HashMap<String, Value>,
}

impl Default for ProgramEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramEvaluator {
    pub fn new() -> Self {
        ProgramEvaluator {
            environment: HashMap::new(),
        }
    }

    pub fn evaluate_program(&mut self, program: &Program) -> Result<Value, String> {
        for module in &program.modules {
            self.evaluate_module(module)?;
        }
        Ok(Value::Unit)
    }

    fn evaluate_module(&mut self, _module: &Module) -> Result<(), String> {
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
            _ => Err(format!("Unsupported expression type for evaluation: {:?}", expr)),
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
                        }
                    }
                    println!();
                    Ok(Value::Unit)
                }
                _ => Err(format!("Unknown function: {}", name)),
            }
        } else {
             Err("Indirect calls not supported yet".to_string())
        }
    }
}
