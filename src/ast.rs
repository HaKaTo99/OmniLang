// src/ast.rs

#[derive(Debug, Clone)]
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
pub struct Review {
    pub interval: String,
    pub criteria: String,
}

#[derive(Debug, Clone)]
pub struct Actor {
    pub role: String,
    pub primary: bool,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub domain: Option<String>,
    pub location: Option<String>,
    pub phase: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    Standard(StandardRule),
    For(ForLoop),
    While(WhileLoop),
    Match(PolicyMatchRule),
}

#[derive(Debug, Clone)]
pub struct StandardRule {
    pub condition: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub iterator: String,
    pub collection: String,
    pub body: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct WhileLoop {
    pub condition: String,
    pub body: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolicyMatchRule {
    pub scrutinee: String,
    pub arms: Vec<PolicyMatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolicyMatchArm {
    pub pattern: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub kind: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Impact {
    pub kind: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub kind: String,
    pub link: String,
}

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
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>), // array[index]
    StructInit(String, Vec<(String, Expr)>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Rem, Assign, Eq, Neq, Lt, Gt, Lte, Gte, And, Or, Dot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg, Not, Ref, RefMut,
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
    While(ExprWhile),
    For(ExprFor),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I32, I64, F64, Bool, String, Named(String), List(Box<Type>),
}

#[derive(Debug, Clone)]
pub struct Decorator {
    pub name: String,
    pub args: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub decorators: Vec<Decorator>,
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Option<BlockExpr>,
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
    pub mode: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub modules: Vec<Module>,
}
#[derive(Debug, Clone)]
pub struct ExprWhile {
    pub condition: Box<Expr>,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct ExprFor {
    pub iterator: String,
    pub collection: Box<Expr>,
    pub body: BlockExpr,
}
