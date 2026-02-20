
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    Mut,
    Const,
    If,
    Else,
    Match,
    Loop,
    For,
    In,
    Return,
    Break,
    Continue,
    Import,
    Pub,
    Struct,
    Enum,
    Trait,
    Impl,
    Go,
    Select,
    Case,
    Default,
    Module, // New: Added for program/module support
    
    // Types
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeChar,
    TypeString,
    TypeByte,
    
    // Literals
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    And,
    Or,
    Not,
    Arrow, // ->
    FatArrow, // =>
    Pipe, // New: |
    
    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    
    // xAetherOS Annotations
    AtMesh,
    AtOracle,
    AtQuantum,
    AtBci,
    AtPqc,
    AtAbility,
    AtRequires,
    
    // Special
    Eof,
    Illegal,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
