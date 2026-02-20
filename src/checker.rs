// src/checker.rs
use crate::ast::{Program, Module, Item, FunctionDecl, StructDecl, Stmt, LetStmt, Expr, BlockExpr, IfExpr, MatchArm, Pattern, BinaryOp, UnaryOp, Literal};
use crate::types::*;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Checker {
    env: TypeEnvironment,
    current_module_mode: Option<String>, // "@gc" or "@ownership"
    in_ownership_mode: bool,
    type_unifier: TypeUnifier,
    errors: Vec<String>,
    borrow_tracker: BorrowTracker,
    structs: HashMap<String, StructDecl>,
}

#[derive(Debug, Clone)]
struct BorrowTracker {
    // Tracks variables and their current borrow state
    variables: HashMap<String, BorrowState>,
}

#[derive(Debug, Clone, PartialEq)]
enum BorrowState {
    Owned,
    BorrowedImmutable,
    BorrowedMutable,
    Moved,
}

impl Checker {
    pub fn new() -> Self {
        Checker {
            env: TypeEnvironment::new(),
            current_module_mode: None,
            in_ownership_mode: false,
            type_unifier: TypeUnifier::new(),
            errors: Vec::new(),
            borrow_tracker: BorrowTracker::new(),
            structs: HashMap::new(),
        }
    }
    
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<String>> {
        for module in &program.modules {
            self.current_module_mode = module.mode.clone();
            self.in_ownership_mode = module.mode.as_ref().map_or(false, |m| m == "@ownership");
            
            if self.in_ownership_mode {
                println!("🔒 Checking module '{}' in OWNERSHIP mode", module.name);
            }
            
            self.check_module(module)?;
        }
        
        // Run type unification
        if let Err(e) = self.type_unifier.unify() {
            self.errors.push(e);
        }
        
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }
        
