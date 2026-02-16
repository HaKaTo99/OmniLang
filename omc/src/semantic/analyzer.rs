use crate::parser::ast::{Program, Statement, Expression};
use crate::semantic::symbol_table::{SymbolTable, SymbolType};

pub struct SemanticAnalyzer {
    pub errors: Vec<String>,
    // We strictly own the current scope.
    // When entering a new scope, we take `current_scope`, wrap it in a new table, and set that as `current_scope`.
    // When exiting, we take `current_scope`, extract the outer one, and set that as `current_scope`.
    // This avoids self-referential lifetimes in the struct.
    scope: SymbolTable,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            errors: Vec::new(),
            scope: SymbolTable::new(),
        };
        
        // Register Built-ins
        analyzer.scope.define("print".to_string(), SymbolType::Function(vec![SymbolType::Int], Box::new(SymbolType::Int)));
        
        analyzer
    }

    pub fn analyze(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.analyze_statement(stmt);
        }
    }

    fn analyze_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { name, value, .. } => {
                let inferred_type = self.analyze_expression(value);
                // For now, Let always succeeds and binds the inferred type
                self.scope.define(name.clone(), inferred_type);
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.analyze_expression(e);
                }
            }
            Statement::Expression(expr) => {
                self.analyze_expression(expr);
            }
            Statement::Block(stmts) => {
                self.enter_scope();
                for s in stmts {
                    self.analyze_statement(s);
                }
                self.exit_scope();
            }
            Statement::Function { name, params, body, .. } => {
                // Register function
                self.scope.define(name.clone(), SymbolType::Function(vec![], Box::new(SymbolType::Int)));

                self.enter_scope();
                for (p_name, _p_type) in params {
                    self.scope.define(p_name.clone(), SymbolType::Int);
                }
                
                if let Statement::Block(stmts) = *body.clone() {
                     for s in stmts {
                        self.analyze_statement(&s);
                    }
                }
                
                self.exit_scope();
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expression) -> SymbolType {
        match expr {
            Expression::Integer(_) => SymbolType::Int,
            Expression::String(_) => SymbolType::String,
            Expression::Boolean(_) => SymbolType::Bool,
            Expression::Identifier(name) => {
                match self.scope.resolve(name) {
                    Some(s) => s.kind,
                    None => {
                        self.errors.push(format!("Undefined variable: '{}'", name));
                        SymbolType::Unknown
                    }
                }
            }
            Expression::Infix { left, operator, right } => {
                let left_type = self.analyze_expression(left);
                let right_type = self.analyze_expression(right);

                // Simple type checking: left must match right for math/comparison
                if left_type != right_type && left_type != SymbolType::Unknown && right_type != SymbolType::Unknown {
                    self.errors.push(format!(
                        "Type mismatch: cannot apply {:?} to {:?} and {:?}",
                        operator, left_type, right_type
                    ));
                    return SymbolType::Unknown;
                }

                match operator {
                    crate::lexer::token::Token::Plus | crate::lexer::token::Token::Minus | 
                    crate::lexer::token::Token::Star | crate::lexer::token::Token::Slash => left_type,
                    crate::lexer::token::Token::EqEq | crate::lexer::token::Token::NotEq |
                    crate::lexer::token::Token::Lt | crate::lexer::token::Token::Gt |
                    crate::lexer::token::Token::LtEq | crate::lexer::token::Token::GtEq => SymbolType::Bool,
                    _ => SymbolType::Unknown,
                }
            }
            Expression::Prefix { right, .. } => {
                self.analyze_expression(right)
            }
            Expression::If { condition, consequence, alternative } => {
                let cond_type = self.analyze_expression(condition);
                if cond_type != SymbolType::Bool && cond_type != SymbolType::Unknown {
                    self.errors.push(format!("If condition must be Bool, got {:?}", cond_type));
                }

                self.analyze_statement(consequence);
                if let Some(alt) = alternative {
                    self.analyze_statement(alt);
                }
                SymbolType::Unknown // If expressions in OmniLang might return a value later, for now void/unknown
            }
            Expression::Call { function, arguments } => {
                let func_type = self.analyze_expression(function);
                for arg in arguments {
                    self.analyze_expression(arg);
                }

                match func_type {
                    SymbolType::Function(_, ret) => *ret,
                    _ => SymbolType::Unknown,
                }
            }
            Expression::Match(match_expr) => {
                self.analyze_expression(&match_expr.scrutinee);
                for arm in &match_expr.arms {
                    // Analyze pattern?
                    // Analyze body
                    // self.analyze_statement? No, body is Box<Statement>
                    self.analyze_statement(&arm.body);
                }
                SymbolType::Unknown // TODO: Infer common type of arms
            }
        }
    }

    fn enter_scope(&mut self) {
        // Take the current scope out
        let current = std::mem::replace(&mut self.scope, SymbolTable::new());
        // Create new enclosed declaration
        self.scope = SymbolTable::new_enclosed(current);
    }

    fn exit_scope(&mut self) {
        if let Some(outer) = self.scope.clone().into_outer() {
            self.scope = outer;
        }
    }
}
