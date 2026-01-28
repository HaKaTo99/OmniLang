// src/types.rs
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,   // Default integer (i64 in parser) -> maps to I32 or similar later
    Float, // Default float (f64)
    I32,
    F64,
    Bool,
    Str,
    String, // Wrapper for Str to match parser expectations if needed
    Named(String),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Reference(Box<Type>, bool), // inner type, is_mutable

    List(Box<Type>),
    Tuple(Vec<Type>),
    Channel(Box<Type>),
    Tensor(Vec<usize>),
    Unit,
    Unknown,
    // For inference during type checking
    InferenceVar(usize),
}

impl Type {
    pub fn is_copy_type(&self) -> bool {
        match self {
            Type::Int | Type::Float | Type::I32 | Type::F64 | Type::Bool => true,
            Type::Reference(_, _) => true,
            Type::Named(name) => {
                // TODO: Check if struct has #[derive(Copy)]
                // For now, assume non-copy by default
                false
            }
            Type::Tuple(types) => types.iter().all(|t| t.is_copy_type()),
            _ => false,
        }
    }
    
    pub fn get_inner_type(&self) -> Option<&Type> {
        match self {
            Type::Reference(inner, _) => Some(inner),
            Type::List(inner) => Some(inner),
            Type::Channel(inner) => Some(inner),
            _ => None,
        }
    }
    
    pub fn from_ast_type(ast_type: &crate::ast::Type) -> Self {
         match ast_type {
            crate::ast::Type::I32 => Type::I32,
            crate::ast::Type::F64 => Type::F64,
            crate::ast::Type::Bool => Type::Bool,
            crate::ast::Type::Named(name) => {
                if name == "String" {
                    Type::String
                } else {
                    Type::Named(name.clone())
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipStatus {
    Owned,
    BorrowedImmutable,
    BorrowedMutable,
    Moved,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub type_info: Type,
    pub is_mutable: bool,
    pub status: OwnershipStatus,
    pub defined_at: usize, // For error reporting (line number)
}

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    pub variables: HashMap<String, Symbol>,
    pub parent: Option<Box<TypeEnvironment>>,
    pub scope_depth: usize,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        TypeEnvironment {
            variables: HashMap::new(),
            parent: None,
            scope_depth: 0,
        }
    }
    
    pub fn enter_scope(&self) -> Self {
        TypeEnvironment {
            variables: HashMap::new(),
            parent: Some(Box::new(self.clone())),
            scope_depth: self.scope_depth + 1,
        }
    }
    
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.variables.get(name).or_else(|| {
            self.parent.as_ref().and_then(|p| p.lookup(name))
        })
    }
    
    pub fn insert(&mut self, symbol: Symbol) -> Result<(), String> {
        if self.variables.contains_key(&symbol.name) {
            return Err(format!("Variable '{}' already defined in this scope", symbol.name));
        }
        self.variables.insert(symbol.name.clone(), symbol);
        Ok(())
    }
}

// Type unification for inference
#[derive(Debug)]
pub struct TypeUnifier {
    next_var: usize,
    constraints: Vec<(Type, Type)>,
    substitutions: HashMap<usize, Type>,
}

impl TypeUnifier {
    pub fn new() -> Self {
        TypeUnifier {
            next_var: 0,
            constraints: Vec::new(),
            substitutions: HashMap::new(),
        }
    }
    
    pub fn fresh_var(&mut self) -> Type {
        let var = Type::InferenceVar(self.next_var);
        self.next_var += 1;
        var
    }
    
    pub fn add_constraint(&mut self, t1: Type, t2: Type) {
        self.constraints.push((t1, t2));
    }
    
    pub fn unify(&mut self) -> Result<(), String> {
        while let Some((mut t1, mut t2)) = self.constraints.pop() {
            self.substitute(&mut t1);
            self.substitute(&mut t2);
            
            match (&t1, &t2) {
                (Type::InferenceVar(v1), Type::InferenceVar(v2)) if v1 == v2 => continue,
                (Type::InferenceVar(v), _) => {
                    if self.occurs_check(*v, &t2) {
                        return Err(format!("Occurs check failed for type variable {}", v));
                    }
                    self.substitutions.insert(*v, t2);
                }
                (_, Type::InferenceVar(v)) => {
                    if self.occurs_check(*v, &t1) {
                        return Err(format!("Occurs check failed for type variable {}", v));
                    }
                    self.substitutions.insert(*v, t1);
                }
                (Type::Reference(inner1, mut1), Type::Reference(inner2, mut2)) => {
                    if mut1 != mut2 {
                        return Err(format!("Mismatched mutability in reference types"));
                    }
                    self.add_constraint(*inner1.clone(), *inner2.clone());
                }
                (Type::List(inner1), Type::List(inner2)) => {
                   self.add_constraint(*inner1.clone(), *inner2.clone());
                }
                (Type::Tuple(types1), Type::Tuple(types2)) => {
                    if types1.len() != types2.len() {
                        return Err(format!("Tuple length mismatch"));
                    }
                    for (t1, t2) in types1.iter().zip(types2.iter()) {
                        self.add_constraint(t1.clone(), t2.clone());
                    }
                }
                _ => {
                    if t1 != t2 {
                        return Err(format!("Type mismatch: {:?} vs {:?}", t1, t2));
                    }
                }
            }
        }
        Ok(())
    }
    
    fn substitute(&mut self, ty: &mut Type) {
        match ty {
            Type::InferenceVar(v) => {
                if let Some(subst) = self.substitutions.get(v) {
                    *ty = subst.clone();
                }
            }
            Type::Reference(inner, _) => {
                self.substitute(inner);
            }
            Type::List(inner) => {
                self.substitute(inner);
            }
            Type::Tuple(types) => {
                for t in types {
                    self.substitute(t);
                }
            }
            Type::Channel(inner) => {
                self.substitute(inner);
            }
            _ => {}
        }
    }
    
    fn occurs_check(&self, var: usize, ty: &Type) -> bool {
        match ty {
            Type::InferenceVar(v) => *v == var,
            Type::Reference(inner, _) => self.occurs_check(var, inner),
            Type::List(inner) => self.occurs_check(var, inner),
            Type::Tuple(types) => types.iter().any(|t| self.occurs_check(var, t)),
            Type::Channel(inner) => self.occurs_check(var, inner),
            _ => false,
        }
    }
}
