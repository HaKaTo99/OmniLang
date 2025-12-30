use std::collections::HashMap;
use crate::ast::{Program, Module, Statement, Expression, Literal, BinaryOp, UnaryOp, Function, Parameter};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Unit,
}

pub struct ProgramEvaluator {
    environment: HashMap<String, Value>,
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

    fn evaluate_module(&mut self, module: &Module) -> Result<(), String> {
        for statement in &module.statements {
            self.evaluate_statement(statement)?;
        }
        Ok(())
    }

    fn evaluate_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Function(func) => {
                // Store function in environment
                self.environment.insert(func.name.clone(), Value::Unit); // Placeholder
                Ok(())
            }
            Statement::Constant(name, expr) => {
                let value = self.evaluate_expression(expr)?;
                self.environment.insert(name.clone(), value);
                Ok(())
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Literal(lit) => self.evaluate_literal(lit),
            Expression::Identifier(name) => {
                self.environment.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(left_val, op, right_val)
            }
            Expression::UnaryOp(op, expr) => {
                let val = self.evaluate_expression(expr)?;
                self.evaluate_unary_op(op, val)
            }
            Expression::FunctionCall(name, args) => {
                self.evaluate_function_call(name, args)
            }
            Expression::If(cond, then_expr, else_expr) => {
                let cond_val = self.evaluate_expression(cond)?;
                match cond_val {
                    Value::Bool(true) => self.evaluate_expression(then_expr),
                    Value::Bool(false) => {
                        if let Some(else_expr) = else_expr {
                            self.evaluate_expression(else_expr)
                        } else {
                            Ok(Value::Unit)
                        }
                    }
                    _ => Err("Condition must be a boolean".to_string()),
                }
            }
            Expression::Block(statements, expr) => {
                for stmt in statements {
                    self.evaluate_statement(stmt)?;
                }
                if let Some(expr) = expr {
                    self.evaluate_expression(expr)
                } else {
                    Ok(Value::Unit)
                }
            }
        }
    }

    fn evaluate_literal(&self, lit: &Literal) -> Result<Value, String> {
        match lit {
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
        }
    }

    fn evaluate_binary_op(&self, left: Value, op: &BinaryOp, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => match op {
                BinaryOp::Add => Ok(Value::Number(l + r)),
                BinaryOp::Subtract => Ok(Value::Number(l - r)),
                BinaryOp::Multiply => Ok(Value::Number(l * r)),
                BinaryOp::Divide => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(Value::Number(l / r))
                    }
                }
                BinaryOp::Equal => Ok(Value::Bool(l == r)),
                BinaryOp::NotEqual => Ok(Value::Bool(l != r)),
                BinaryOp::Less => Ok(Value::Bool(l < r)),
                BinaryOp::LessEqual => Ok(Value::Bool(l <= r)),
                BinaryOp::Greater => Ok(Value::Bool(l > r)),
                BinaryOp::GreaterEqual => Ok(Value::Bool(l >= r)),
            },
            (Value::String(l), Value::String(r)) => match op {
                BinaryOp::Add => Ok(Value::String(l + &r)),
                BinaryOp::Equal => Ok(Value::Bool(l == r)),
                BinaryOp::NotEqual => Ok(Value::Bool(l != r)),
                _ => Err(format!("Unsupported operation {:?} for strings", op)),
            },
            (Value::Bool(l), Value::Bool(r)) => match op {
                BinaryOp::Equal => Ok(Value::Bool(l == r)),
                BinaryOp::NotEqual => Ok(Value::Bool(l != r)),
                BinaryOp::And => Ok(Value::Bool(l && r)),
                BinaryOp::Or => Ok(Value::Bool(l || r)),
                _ => Err(format!("Unsupported operation {:?} for booleans", op)),
            },
            _ => Err(format!("Type mismatch for operation {:?}", op)),
        }
    }

    fn evaluate_unary_op(&self, op: &UnaryOp, val: Value) -> Result<Value, String> {
        match (op, val) {
            (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            _ => Err(format!("Unsupported unary operation {:?} for given type", op)),
        }
    }

    fn evaluate_function_call(&mut self, name: &str, args: &[Expression]) -> Result<Value, String> {
        // For now, just handle built-in functions
        match name {
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
    }
}
