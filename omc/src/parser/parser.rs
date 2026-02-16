use crate::lexer::token::Token;
use crate::lexer::lexer::Lexer;
use crate::parser::ast::{Program, Statement, Expression};

pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Self {
        let mut p = Parser {
            l,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: vec![],
        };
        // Read two tokens to set cur and peek
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != Token::Eof {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                program.statements.push(s);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Fn => self.parse_function_statement(), // New
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }
        Some(Statement::Expression(expr))
    }

    fn parse_function_statement(&mut self) -> Option<Statement> {
        self.next_token(); // skip fn

        let name = match &self.cur_token {
            Token::Ident(n) => n.clone(),
            _ => return None,
        };
        // self.next_token(); // REMOVED: Do not consume LParen yet, let expect_peek do it.

        if !self.expect_peek(Token::LParen) {
            return None;
        }

        let params = self.parse_function_parameters();

        // TODO: Return type parsing (-> Type)
        
        if !self.expect_peek(Token::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Statement::Function {
            name,
            params,
            return_type: None,
            body: Box::new(Statement::Block(body)),
            annotations: vec![],
        })
    }

    fn parse_function_parameters(&mut self) -> Vec<(String, String)> {
        let mut params = vec![];

        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return params;
        }

        self.next_token();

        // Fix borrow checker : clone name first
        let first_param = if let Token::Ident(name) = &self.cur_token {
            Some(name.clone())
        } else {
            None
        };

        if let Some(name) = first_param {
             let mut p_type = "Any".to_string();
             if self.peek_token_is(Token::Colon) {
                 self.next_token();
                 self.next_token();
                 if let Token::Ident(t) = &self.cur_token {
                     p_type = t.clone();
                 } else if let Token::TypeInt = &self.cur_token {
                     p_type = "int".to_string();
                 }
             }
             params.push((name, p_type));
        }

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            
            let param_name = if let Token::Ident(name) = &self.cur_token {
                Some(name.clone())
            } else {
                None
            };
            
            if let Some(name) = param_name {
                 let mut p_type = "Any".to_string();
                 if self.peek_token_is(Token::Colon) {
                     self.next_token();
                     self.next_token();
                     if let Token::Ident(t) = &self.cur_token {
                         p_type = t.clone();
                     } else if let Token::TypeInt = &self.cur_token {
                        p_type = "int".to_string();
                    }
                 }
                 params.push((name, p_type));
            }
        }

        if !self.expect_peek(Token::RParen) {
            return vec![];
        }

        params
    }

    fn parse_block_statement(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        self.next_token();

        while !self.cur_token_is(Token::RBrace) && !self.cur_token_is(Token::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        statements
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // 1. Prefix
        let mut left = match &self.cur_token {
            Token::Ident(name) => Some(Expression::Identifier(name.clone())),
            Token::Int(val) => Some(Expression::Integer(*val)),
            Token::String(val) => Some(Expression::String(val.clone())),
            Token::Bool(val) => Some(Expression::Boolean(*val)),
            Token::Not | Token::Minus => self.parse_prefix_expression(),
            Token::LParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(), // New
            _ => None,
        };

        // 2. Infix
        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            match self.peek_token {
                Token::Plus | Token::Minus | Token::Slash | Token::Star |
                Token::EqEq | Token::NotEq | Token::Lt | Token::Gt |
                Token::LtEq | Token::GtEq => {
                    self.next_token();
                    let left_expr = left.unwrap();
                    left = self.parse_infix_expression(left_expr);
                }
                Token::LParen => {
                    self.next_token();
                    let function = left.unwrap();
                    left = self.parse_call_expression(function);
                }
                _ => return left,
            }
        }
        left
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        self.next_token(); // Move past 'if'
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(Token::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();
        let mut alternative = None;

        if self.peek_token_is(Token::Else) {
            self.next_token();
            
            if !self.expect_peek(Token::LBrace) {
                 // Support 'else if'?
                 // for now just else block
                 return None;
            }
            alternative = Some(Box::new(Statement::Block(self.parse_block_statement())));
        }

        Some(Expression::If {
            condition: Box::new(condition),
            consequence: Box::new(Statement::Block(consequence)),
            alternative,
        })
    }




    fn parse_let_statement(&mut self) -> Option<Statement> {
        match &self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => return None,
        }

        let name = match &self.cur_token {
            Token::Ident(n) => n.clone(),
            _ => return None,
        };

        if !self.expect_peek(Token::Eq) {
            return None;
        }

        self.next_token(); // Move to expression
        
        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let {
            name,
            value,
            kind: None,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return(value))
    }



    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.cur_token.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Some(Expression::Prefix {
            operator,
            right: Box::new(right),
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.cur_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        exp
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_call_arguments();
        Some(Expression::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut args = vec![];

        if self.peek_token_is(Token::RParen) {
            self.next_token();
            return args;
        }

        self.next_token();
        if let Some(arg) = self.parse_expression(Precedence::Lowest) {
            args.push(arg);
        }

        while self.peek_token_is(Token::Comma) {
            self.next_token();
            self.next_token();
            if let Some(arg) = self.parse_expression(Precedence::Lowest) {
                args.push(arg);
            }
        }

        if !self.expect_peek(Token::RParen) {
            return vec![];
        }

        args
    }

    fn peek_precedence(&self) -> Precedence {
        self.token_precedence(&self.peek_token)
    }

    fn cur_precedence(&self) -> Precedence {
        self.token_precedence(&self.cur_token)
    }

    fn token_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::EqEq | Token::NotEq => Precedence::Equals,
            Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Star => Precedence::Product,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn cur_token_is(&self, t: Token) -> bool {
        // Simplified check by discriminants, but exact match for now
        std::mem::discriminant(&self.cur_token) == std::mem::discriminant(&t)
    }

    fn peek_token_is(&self, t: Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(&t)
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!("Expected next token to be {:?}, got {:?} instead", t, self.peek_token);
        self.errors.push(msg);
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}


