use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Int,
    String,
    Bool,
    Function(Vec<SymbolType>, Box<SymbolType>), // Parameters, Return Type
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    #[allow(dead_code)]
    pub name: String,
    pub kind: SymbolType,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    outer: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: SymbolTable) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(Box::new(outer)),
        }
    }

    pub fn define(&mut self, name: String, kind: SymbolType) -> Symbol {
        let symbol = Symbol { name: name.clone(), kind };
        self.store.insert(name, symbol.clone());
        symbol
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        if let Some(s) = self.store.get(name) {
            return Some(s.clone());
        }

        if let Some(outer) = &self.outer {
            return outer.resolve(name);
        }

        None
    }

    pub fn into_outer(self) -> Option<SymbolTable> {
        self.outer.map(|b| *b)
    }
}