        Ok(())
    }
    
    fn check_module(&mut self, module: &Module) -> Result<(), Vec<String>> {
        // First pass: collect all declarations
        for item in &module.items {
            match item {
                Item::Function(func) => {
                    self.register_function(func)?;
                }
                Item::Struct(struct_decl) => {
                    self.register_struct(struct_decl)?;
                }
                Item::Trait(_trait_decl) => {
                    // self.register_trait(trait_decl)?;
                }
                Item::Impl(_impl_decl) => {
                    // self.register_impl(impl_decl)?;
                }
                Item::Const(const_decl) => {
                    self.register_const(const_decl)?;
                }
            }
        }
        
        // Second pass: check bodies
        for item in &module.items {
            if let Item::Function(func) = item {
                self.check_function_body(func)?;
            }
        }
        
        Ok(())
    }
    
    fn register_function(&mut self, func: &FunctionDecl) -> Result<(), Vec<String>> {
        let params_types = func.params.iter().map(|p| Type::from_ast_type(&p.param_type)).collect();
        let return_type = func.return_type.as_ref().map(|t| Type::from_ast_type(t)).unwrap_or(Type::Unit);
        
        let func_type = Type::Function {
            params: params_types,
            return_type: Box::new(return_type),
        };
        
        let symbol = Symbol {
            name: func.name.clone(),
            type_info: func_type,
            is_mutable: false,
            status: OwnershipStatus::Owned,
            defined_at: 0,
        };

        if let Err(e) = self.env.insert(symbol) {
            self.errors.push(e);
        }
        Ok(())
    }
    
    fn register_struct(&mut self, struct_decl: &StructDecl) -> Result<(), Vec<String>> {
        self.structs.insert(struct_decl.name.clone(), struct_decl.clone());
        Ok(())
    }

    fn register_const(&mut self, const_decl: &crate::ast::ConstDecl) -> Result<(), Vec<String>> {
        let type_info = Type::from_ast_type(&const_decl.const_type);
        let symbol = Symbol {
            name: const_decl.name.clone(),
            type_info: type_info.clone(),
            is_mutable: false,
            status: OwnershipStatus::Owned,
            defined_at: 0,
        };
        if let Err(e) = self.env.insert(symbol) {
            self.errors.push(e);
        }
        Ok(())
    }
    
    fn check_function_body(&mut self, func: &FunctionDecl) -> Result<(), Vec<String>> {
        let mut function_env = self.env.enter_scope();
        let mut function_borrow_tracker = self.borrow_tracker.clone(); // Clone for function scope
        
        // Register parameters
        for param in &func.params {
            let symbol = Symbol {
                name: param.name.clone(),
                type_info: Type::from_ast_type(&param.param_type),
                is_mutable: true,
                status: OwnershipStatus::Owned,
                defined_at: 0, 
            };
            
            if let Err(e) = function_env.insert(symbol) {
                self.errors.push(e);
            }
            function_borrow_tracker.declare_variable(&param.name, BorrowState::Owned);
        }
        
        // Check function body with the new environment
        let body_type = self.check_block(&func.body, &mut function_env, &mut function_borrow_tracker)?;
        
        // Verify return type
        let expected_return_type = func.return_type.as_ref().map(|t| Type::from_ast_type(t)).unwrap_or(Type::Unit);
        if body_type != expected_return_type && body_type != Type::Divergent {
            self.errors.push(format!(
                "Mismatched return type for function '{}': expected {:?}, found {:?}",
                func.name, expected_return_type, body_type
            ));
        }
        
        Ok(())
    }
    
    fn check_block(&mut self, block: &BlockExpr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        let mut block_env = env.enter_scope();
        let mut last_return_type = None;
        
        for stmt in &block.statements {
            if let Some(t) = self.check_statement(stmt, &mut block_env, borrow_tracker)? {
                last_return_type = Some(t);
            }
        }
        
        if let Some(expr) = &block.final_expr {
            Ok(self.check_expression(expr, &mut block_env, borrow_tracker)?)
        } else if let Some(rt) = last_return_type {
            // If the block ends with a Return (or contains one and no final expr),
            // it diverges.
            Ok(rt)
        } else {
            Ok(Type::Unit)
        }
    }
    
    fn check_statement(&mut self, stmt: &Stmt, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Option<Type>, Vec<String>> {
        match stmt {
            Stmt::Let(let_stmt) => {
                self.check_let_statement(let_stmt, env, borrow_tracker)?;
                Ok(None)
            }
            Stmt::Expr(expr) => {
                self.check_expression(expr, env, borrow_tracker)?;
                Ok(None)
            },
            Stmt::Return(expr) => {
                let _t = self.check_expression(expr, env, borrow_tracker)?;
                // We should ideally check t against function return type here, 
                // but let's just return Divergent for now to help with If-checking.
                Ok(Some(Type::Divergent))
            }
            Stmt::While(while_stmt) => {
                let cond_type = self.check_expression(&while_stmt.condition, env, borrow_tracker)?;
                if cond_type != Type::Bool {
                    self.errors.push(format!("While condition must be boolean, found {:?}", cond_type));
                }
                self.check_block(&while_stmt.body, env, borrow_tracker)?;
                Ok(None)
            }
            Stmt::For(for_stmt) => {
                let collection_type = self.check_expression(&for_stmt.collection, env, borrow_tracker)?;
                if let Type::List(inner_type) = collection_type {
                    let mut for_env = env.enter_scope();
                    for_env.insert(Symbol {
                        name: for_stmt.iterator.clone(),
                        type_info: (*inner_type).clone(),
                        is_mutable: false,
                        status: OwnershipStatus::Owned,
                        defined_at: 0,
                    }).map_err(|e| self.errors.push(e)).ok();
                    
                    for stmt in &for_stmt.body.statements {
                        self.check_statement(stmt, &mut for_env, borrow_tracker)?;
                    }
                    if let Some(final_expr) = &for_stmt.body.final_expr {
                        self.check_expression(final_expr, &mut for_env, borrow_tracker)?;
                    }
                } else if collection_type != Type::Unknown {
                    self.errors.push(format!("For collection must be a list, found {:?}", collection_type));
                }
                Ok(None)
            }
        }
    }
    
    fn check_let_statement(&mut self, let_stmt: &LetStmt, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<(), Vec<String>> {
        let value_type = self.check_expression(&let_stmt.value, env, borrow_tracker)?;

        if let Some(annot_type_ast) = &let_stmt.type_annotation {
            let annot_type = Type::from_ast_type(annot_type_ast);
            if annot_type != value_type {
                self.errors.push(format!(
                    "Type mismatch for variable '{}': expected {:?}, found {:?}",
                    let_stmt.name, annot_type, value_type
                ));
            }
        }
        
        let symbol = Symbol {
            name: let_stmt.name.clone(),
            type_info: value_type.clone(),
            is_mutable: let_stmt.is_mut,
            status: OwnershipStatus::Owned,
            defined_at: 0,
        };
        
        if let Err(e) = env.insert(symbol) {
            self.errors.push(e);
        }
        
        borrow_tracker.declare_variable(&let_stmt.name, BorrowState::Owned);
        
        if self.in_ownership_mode {
            if !value_type.is_copy_type() {
                if let Expr::Identifier(name) = &let_stmt.value {
                    borrow_tracker.move_var(name).map_err(|e| self.errors.push(e)).ok();
                }
            }
        }
        
        Ok(())
    }

    fn check_expression(&mut self, expr: &Expr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        match expr {
            Expr::Literal(lit) => self.check_literal(lit),
            Expr::Identifier(name) => self.check_identifier(name, env, borrow_tracker),
            Expr::BinaryOp(left, op, right) => self.check_binary_op(left, op, right, env, borrow_tracker),
            Expr::UnaryOp(op, operand) => self.check_unary_op(op, operand, env, borrow_tracker),
            Expr::Call(callee, args) => self.check_call_expr(callee, args, env, borrow_tracker),
            Expr::Index(array_expr, index_expr) => {
                let array_type = self.check_expression(array_expr, env, borrow_tracker)?;
                let index_type = self.check_expression(index_expr, env, borrow_tracker)?;

                // Simple check for now: array must be list, index must be numeric (I32 or F64)
                if matches!(index_type, Type::I32 | Type::F64 | Type::Int) {
                    if let Type::List(inner) = array_type {
                        return Ok(*inner);
                    } else if let Type::InferenceVar(_) = array_type {
                         // If it's an inference var, assume it's a list of something
                         let new_var = self.type_unifier.fresh_var();
                         self.type_unifier.add_constraint(array_type, Type::List(Box::new(new_var.clone())));
                         return Ok(new_var);
                    } else {
                        self.errors.push(format!("Indexing requires a List, found {:?}", array_type));
                        return Ok(Type::Unknown);
                    }
                } else {
                     self.errors.push(format!("Index must be numeric, found {:?}", index_type));
                     return Ok(Type::Unknown);
                }
            }
            Expr::Block(block_expr) => self.check_block(block_expr, env, borrow_tracker),
            Expr::If(if_expr) => self.check_if_expr(if_expr, env, borrow_tracker),
            Expr::Match(value, arms) => self.check_match_expr(value, arms, env, borrow_tracker),
            Expr::Lambda(params, body) => self.check_lambda_expr(params, body, env, borrow_tracker, None),
            Expr::StructInit(name, fields) => {
                if let Some(s) = self.structs.get(name).cloned() {
                    for (f_name, f_expr) in fields {
                        let val_type = self.check_expression(f_expr, env, borrow_tracker)?;
                        if let Some(target_field) = s.fields.iter().find(|f| f.name == *f_name) {
                            let target_type = Type::from_ast_type(&target_field.field_type);
                            if val_type != target_type && val_type != Type::Unknown {
                                self.errors.push(format!("Field '{}' of struct '{}' expects {:?}, found {:?}", f_name, name, target_type, val_type));
                            }
                        } else {
                            self.errors.push(format!("Struct '{}' has no field named '{}'", name, f_name));
                        }
                    }
                    Ok(Type::Named(name.clone()))
                } else {
                    self.errors.push(format!("Undefined struct: '{}'", name));
                    Ok(Type::Unknown)
                }
            }
            Expr::Array(elements) => {
                if elements.is_empty() {
                    // Empty list, type unknown but technically List<Unknown> or use InferenceVar?
                    // For now: List(Unknown)
                    return Ok(Type::List(Box::new(Type::Unknown)));
                }
                
                let first_type = self.check_expression(&elements[0], env, borrow_tracker)?;
                for elem in &elements[1..] {
                    let elem_type = self.check_expression(elem, env, borrow_tracker)?;
                    if elem_type != first_type {
                        self.errors.push(format!("Array elements must have the same type. Expected {:?}, found {:?}", first_type, elem_type));
                    }
                }
                Ok(Type::List(Box::new(first_type)))
            }
        }
    }

    fn check_if_expr(&mut self, if_expr: &IfExpr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        let cond_type = self.check_expression(&if_expr.condition, env, borrow_tracker)?;
        if cond_type != Type::Bool {
            self.errors.push(format!(
                "If condition must be a boolean, but found {:?}",
                cond_type
            ));
        }

        let then_type = self.check_block(&if_expr.then_branch, env, borrow_tracker)?;

        let else_type = if let Some(else_branch) = &if_expr.else_branch {
            self.check_expression(else_branch, env, borrow_tracker)?
        } else {
            Type::Unit
        };

        if then_type != else_type && then_type != Type::Divergent && else_type != Type::Divergent {
            self.errors.push(format!(
                "If branches have mismatched types: `then` has type {:?}, but `else` has type {:?}",
                then_type, else_type
            ));
            Ok(Type::Unknown) // Return Unknown on mismatch
        } else {
            if then_type == Type::Divergent { Ok(else_type) } else { Ok(then_type) }
        }
    }

    fn check_match_expr(&mut self, value: &Expr, arms: &[MatchArm], env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        let value_type = self.check_expression(value, env, borrow_tracker)?;

        let mut arm_types = Vec::new();
        for arm in arms {
            // Check pattern exhaustiveness and binding
            self.check_pattern(&arm.pattern, &value_type, env, borrow_tracker)?;

            // Check guard if present
            if let Some(guard) = &arm.guard {
                let guard_type = self.check_expression(guard, env, borrow_tracker)?;
                if guard_type != Type::Bool {
                    self.errors.push(format!("Match guard must be boolean, found {:?}", guard_type));
                }
            }

            // Check arm body
            let arm_type = self.check_expression(&arm.body, env, borrow_tracker)?;
            arm_types.push(arm_type);
        }

        // All arms must have the same type
        if let Some(first_type) = arm_types.first() {
            for arm_type in &arm_types[1..] {
                if arm_type != first_type {
                    self.errors.push(format!("Match arms have mismatched types: expected {:?}, found {:?}", first_type, arm_type));
                }
            }
            Ok(first_type.clone())
        } else {
            Ok(Type::Unit) // Empty match
        }
    }

    fn check_lambda_expr(&mut self, params: &[String], body: &Expr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker, optional_param_types: Option<Vec<Type>>) -> Result<Type, Vec<String>> {
        // Create a new scope for lambda parameters
        let mut lambda_env = env.enter_scope();
        let mut lambda_borrow_tracker = borrow_tracker.clone();

        // Register parameters
        let mut param_types = Vec::new();
        for (i, param) in params.iter().enumerate() {
            let param_type = if let Some(types) = &optional_param_types {
                types.get(i).cloned().unwrap_or_else(|| self.type_unifier.fresh_var())
            } else {
                self.type_unifier.fresh_var()
            };
            param_types.push(param_type.clone());

            let symbol = Symbol {
                name: param.clone(),
                type_info: param_type,
                is_mutable: false,
                status: OwnershipStatus::Owned,
                defined_at: 0,
            };
            if let Err(e) = lambda_env.insert(symbol) {
                self.errors.push(e);
            }
            lambda_borrow_tracker.declare_variable(param, BorrowState::Owned);
        }

        // Check lambda body
        let return_type = self.check_expression(body, &mut lambda_env, &mut lambda_borrow_tracker)?;

        // Return function type
        Ok(Type::Function {
            params: param_types,
            return_type: Box::new(return_type),
        })
    }

    fn check_pattern(&mut self, pattern: &Pattern, expected_type: &Type, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<(), Vec<String>> {
        match pattern {
            Pattern::Wildcard => Ok(()), // Wildcard matches anything
            Pattern::Literal(lit) => {
                let lit_type = match lit {
                    Literal::Int(_) => Type::I32,
                    Literal::Float(_) => Type::F64,
                    Literal::Bool(_) => Type::Bool,
                    Literal::Str(_) => Type::String,
                };
                if &lit_type != expected_type {
                    self.errors.push(format!("Pattern literal type mismatch: expected {:?}, found {:?}", expected_type, lit_type));
                }
                Ok(())
            }
            Pattern::Identifier(name) => {
                // Bind the identifier to the expected type
                let symbol = Symbol {
                    name: name.clone(),
                    type_info: expected_type.clone(),
                    is_mutable: false,
                    status: OwnershipStatus::Owned,
                    defined_at: 0,
                };
                if let Err(e) = env.insert(symbol) {
                    self.errors.push(e);
                }
                borrow_tracker.declare_variable(name, BorrowState::Owned);
                Ok(())
            }
            Pattern::Tuple(_patterns) => {
                // For now, assume tuple patterns match any type
                // In a full implementation, we'd check tuple structure
                Ok(())
            }
        }
    }


    fn check_call_expr(&mut self, callee: &Expr, args: &[Expr], env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        if let Expr::Identifier(name) = callee {
            match name.as_str() {
                "print" => {
                    for arg in args {
                        self.check_expression(arg, env, borrow_tracker)?;
                    }
                    return Ok(Type::Unit);
                }
                "assert" => {
                     if args.len() != 1 { self.errors.push("assert expects 1 arg".to_string()); }
                     else {
                         let t = self.check_expression(&args[0], env, borrow_tracker)?;
                         self.type_unifier.add_constraint(t, Type::Bool);
                     }
                     return Ok(Type::Unit);
                }
                "assert_eq" => {
                     if args.len() != 2 { self.errors.push("assert_eq expects 2 args".to_string()); }
                     else {
                         let t1 = self.check_expression(&args[0], env, borrow_tracker)?;
                         let t2 = self.check_expression(&args[1], env, borrow_tracker)?;
                         self.type_unifier.add_constraint(t1, t2);
                     }
                     return Ok(Type::Unit);
                }
                "map" => {
                     if args.len() != 2 { 
                         self.errors.push("map expects 2 args".to_string()); 
                         return Ok(Type::Unknown);
                     }
                     let list_type = self.check_expression(&args[0], env, borrow_tracker)?;
                     
                      let item_type = if let Type::List(inner) = &list_type {
                          (**inner).clone()
                      } else {
                          self.type_unifier.fresh_var()
                      };
                      
                      let func_type = if let Expr::Lambda(params, body) = &args[1] {
                          self.check_lambda_expr(params, body, env, borrow_tracker, Some(vec![item_type.clone()]))?
                      } else {
                          self.check_expression(&args[1], env, borrow_tracker)?
                      };
                      
                      let result_type = self.type_unifier.fresh_var();
                      let expected_func_type = Type::Function {
                          params: vec![item_type],
                          return_type: Box::new(result_type.clone())
                      };
                      self.type_unifier.add_constraint(func_type, expected_func_type);
                      
                      return Ok(Type::List(Box::new(result_type)));
                }
                "filter" => {
                     if args.len() != 2 { self.errors.push("filter expects 2 args".to_string()); return Ok(Type::Unknown); }
                     let list_type = self.check_expression(&args[0], env, borrow_tracker)?;
                      let item_type = if let Type::List(inner) = &list_type {
                          (**inner).clone()
                      } else {
                          self.type_unifier.fresh_var()
                      };
                      
                      let func_type = if let Expr::Lambda(params, body) = &args[1] {
                          self.check_lambda_expr(params, body, env, borrow_tracker, Some(vec![item_type.clone()]))?
                      } else {
                          self.check_expression(&args[1], env, borrow_tracker)?
                      };
                      
                      self.type_unifier.add_constraint(list_type, Type::List(Box::new(item_type.clone())));
                      
                      let expected_func_type = Type::Function { params: vec![item_type.clone()], return_type: Box::new(Type::Bool) };
                      self.type_unifier.add_constraint(func_type, expected_func_type);
                     
                     return Ok(Type::List(Box::new(item_type)));
                }
                "reduce" => {
                    if args.len() != 3 { self.errors.push("reduce expects 3 args".to_string()); return Ok(Type::Unknown); }
                    let list_type = self.check_expression(&args[0], env, borrow_tracker)?;
                    let init_type = self.check_expression(&args[2], env, borrow_tracker)?;
                    
                      let item_type = if let Type::List(inner) = &list_type {
                          (**inner).clone()
                      } else {
                          self.type_unifier.fresh_var()
                      };
                      
                      let func_type = if let Expr::Lambda(params, body) = &args[1] {
                          self.check_lambda_expr(params, body, env, borrow_tracker, Some(vec![init_type.clone(), item_type.clone()]))?
                      } else {
                          self.check_expression(&args[1], env, borrow_tracker)?
                      };
                      
                    let acc_type = init_type.clone(); // or fresh var and constrain logic? init determines acc type usually.
                    
                    self.type_unifier.add_constraint(list_type, Type::List(Box::new(item_type.clone())));
                     // func: (acc, item) -> acc
                    let expected_func_type = Type::Function {
                        params: vec![acc_type.clone(), item_type],
                        return_type: Box::new(acc_type.clone())
                    };
                    self.type_unifier.add_constraint(func_type, expected_func_type);
                    
                    return Ok(acc_type);
                }
                "io_open" => {
                    if args.len() != 1 { self.errors.push("io_open expects 1 arg".to_string()); }
                    return Ok(Type::Port);
                }
                "io_write" => {
                    if args.len() != 2 { self.errors.push("io_write expects 2 args".to_string()); }
                    return Ok(Type::Bool);
                }
                "io_read" => {
                    if args.len() != 1 { self.errors.push("io_read expects 1 arg".to_string()); }
                    return Ok(Type::String);
                }
                "io_poll" => {
                    return Ok(Type::Bool);
                }
                "hid_get_key" => {
                    return Ok(Type::F64);
                }
                "cam_capture" => {
                    return Ok(Type::Stream);
                }
                "math_exp" | "math_sqrt" | "math_sin" | "math_cos" => {
                    if args.len() != 1 { self.errors.push(format!("{} expects 1 arg", name)); }
                    // Should verify arg is number, simple pass for now
                    return Ok(Type::F64);
                }
                "math_random" => {
                    return Ok(Type::F64);
                }
                "crypto_hash" => {
                    if args.len() != 1 { self.errors.push(format!("{} expects 1 arg", name)); }
                    // Should check arg type is string
                    return Ok(Type::String);
                }
                "time_now" => {
                    return Ok(Type::F64);
                }
                "str_len" => {
                    if args.len() != 1 { self.errors.push(format!("{} expects 1 arg", name)); }
                    return Ok(Type::F64);
                }
                "str_sub" => {
                    if args.len() != 3 { self.errors.push(format!("{} expects 3 args", name)); }
                    return Ok(Type::String);
                }
                "str_replace" => {
                    if args.len() != 3 { self.errors.push(format!("{} expects 3 args", name)); }
                    return Ok(Type::String);
                }
                _ => {} // Fallthrough
            }
        }

        let callee_type = self.check_expression(callee, env, borrow_tracker)?;

        if let Type::Function { params: param_types, return_type } = callee_type {
            if args.len() != param_types.len() {
                self.errors.push(format!(
                    "Incorrect number of arguments for function call: expected {}, found {}",
                    param_types.len(),
                    args.len()
                ));
                return Ok(*return_type); // Return expected return type even on error
            }

            for (_i, (arg_expr, expected_type)) in args.iter().zip(param_types.iter()).enumerate() {
                let arg_type = self.check_expression(arg_expr, env, borrow_tracker)?;
                // Instead of strict check, add constraint for inference
                self.type_unifier.add_constraint(arg_type.clone(), expected_type.clone());
                
                // Note: strict check removed to allow InferenceVar to unify later.
                // However, if types are concrete and mismatch, unify will report error later.

                if self.in_ownership_mode && !arg_type.is_copy_type() {
                     if let Expr::Identifier(name) = arg_expr {
                        borrow_tracker.move_var(name).map_err(|e| self.errors.push(e)).ok();
                    }
                }
            }

            Ok(*return_type)
        } else {
            self.errors.push(format!("Cannot call non-function type {:?}", callee_type));
            Ok(Type::Unknown)
        }
    }
    
    fn check_literal(&self, lit: &Literal) -> Result<Type, Vec<String>> {
        match lit {
            Literal::Int(_) => Ok(Type::I32),
            Literal::Float(_) => Ok(Type::F64),
            Literal::Bool(_) => Ok(Type::Bool),
            Literal::Str(_) => Ok(Type::String),
        }
    }
    
    fn check_identifier(&mut self, name: &str, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        if self.in_ownership_mode {
            if let Some(state) = borrow_tracker.get_state(name) {
                if state == BorrowState::Moved {
                    self.errors.push(format!("Use of moved value: '{}'", name));
                    return Ok(Type::Unknown);
                }
            }
        }
        
        if let Some(symbol) = env.lookup(name) {
            Ok(symbol.type_info.clone())
        } else {
            self.errors.push(format!("Undefined variable: '{}'", name));
            Ok(Type::Unknown)
        }
    }
    
    fn check_binary_op(&mut self, left: &Expr, op: &BinaryOp, right: &Expr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        let left_type = self.check_expression(left, env, borrow_tracker)?;
        
        // For Dot access, we don't evaluate the right side in the environment
        if *op == BinaryOp::Dot {
            if let Type::Named(struct_name) = &left_type {
                if let Expr::Identifier(field_name) = right {
                    if let Some(s) = self.structs.get(struct_name) {
                        if let Some(f) = s.fields.iter().find(|f| f.name == *field_name) {
                            return Ok(Type::from_ast_type(&f.field_type));
                        } else {
                            self.errors.push(format!("Struct '{}' has no field '{}'", struct_name, field_name));
                        }
                    } else {
                        self.errors.push(format!("Undefined struct: '{}'", struct_name));
                    }
                } else {
                    self.errors.push(format!("Right side of '.' must be an identifier, found {:?}", right));
                }
            } else if left_type != Type::Unknown {
                self.errors.push(format!("Cannot access field on non-struct type {:?}", left_type));
            }
            return Ok(Type::Unknown);
        }

        let right_type = self.check_expression(right, env, borrow_tracker)?;
        
        match op {
            BinaryOp::Assign => {
                if self.in_ownership_mode {
                    if !right_type.is_copy_type() {
                         if let Expr::Identifier(name) = right {
                            borrow_tracker.move_var(name).map_err(|e| self.errors.push(e)).ok();
                        }
                    }
                }
                Ok(right_type)
            }
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                if left_type == Type::I32 && right_type == Type::I32 {
                    Ok(Type::I32)
                } else if (left_type == Type::F64 && right_type == Type::F64) || 
                          (left_type == Type::F64 && right_type == Type::I32) ||
                          (left_type == Type::I32 && right_type == Type::F64) {
                    Ok(Type::F64)
                } else if *op == BinaryOp::Add {
                    // String Concatenation Support (Implicit)
                    let left_is_str = matches!(left_type, Type::String) || matches!(left_type, Type::Named(ref s) if s == "String");
                    let right_is_str = matches!(right_type, Type::String) || matches!(right_type, Type::Named(ref s) if s == "String");
                    
                    if left_is_str || right_is_str {
                        return Ok(Type::String);
                    }

                    if let (Type::List(l), Type::List(r)) = (&left_type, &right_type) {
                        if l == r {
                            return Ok(left_type.clone());
                        }
                    }
                    self.errors.push(format!("Type mismatch in binary operation: {:?} {:?} {:?}", left_type, op, right_type));
                    Ok(Type::Unknown)
                } else {
                    self.errors.push(format!("Type mismatch in binary operation: {:?} {:?} {:?}", left_type, op, right_type));
                    Ok(Type::Unknown)
                }
            }
            BinaryOp::And | BinaryOp::Or => {
                if left_type == Type::Bool && right_type == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    self.errors.push(format!("Logical operators expect booleans, found {:?} and {:?}", left_type, right_type));
                    Ok(Type::Unknown)
                }
            }
            _ => Ok(Type::Bool), // Assume comparisons return bool (Eq, Neq, Lt, Gt, etc)
        }
    }
    
    fn check_unary_op(&mut self, op: &UnaryOp, operand: &Expr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        let operand_type = self.check_expression(operand, env, borrow_tracker)?;
        
        match op {
            UnaryOp::Ref => {
                if self.in_ownership_mode {
                    if let Expr::Identifier(name) = operand {
                        borrow_tracker.borrow(name, false).map_err(|e| self.errors.push(e)).ok();
                    }
                }
                Ok(Type::Reference(Box::new(operand_type), false))
            },
            UnaryOp::RefMut => {
                if self.in_ownership_mode {
                    if let Expr::Identifier(name) = operand {
                         borrow_tracker.borrow(name, true).map_err(|e| self.errors.push(e)).ok();
                    }
                }
                Ok(Type::Reference(Box::new(operand_type), true))
            },
            _ => Ok(operand_type),
        }
    }
}


