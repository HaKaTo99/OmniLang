// src/checker.rs
use crate::ast::{Program, Module, Item, FunctionDecl, StructDecl, Stmt, LetStmt, Expr, BlockExpr, IfExpr, MatchArm, Pattern, BinaryOp, UnaryOp, Literal};
use crate::types::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug)]
pub struct Checker {
    env: TypeEnvironment,
    current_module_mode: Option<String>, // "@gc" or "@ownership"
    in_ownership_mode: bool,
    type_unifier: TypeUnifier,
    errors: Vec<String>,
    warnings: Vec<String>,
    borrow_tracker: BorrowTracker,
}

#[derive(Debug, Clone)]
struct BorrowTracker {
    // Tracks variables and their current borrow state
    variables: HashMap<String, BorrowState>,
    // Tracks which variables are borrowed by whom
    borrow_graph: HashMap<String, HashSet<String>>,
    current_scope: Vec<HashSet<String>>,
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
            warnings: Vec::new(),
            borrow_tracker: BorrowTracker::new(),
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
                Item::Trait(trait_decl) => {
                    // self.register_trait(trait_decl)?;
                }
                Item::Impl(impl_decl) => {
                    // self.register_impl(impl_decl)?;
                }
                Item::Const(const_decl) => {
                    // self.register_const(const_decl)?;
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
        let struct_type = Type::Named(struct_decl.name.clone());
        // We can add more info here, like fields, for richer type info
        // For now, just registering the name is enough for type checking.
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
        if body_type != expected_return_type {
            self.errors.push(format!(
                "Mismatched return type for function '{}': expected {:?}, found {:?}",
                func.name, expected_return_type, body_type
            ));
        }
        
        Ok(())
    }
    
    fn check_block(&mut self, block: &BlockExpr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        let mut block_env = env.enter_scope();
        
        for stmt in &block.statements {
            self.check_statement(stmt, &mut block_env, borrow_tracker)?;
        }
        
        if let Some(expr) = &block.final_expr {
            self.check_expression(expr, &mut block_env, borrow_tracker)
        } else {
            Ok(Type::Unit)
        }
    }
    
    fn check_statement(&mut self, stmt: &Stmt, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<(), Vec<String>> {
        match stmt {
            Stmt::Let(let_stmt) => self.check_let_statement(let_stmt, env, borrow_tracker),
            Stmt::Expr(expr) => {
                self.check_expression(expr, env, borrow_tracker)?;
                Ok(())
            },
            Stmt::Return(expr) => {
                // Return handling is implicitly done by final_expr in a block
                Ok(())
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
            Expr::Block(block_expr) => self.check_block(block_expr, env, borrow_tracker),
            Expr::If(if_expr) => self.check_if_expr(if_expr, env, borrow_tracker),
            Expr::Match(value, arms) => self.check_match_expr(value, arms, env, borrow_tracker),
            Expr::Lambda(params, body) => self.check_lambda_expr(params, body, env, borrow_tracker),
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
            _ => {
                self.errors.push(format!("Unsupported expression: {:?}", expr));
                Ok(Type::Unknown)
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

        if then_type != else_type {
            self.errors.push(format!(
                "If branches have mismatched types: `then` has type {:?}, but `else` has type {:?}",
                then_type, else_type
            ));
            Ok(Type::Unknown) // Return Unknown on mismatch
        } else {
            Ok(then_type) // Both branches have the same type
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

    fn check_lambda_expr(&mut self, params: &[String], body: &Expr, env: &mut TypeEnvironment, borrow_tracker: &mut BorrowTracker) -> Result<Type, Vec<String>> {
        // Create a new scope for lambda parameters
        let mut lambda_env = env.enter_scope();
        let mut lambda_borrow_tracker = borrow_tracker.clone();

        // Register parameters (assume all parameters are of type Unknown for now, or we could infer)
        let mut param_types = Vec::new();
        for param in params {
            // For simplicity, assume parameters are of type Unknown
            // In a full implementation, we'd do type inference
            let param_type = self.type_unifier.fresh_var();
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
                    Literal::Str(_) => Type::Named("String".to_string()),
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
            Pattern::Tuple(patterns) => {
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
                     let func_type = self.check_expression(&args[1], env, borrow_tracker)?;
                     
                     // Infer output type
                     let item_type = self.type_unifier.fresh_var();
                     let result_type = self.type_unifier.fresh_var();
                     
                     // Constraint: list must be List<item_type>
                     self.type_unifier.add_constraint(list_type, Type::List(Box::new(item_type.clone())));
                     
                     // Constraint: func must be item_type -> result_type
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
                     let func_type = self.check_expression(&args[1], env, borrow_tracker)?;
                     let item_type = self.type_unifier.fresh_var();
                     
                     self.type_unifier.add_constraint(list_type, Type::List(Box::new(item_type.clone())));
                     
                     let expected_func_type = Type::Function { params: vec![item_type.clone()], return_type: Box::new(Type::Bool) };
                     self.type_unifier.add_constraint(func_type, expected_func_type);
                     
                     return Ok(Type::List(Box::new(item_type)));
                }
                "reduce" => {
                    if args.len() != 3 { self.errors.push("reduce expects 3 args".to_string()); return Ok(Type::Unknown); }
                    let list_type = self.check_expression(&args[0], env, borrow_tracker)?;
                    let func_type = self.check_expression(&args[1], env, borrow_tracker)?;
                    let init_type = self.check_expression(&args[2], env, borrow_tracker)?;
                    
                    let item_type = self.type_unifier.fresh_var();
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

            for (i, (arg_expr, expected_type)) in args.iter().zip(param_types.iter()).enumerate() {
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
            Literal::Str(_) => Ok(Type::Named("String".to_string())),
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
                if (left_type == Type::I32 && right_type == Type::I32) || (left_type == Type::F64 && right_type == Type::F64) {
                    Ok(left_type)
                } else {
                    self.errors.push(format!("Type mismatch in binary operation: {:?} {:?} {:?}", left_type, op, right_type));
                    Ok(Type::Unknown)
                }
            }
            _ => Ok(Type::Bool), // Assume comparisons return bool
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
            borrow_graph: HashMap::new(),
            current_scope: vec![HashSet::new()],
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
