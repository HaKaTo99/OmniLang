// src/ast.rs

// Root dari dokumen Kebijakan OmniLang
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Policy {
    pub intent: Option<String>,
    pub actors: Vec<Actor>,
    pub context: Option<Context>,
    pub assumptions: Vec<String>,
    pub rules: Vec<Rule>,
    pub constraints: Vec<Constraint>,
    pub impacts: Vec<Impact>,
    pub traces: Vec<Trace>,
    pub reviews: Vec<Review>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Review {
    pub interval: String,
    pub criteria: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Actor {
    pub role: String,
    pub primary: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Context {
    pub domain: Option<String>,
    pub location: Option<String>,
    pub phase: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Rule {
    Standard(StandardRule),
    For(ForLoop),
    While(WhileLoop),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StandardRule {
    pub condition: String,
    pub action: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ForLoop {
    pub iterator: String,
    pub collection: String,
    pub body: Vec<Rule>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WhileLoop {
    pub condition: String,
    pub body: Vec<Rule>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Constraint {
    pub kind: String, // Legal, Ethical, Technical
    pub description: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Impact {
    pub kind: String, // Benefit, Risk
    pub description: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Trace {
    pub kind: String, // Moral, Regulation, Evidence
    pub link: String,
}

// Programming Language AST
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Block(BlockExpr),
    If(IfExpr),
    Match(Box<Expr>, Vec<MatchArm>),
    Lambda(Vec<String>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
    Ref,
    RefMut,
}

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub statements: Vec<Stmt>,
    pub final_expr: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(LetStmt),
    Expr(Expr),
    Return(Expr),
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    pub value: Expr,
    pub type_annotation: Option<Type>,
    pub is_mut: bool,
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub then_branch: BlockExpr,
    pub else_branch: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Identifier(String),
    Tuple(Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
}

#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub name: String,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone)]
pub struct ImplDecl {
    pub trait_name: Option<String>,
    pub struct_name: String,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone)]
pub struct ConstDecl {
    pub name: String,
    pub value: Expr,
    pub const_type: Type,
}

#[derive(Debug, Clone)]
pub enum Item {
    Function(FunctionDecl),
    Struct(StructDecl),
    Trait(TraitDecl),
    Impl(ImplDecl),
    Const(ConstDecl),
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub mode: Option<String>, // "@gc", "@ownership", etc.
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub modules: Vec<Module>,
}