impl BorrowTracker {
    fn new() -> Self {
        BorrowTracker {
            variables: HashMap::new(),
        }
    }
    
    fn declare_variable(&mut self, name: &str, state: BorrowState) {
        self.variables.insert(name.to_string(), state);
    }
    
    fn get_state(&self, name: &str) -> Option<BorrowState> {
        self.variables.get(name).cloned()
    }
    
    fn move_var(&mut self, name: &str) -> Result<(), String> {
        let state = self.variables.get(name).cloned();
        match state {
            Some(BorrowState::Owned) => {
                self.variables.insert(name.to_string(), BorrowState::Moved);
                Ok(())
            }
            Some(BorrowState::Moved) => Err(format!("Use of moved value: '{}'", name)),
            Some(BorrowState::BorrowedImmutable) | Some(BorrowState::BorrowedMutable) => {
                Err(format!("Cannot move '{}' because it is borrowed", name))
            }
            None => Err(format!("Variable '{}' not found", name)),
        }
    }

    fn borrow(&mut self, name: &str, is_mutable: bool) -> Result<(), String> {
        let state = self.variables.get(name).cloned();
        
        match state {
            Some(BorrowState::Moved) => {
                Err(format!("Cannot borrow moved value: '{}'", name))
            }
            Some(BorrowState::BorrowedMutable) => {
                Err(format!("Cannot borrow '{}' as {} because it is already borrowed as mutable", name, if is_mutable { "mutable" } else { "immutable" }))
            }
            Some(BorrowState::BorrowedImmutable) => {
                if is_mutable {
                    Err(format!("Cannot borrow '{}' as mutable because it is also borrowed as immutable", name))
                } else {
                    Ok(())
                }
            }
            Some(BorrowState::Owned) => {
                let new_state = if is_mutable { BorrowState::BorrowedMutable } else { BorrowState::BorrowedImmutable };
                self.variables.insert(name.to_string(), new_state);
                Ok(())
            }
            None => Err(format!("Variable '{}' not found", name)),
        }
    }
}
