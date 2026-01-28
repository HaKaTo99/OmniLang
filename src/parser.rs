use crate::ast::*;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse_policy(&mut self) -> Result<Policy, String> {
        if self.tokens.is_empty() {
            return Err("Cannot parse an empty token list.".to_string());
        }

        let mut intent = None;
        let mut actors = Vec::new();
        let mut context = None;
        let mut assumptions = Vec::new();
        let mut rules = Vec::new();
        let mut constraints = Vec::new();
        let mut impacts = Vec::new();
        let mut traces = Vec::new();
        let mut reviews = Vec::new();

        while !self.is_at_end() {
            let token = self.peek().clone();
            match token.token_type {
                TokenType::Intent => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after INTENT")?;
                    let desc = self.parse_text_line()?;
                    intent = Some(desc);
                }
                TokenType::Actor => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after ACTOR")?;
                    actors = self.parse_actors()?;
                }
                TokenType::Context => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after CONTEXT")?;
                    context = Some(self.parse_context()?);
                }
                TokenType::Assumption => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after ASSUMPTION")?;
                    assumptions.extend(self.parse_assumptions()?);
                }
                TokenType::Rule => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after RULE")?;
                    rules.extend(self.parse_rules()?);
                }
                TokenType::Constraint => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after CONSTRAINT")?;
                    constraints.extend(self.parse_constraints()?);
                }
                TokenType::Impact => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after IMPACT")?;
                    impacts.extend(self.parse_impacts()?);
                }
                TokenType::Trace => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after TRACE")?;
                    traces.extend(self.parse_traces()?);
                }
                TokenType::Review => {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':' after REVIEW")?;
                    reviews.extend(self.parse_reviews()?);
                }
                TokenType::Eof => break,
                _ => {
                    return Err(format!("Unexpected token {:?} at line {}", token.token_type, token.line));
                }
            }
        }

        Ok(Policy {
            intent,
            actors,
            context,
            assumptions,
            rules,
            constraints,
            impacts,
            traces,
            reviews,
        })
    }

    fn parse_text_line(&mut self) -> Result<String, String> {
        let mut content = String::new();
        while !self.is_at_end() && !self.is_section_header(self.peek()) {
            let t = self.advance();
            let chunk = match &t.token_type {
                TokenType::Ident(s) | TokenType::String(s) => s.clone(),
                TokenType::Number(n) => n.to_string(),
                TokenType::Comma => ", ".to_string(),
                // Preserve punctuation/keywords via lexeme so URLs and tags stay intact
                _ => t.lexeme.clone(),
            };
            if !chunk.is_empty() {
                if !content.is_empty()
                    && !matches!(chunk.as_str(), "," | "." | ":" | ";" | "/" | "\\" | "]" | "[")
                {
                    content.push(' ');
                }
                content.push_str(&chunk);
            }
        }
        Ok(content.trim().to_string())
    }

    fn parse_actors(&mut self) -> Result<Vec<Actor>, String> {
        let mut actors = Vec::new();
        while self.match_token(TokenType::Minus) {
            let is_primary = if self.match_token(TokenType::Primary) {
                true
            } else if self.match_token(TokenType::Secondary) {
                false
            } else {
                return Err("Expected Primary or Secondary".to_string());
            };

            self.consume(TokenType::Colon, "Expected ':'")?;
            let role = self.parse_text_line()?;
            actors.push(Actor {
                role,
                primary: is_primary,
            });
        }
        Ok(actors)
    }

    fn parse_context(&mut self) -> Result<Context, String> {
        let mut domain = None;
        let mut location = None;
        let mut phase = None;

        while self.match_token(TokenType::Minus) {
            if self.match_token(TokenType::Domain) {
                self.consume(TokenType::Colon, ":")?;
                domain = Some(self.parse_text_line()?);
            } else if self.match_token(TokenType::Lokasi) {
                self.consume(TokenType::Colon, ":")?;
                location = Some(self.parse_text_line()?);
            } else if self.match_token(TokenType::Fase) {
                self.consume(TokenType::Colon, ":")?;
                phase = Some(self.parse_text_line()?);
            } else {
                // Consume unknown context key
                self.parse_text_line()?;
            }
        }
        Ok(Context {
            domain,
            location,
            phase,
        })
    }

    fn parse_assumptions(&mut self) -> Result<Vec<String>, String> {
        let mut list = Vec::new();
        while self.match_token(TokenType::Minus) {
            list.push(self.parse_text_line()?);
        }
        Ok(list)
    }

    fn parse_rules(&mut self) -> Result<Vec<Rule>, String> {
        let mut rules = Vec::new();
        while self.match_token(TokenType::Minus) {
            if self.check(TokenType::If) {
                self.advance();
                let mut condition = String::new();
                while !self.check(TokenType::Then) && !self.is_at_end() {
                    let chunk = self.parse_text_chunk()?;
                    let is_punct = matches!(chunk.as_str(), "." | "[" | "]" | "(" | ")" | ",");
                    if !condition.is_empty() && !is_punct {
                        condition.push(' ');
                    }
                    condition.push_str(&chunk);
                }

                // Normalize spacing so evaluator sees clean tokens
                condition = condition
                    .replace(" .", ".")
                    .replace(". ", ".")
                    .replace("[ ", "[")
                    .replace(" ]", "]")
                    .replace("IN[", "IN [")
                    .replace("in[", "in [");

                if self.match_token(TokenType::Then) {
                    let action = self.parse_text_line()?;
                    rules.push(Rule::Standard(StandardRule {
                        condition: condition.trim().to_string(),
                        action,
                    }));
                    } else {
                        return Err("Unexpected end of rule: missing THEN".to_string());
                }
            } else if self.check(TokenType::Match) || matches!(self.peek().token_type, TokenType::Ident(ref s) if s.eq_ignore_ascii_case("match")) {
                rules.push(self.parse_match_rule()?);
            } else if self.check(TokenType::For) {
                rules.push(self.parse_for_rule()?);
            } else if self.check(TokenType::While) {
                rules.push(self.parse_while_rule()?);
            } else {
                // Unknown rule start, skip
                self.advance();
            }
        }
        Ok(rules)
    }
    fn parse_match_rule(&mut self) -> Result<Rule, String> {
        self.advance(); // MATCH
        let scrutinee = self.parse_text_chunk()?;
        self.consume(TokenType::LBrace, "Expected '{' after MATCH expression")?;

        let mut arms = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            self.consume(TokenType::Minus, "Expected '-' before match arm")?;
            let pattern = self.parse_text_chunk_until_arrow()?;
            self.consume(TokenType::Arrow, "Expected '=>' after pattern")?;
            let action = self.parse_text_line()?;
            arms.push(PolicyMatchArm { pattern, action });
        }

        self.consume(TokenType::RBrace, "Expected '}' after match arms")?;
        Ok(Rule::Match(PolicyMatchRule { scrutinee, arms }))
    }

    fn parse_text_chunk_until_arrow(&mut self) -> Result<String, String> {
        let mut parts = Vec::new();
        while !self.check(TokenType::Arrow) && !self.is_at_end() && !self.is_section_header(self.peek()) {
            parts.push(self.advance().lexeme.clone());
        }
        Ok(parts.join(" ").trim().to_string())
    }

    fn parse_for_rule(&mut self) -> Result<Rule, String> {
        self.advance(); // FOR
        let iterator = self.consume_ident("Expected iterator name after FOR")?;
        self.consume(TokenType::In, "Expected IN after iterator")?;
        let collection = self.consume_ident("Expected collection name after IN")?;

        self.consume(TokenType::LBrace, "Expected '{' to start FOR body")?;
        let mut body = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(TokenType::Minus) {
                if self.check(TokenType::If) {
                    self.advance();
                    let mut condition = String::new();
                    while !self.check(TokenType::Then) && !self.is_at_end() {
                        let chunk = self.parse_text_chunk()?;
                        let is_punct = matches!(chunk.as_str(), "." | "[" | "]" | "(" | ")" | ",");
                        if !condition.is_empty() && !is_punct {
                            condition.push(' ');
                        }
                        condition.push_str(&chunk);
                    }
                    condition = condition
                        .replace(" .", ".")
                        .replace(". ", ".")
                        .replace("[ ", "[")
                        .replace(" ]", "]")
                        .replace("IN[", "IN [")
                        .replace("in[", "in [");
                    if self.match_token(TokenType::Then) {
                        let action = self.parse_text_line()?;
                        body.push(Rule::Standard(StandardRule {
                            condition: condition.trim().to_string(),
                            action,
                        }));
                    }
                } else if self.check(TokenType::For) {
                    body.push(self.parse_for_rule()?);
                } else if self.check(TokenType::While) {
                    body.push(self.parse_while_rule()?);
                } else {
                    return Err("Unexpected token in FOR body".to_string());
                }
            } else {
                return Err("Expected '-' to start rule inside FOR".to_string());
            }
        }
        self.consume(TokenType::RBrace, "Expected '}' to end FOR body")?;

        Ok(Rule::For(ForLoop {
            iterator,
            collection,
            body,
        }))
    }

    fn parse_while_rule(&mut self) -> Result<Rule, String> {
        self.advance(); // WHILE
        let mut condition = String::new();
        while !self.check(TokenType::LBrace) && !self.is_at_end() {
            let chunk = self.parse_text_chunk()?;
            let is_punct = matches!(chunk.as_str(), "." | "[" | "]" | "(" | ")" | ",");
            if !condition.is_empty() && !is_punct {
                condition.push(' ');
            }
            condition.push_str(&chunk);
        }
        condition = condition
            .replace(" .", ".")
            .replace(". ", ".")
            .replace("[ ", "[")
            .replace(" ]", "]")
            .replace("IN[", "IN [")
            .replace("in[", "in [");

        self.consume(TokenType::LBrace, "Expected '{' to start WHILE body")?;
        let mut body = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(TokenType::Minus) {
                if self.check(TokenType::If) {
                    self.advance();
                    let mut cond = String::new();
                    while !self.check(TokenType::Then) && !self.is_at_end() {
                        cond.push_str(&self.parse_text_chunk()?);
                        cond.push(' ');
                    }
                    if self.match_token(TokenType::Then) {
                        let action = self.parse_text_line()?;
                        body.push(Rule::Standard(StandardRule {
                            condition: cond.trim().to_string(),
                            action,
                        }));
                    }
                } else if self.check(TokenType::For) {
                    body.push(self.parse_for_rule()?);
                } else if self.check(TokenType::While) {
                    body.push(self.parse_while_rule()?);
                } else {
                    return Err("Unexpected token in WHILE body".to_string());
                }
            } else {
                return Err("Expected '-' to start rule inside WHILE".to_string());
            }
        }
        self.consume(TokenType::RBrace, "Expected '}' to end WHILE body")?;

        Ok(Rule::While(WhileLoop {
            condition: condition.trim().to_string(),
            body,
        }))
    }

    fn parse_constraints(&mut self) -> Result<Vec<Constraint>, String> {
        let mut list = Vec::new();
        while self.match_token(TokenType::Minus) {
            let kind = if self.match_token(TokenType::Legal) {
                "Legal"
            } else if self.match_token(TokenType::Ethical) {
                "Ethical"
            } else if self.match_token(TokenType::Technical) {
                "Technical"
            } else {
                "Unknown"
            };

            if kind != "Unknown" {
                self.consume(TokenType::Colon, ":")?;
            }

            let desc = self.parse_text_line()?;
            list.push(Constraint {
                kind: kind.to_string(),
                description: desc,
            });
        }
        Ok(list)
    }

    fn parse_impacts(&mut self) -> Result<Vec<Impact>, String> {
        let mut list = Vec::new();
        while self.match_token(TokenType::Minus) {
            let kind = if self.match_token(TokenType::Benefit) {
                "Benefit"
            } else if self.match_token(TokenType::Risk) {
                "Risk"
            } else if self.match_token(TokenType::TradeOff) {
                "TradeOff"
            } else {
                "Unknown"
            };

            if kind != "Unknown" {
                self.consume(TokenType::Colon, ":")?;
            }

            let desc = self.parse_text_line()?;
            list.push(Impact {
                kind: kind.to_string(),
                description: desc,
            });
        }
        Ok(list)
    }

    fn parse_traces(&mut self) -> Result<Vec<Trace>, String> {
        let mut list = Vec::new();
        while self.match_token(TokenType::Minus) {
            let kind = if self.match_token(TokenType::Moral) {
                "Moral"
            } else if self.match_token(TokenType::Regulation) {
                "Regulation"
            } else if self.match_token(TokenType::Evidence) {
                "Evidence"
            } else {
                "Unknown"
            };

            if kind != "Unknown" {
                self.consume(TokenType::Colon, ":")?;
            }

            let link = self.parse_text_line()?;
            list.push(Trace {
                kind: kind.to_string(),
                link,
            });
        }
        Ok(list)
    }

    fn parse_reviews(&mut self) -> Result<Vec<Review>, String> {
        let mut list = Vec::new();
        while self.match_token(TokenType::Minus) {
            let mut interval = String::new();
            let mut criteria = String::new();

            if let TokenType::Ident(s) = self.peek().token_type.clone() {
                if s.to_lowercase() == "interval" {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':'")?;
                    interval = self.parse_text_line()?;
                } else if s.to_lowercase() == "criteria" {
                    self.advance();
                    self.consume(TokenType::Colon, "Expected ':'")?;
                    criteria = self.parse_text_line()?;
                } else {
                    self.parse_text_line()?;
                }
            } else {
                self.parse_text_line()?;
            }
            list.push(Review { interval, criteria });
        }
        Ok(list)
    }

    // Helper functions
    fn parse_text_chunk(&mut self) -> Result<String, String> {
        let t = self.advance();
        match &t.token_type {
            TokenType::Ident(s) | TokenType::String(s) => Ok(s.clone()),
            TokenType::Number(n) => Ok(n.to_string()),
            TokenType::Gt => Ok(">".to_string()),
            TokenType::Lt => Ok("<".to_string()),
            TokenType::Eq => Ok("==".to_string()),
            TokenType::Neq => Ok("!=".to_string()),
            TokenType::Gte => Ok(">=".to_string()),
            TokenType::Lte => Ok("<=".to_string()),
            TokenType::Assign => Ok("=".to_string()),
            TokenType::Plus => Ok("+".to_string()),
            TokenType::Minus => Ok("-".to_string()),
            TokenType::Mul => Ok("*".to_string()),
            TokenType::Div => Ok("/".to_string()),
            TokenType::Percent => Ok("%".to_string()),
            TokenType::In => Ok("IN".to_string()),
            TokenType::LParen => Ok("(".to_string()),
            TokenType::RParen => Ok(")".to_string()),
            TokenType::Dot => Ok(".".to_string()),
            TokenType::Comma => Ok(",".to_string()),
            TokenType::LBracket => Ok("[".to_string()),
            TokenType::RBracket => Ok("]".to_string()),
            TokenType::Colon => Ok(":".to_string()),
            _ => Ok("".to_string()),
        }
    }
    fn is_section_header(&self, t: &Token) -> bool {
        matches!(
            t.token_type,
            TokenType::Intent
                | TokenType::Actor
                | TokenType::Context
                | TokenType::Assumption
                | TokenType::Rule
                | TokenType::Constraint
                | TokenType::Impact
                | TokenType::Trace
                | TokenType::Review
                | TokenType::Minus
                | TokenType::LBrace
                | TokenType::RBrace
                | TokenType::Eof
        )
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.previous()
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            return matches!(t, TokenType::Eof);
        }
        // Compare the full TokenType (including inner values for Ident/String)
        self.peek().token_type == t
    }

    fn is_at_end(&self) -> bool {
        self.tokens.is_empty()
            || self.pos >= self.tokens.len()
            || self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        if self.pos >= self.tokens.len() {
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
        if self.pos == 0 {
            return &self.tokens[0];
        }
        &self.tokens[self.pos - 1]
    }

    fn match_token(&mut self, t: TokenType) -> bool {
        if self.check(t) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, t: TokenType, msg: &str) -> Result<&Token, String> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(format!(
                "{} at line {} (found {:?})",
                msg,
                self.peek().line,
                self.peek().token_type
            ))
        }
    }

    fn consume_ident(&mut self, msg: &str) -> Result<String, String> {
        if let TokenType::Ident(s) = &self.peek().token_type {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(format!("{} at line {}", msg, self.peek().line))
        }
    }

    // Programming Language Parsing Methods
    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut modules = Vec::new();

        while !self.is_at_end() {
            if self.match_token(TokenType::Ident("module".to_string())) {
                modules.push(self.parse_module()?);
            } else {
                return Err(format!("Expected module declaration, found {:?}", self.peek().token_type));
            }
        }

        Ok(Program { modules })
    }

    fn parse_module(&mut self) -> Result<Module, String> {
        let name = self.consume_ident("Expected module name")?;
        let mut mode = None;

        if self.match_token(TokenType::LParen) {
            if let TokenType::String(s) = &self.peek().token_type {
                mode = Some(s.clone());
                self.advance();
            }
            self.consume(TokenType::RParen, "Expected ')' after module mode")?;
        }

        self.consume(TokenType::LBrace, "Expected '{' to start module")?;

        let mut items = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            items.push(self.parse_item()?);
        }

        self.consume(TokenType::RBrace, "Expected '}' to end module")?;

        Ok(Module { name, mode, items })
    }

    fn parse_item(&mut self) -> Result<Item, String> {
        if self.match_token(TokenType::Ident("fn".to_string())) {
            Ok(Item::Function(self.parse_function()?))
        } else if self.match_token(TokenType::Ident("struct".to_string())) {
            Ok(Item::Struct(self.parse_struct()?))
        } else if self.match_token(TokenType::Ident("trait".to_string())) {
            Ok(Item::Trait(self.parse_trait()?))
        } else if self.match_token(TokenType::Ident("impl".to_string())) {
            Ok(Item::Impl(self.parse_impl()?))
        } else if self.match_token(TokenType::Ident("const".to_string())) {
            Ok(Item::Const(self.parse_const()?))
        } else {
            Err(format!("Expected item, found {:?}", self.peek().token_type))
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, String> {
        let name = self.consume_ident("Expected function name")?;
        self.consume(TokenType::LParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        if !self.check(TokenType::RParen) {
            loop {
                let param_name = self.consume_ident("Expected parameter name")?;
                self.consume(TokenType::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;
                params.push(Param { name: param_name, param_type });

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RParen, "Expected ')' after parameters")?;

        let return_type = if self.match_token(TokenType::RArrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(TokenType::LBrace, "Expected '{' to start function body")?;
        let body = self.parse_block()?;
        self.consume(TokenType::RBrace, "Expected '}' to end function body")?;

        Ok(FunctionDecl { name, params, return_type, body })
    }

    fn parse_struct(&mut self) -> Result<StructDecl, String> {
        let name = self.consume_ident("Expected struct name")?;
        self.consume(TokenType::LBrace, "Expected '{' to start struct")?;

        let mut fields = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let field_name = self.consume_ident("Expected field name")?;
            self.consume(TokenType::Colon, "Expected ':' after field name")?;
            let field_type = self.parse_type()?;
            fields.push(Field { name: field_name, field_type });

            if !self.match_token(TokenType::Comma) && !self.check(TokenType::RBrace) {
                return Err("Expected ',' or '}' after field".to_string());
            }
        }

        self.consume(TokenType::RBrace, "Expected '}' to end struct")?;
        Ok(StructDecl { name, fields })
    }

    fn parse_trait(&mut self) -> Result<TraitDecl, String> {
        let name = self.consume_ident("Expected trait name")?;
        self.consume(TokenType::LBrace, "Expected '{' to start trait")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(TokenType::Ident("fn".to_string())) {
                methods.push(self.parse_function()?);
            } else {
                return Err("Expected function in trait".to_string());
            }
        }

        self.consume(TokenType::RBrace, "Expected '}' to end trait")?;
        Ok(TraitDecl { name, methods })
    }

    fn parse_impl(&mut self) -> Result<ImplDecl, String> {
        let trait_name = if matches!(self.peek().token_type, TokenType::Ident(_)) && self.peek().lexeme != "for" {
            Some(self.consume_ident("Expected trait name")?)
        } else {
            None
        };

        if trait_name.is_some() {
            self.consume(TokenType::Ident("for".to_string()), "Expected 'for' after trait name")?;
        }

        let struct_name = self.consume_ident("Expected struct name")?;
        self.consume(TokenType::LBrace, "Expected '{' to start impl")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(TokenType::Ident("fn".to_string())) {
                methods.push(self.parse_function()?);
            } else {
                return Err("Expected function in impl".to_string());
            }
        }

        self.consume(TokenType::RBrace, "Expected '}' to end impl")?;
        Ok(ImplDecl { trait_name, struct_name, methods })
    }

    fn parse_const(&mut self) -> Result<ConstDecl, String> {
        let name = self.consume_ident("Expected const name")?;
        self.consume(TokenType::Colon, "Expected ':' after const name")?;
        let const_type = self.parse_type()?;
        self.consume(TokenType::Assign, "Expected '=' after const type")?;
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after const value")?;

        Ok(ConstDecl { name, value, const_type })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if self.match_token(TokenType::Ident("i32".to_string())) {
            Ok(Type::I32)
        } else if self.match_token(TokenType::Ident("f64".to_string())) {
            Ok(Type::F64)
        } else if self.match_token(TokenType::Ident("bool".to_string())) {
            Ok(Type::Bool)
        } else if self.match_token(TokenType::Ident("String".to_string())) {
            Ok(Type::Named("String".to_string()))
        } else if let TokenType::Ident(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            Ok(Type::Named(name))
        } else {
            Err(format!("Expected type, found {:?}", self.peek().token_type))
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;

        while self.match_token(TokenType::Eq) || self.match_token(TokenType::Neq) {
            let op = match self.previous().token_type {
                TokenType::Eq => BinaryOp::Eq,
                TokenType::Neq => BinaryOp::Neq,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;

        while self.match_token(TokenType::Gt) || self.match_token(TokenType::Lt) ||
              self.match_token(TokenType::Gte) || self.match_token(TokenType::Lte) {
            let op = match self.previous().token_type {
                TokenType::Gt => BinaryOp::Gt,
                TokenType::Lt => BinaryOp::Lt,
                TokenType::Gte => BinaryOp::Gte,
                TokenType::Lte => BinaryOp::Lte,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;

        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let op = match self.previous().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;

        while self.match_token(TokenType::Mul) || self.match_token(TokenType::Div) || self.match_token(TokenType::Percent) {
            let op = match self.previous().token_type {
                TokenType::Mul => BinaryOp::Mul,
                TokenType::Div => BinaryOp::Div,
                TokenType::Percent => BinaryOp::Rem,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = Expr::BinaryOp(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.match_token(TokenType::Minus) {
            let right = self.parse_unary()?;
            Ok(Expr::UnaryOp(UnaryOp::Neg, Box::new(right)))
        } else if self.match_token(TokenType::Ampersand) {
            let right = self.parse_unary()?;
            Ok(Expr::UnaryOp(UnaryOp::Ref, Box::new(right)))
        } else {
            self.parse_call()
        }
    }

    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(TokenType::LParen) {
                let mut args = Vec::new();
                if !self.check(TokenType::RParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RParen, "Expected ')' after arguments")?;
                expr = Expr::Call(Box::new(expr), args);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        if self.match_token(TokenType::Ident("true".to_string())) {
            Ok(Expr::Literal(Literal::Bool(true)))
        } else if self.match_token(TokenType::Ident("false".to_string())) {
            Ok(Expr::Literal(Literal::Bool(false)))
        } else if let TokenType::Number(n) = self.peek().token_type {
            self.advance();
            if n.fract() == 0.0 {
                Ok(Expr::Literal(Literal::Int(n as i64)))
            } else {
                Ok(Expr::Literal(Literal::Float(n)))
            }
        } else if let TokenType::String(s) = &self.peek().token_type {
            let s = s.clone();
            self.advance();
            Ok(Expr::Literal(Literal::Str(s)))
        } else if self.match_token(TokenType::Match) {
            self.parse_match()
        } else if let TokenType::Ident(name) = &self.peek().token_type {
            if name == "if" {
                self.advance();
                self.parse_if()
            } else {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
        } else if self.match_token(TokenType::Pipe) {
            self.parse_lambda()
        } else if self.match_token(TokenType::LParen) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RParen, "Expected ')' after expression")?;
            Ok(expr)
        } else if self.match_token(TokenType::LBrace) {
            let block = self.parse_block()?;
            self.consume(TokenType::RBrace, "Expected '}' after block")?;
            Ok(Expr::Block(block))
        } else if self.match_token(TokenType::LBracket) {
            self.parse_array()
        } else {
            Err(format!("Expected expression, found {:?}", self.peek().token_type))
        }
    }

    fn parse_array(&mut self) -> Result<Expr, String> {
        let mut elements = Vec::new();
        if !self.check(TokenType::RBracket) {
            loop {
                elements.push(self.parse_expression()?);
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RBracket, "Expected ']' after array elements")?;
        Ok(Expr::Array(elements))
    }

    fn parse_match(&mut self) -> Result<Expr, String> {
        let value = self.parse_expression()?;
        self.consume(TokenType::LBrace, "Expected '{' after match value")?;

        let mut arms = Vec::new();
        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.consume(TokenType::Arrow, "Expected '=>' after pattern")?;
            let body = self.parse_expression()?;
            arms.push(MatchArm { pattern, guard: None, body });

            self.match_token(TokenType::Comma);
        }

        self.consume(TokenType::RBrace, "Expected '}' to end match")?;
        Ok(Expr::Match(Box::new(value), arms))
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        if self.match_token(TokenType::Ident("_".to_string())) {
            Ok(Pattern::Wildcard)
        } else if let TokenType::Number(n) = self.peek().token_type {
            self.advance();
            if n.fract() == 0.0 {
                Ok(Pattern::Literal(Literal::Int(n as i64)))
            } else {
                Ok(Pattern::Literal(Literal::Float(n)))
            }
        } else if let TokenType::String(s) = &self.peek().token_type {
            let s = s.clone();
            self.advance();
            Ok(Pattern::Literal(Literal::Str(s)))
        } else if self.match_token(TokenType::Ident("true".to_string())) {
            Ok(Pattern::Literal(Literal::Bool(true)))
        } else if self.match_token(TokenType::Ident("false".to_string())) {
            Ok(Pattern::Literal(Literal::Bool(false)))
        } else if let TokenType::Ident(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            Ok(Pattern::Identifier(name))
        } else {
            Err(format!("Expected pattern, found {:?}", self.peek().token_type))
        }
    }

    fn parse_lambda(&mut self) -> Result<Expr, String> {
        let mut params = Vec::new();
        if !self.check(TokenType::Pipe) {
            loop {
                let param = self.consume_ident("Expected parameter name")?;
                params.push(param);
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::Pipe, "Expected '|' after lambda parameters")?;
        let body = self.parse_expression()?;
        Ok(Expr::Lambda(params, Box::new(body)))
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        let condition = self.parse_expression()?;
        self.consume(TokenType::LBrace, "Expected '{' after if condition")?;
        let then_branch = self.parse_block()?;
        self.consume(TokenType::RBrace, "Expected '}' after if body")?;

        let else_branch = if self.match_token(TokenType::Ident("else".to_string())) {
            if self.match_token(TokenType::Ident("if".to_string())) {
                Some(Box::new(self.parse_if()?))
            } else {
                self.consume(TokenType::LBrace, "Expected '{' after else")?;
                let block = self.parse_block()?;
                self.consume(TokenType::RBrace, "Expected '}' after else body")?;
                Some(Box::new(Expr::Block(block)))
            }
        } else {
            None
        };

        Ok(Expr::If(IfExpr { condition: Box::new(condition), then_branch, else_branch }))
    }

    fn parse_block(&mut self) -> Result<BlockExpr, String> {
        let mut statements = Vec::new();
        let mut final_expr = None;

        while !self.check(TokenType::RBrace) && !self.is_at_end() {
            if self.match_token(TokenType::Ident("let".to_string())) {
                statements.push(Stmt::Let(self.parse_let_statement()?));
            } else if self.match_token(TokenType::Ident("return".to_string())) {
                let expr = self.parse_expression()?;
                statements.push(Stmt::Return(expr));
                self.consume(TokenType::Semicolon, "Expected ';' after return")?;
            } else {
                let expr = self.parse_expression()?;
                if self.check(TokenType::Semicolon) {
                    self.advance();
                    statements.push(Stmt::Expr(expr));
                } else {
                    final_expr = Some(Box::new(expr));
                    break;
                }
            }
        }

        Ok(BlockExpr { statements, final_expr })
    }

    fn parse_let_statement(&mut self) -> Result<LetStmt, String> {
        let is_mut = self.match_token(TokenType::Ident("mut".to_string()));
        let name = self.consume_ident("Expected variable name")?;
        let type_annotation = if self.match_token(TokenType::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.consume(TokenType::Assign, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after let statement")?;

        Ok(LetStmt { name, value, type_annotation, is_mut })
    }
}
