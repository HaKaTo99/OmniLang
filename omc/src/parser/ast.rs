use crate::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
        kind: Option<String>, // Type annotation
    },
    Return(Option<Expression>),
    Expression(Expression),
    Block(Vec<Statement>),
    Function {
        name: String,
        params: Vec<(String, String)>, // name, type
        return_type: Option<String>,
        body: Box<Statement>, // Block
        annotations: Vec<String>, // @mesh, @oracle
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    String(String),
    Boolean(bool),
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>, // Block
        alternative: Option<Box<Statement>>, // Block
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Match(MatchExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpr {
    pub scrutinee: Box<Expression>,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Box<Statement>, // Block or ExpressionStatement
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(Expression), // Integer, String, Boolean
    Wildcard,
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}
