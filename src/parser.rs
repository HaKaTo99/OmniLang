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
        let mut intent = None;
        let mut actors = Vec::new();
        let mut context = None;
        let mut rules = Vec::new();
        let mut constraints = Vec::new();
        let mut impacts = Vec::new();
        let mut traces = Vec::new();

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
                _ => {
                    // Skip or Error? For now skip unexpected to be robust
                     self.advance();
                }
            }
        }

        Ok(Policy {
            intent,
            actors,
            context,
            rules,
            constraints,
            impacts,
            traces,
        })
    }

    fn parse_text_line(&mut self) -> Result<String, String> {
        // Collect all tokens until newline or next key section (heuristic)
        // Since lexer is simple, we might just grab Idents/Strings until next newline logic simulation
        // But our lexer doesn't pass newlines. We have to rely on tokens.
        // Heuristic: Read until we hit a "Section Header Key" or EOF. 
        // Better: Read identifiers and strings and join them.
        let mut content = String::new();
        while !self.is_at_end() && !self.is_section_header(self.peek()) {
            let t = self.advance();
            match &t.token_type {
                 TokenType::Ident(s) | TokenType::String(s) => {
                     content.push_str(s);
                     content.push(' ');
                 }
                 TokenType::Number(n) => {
                     content.push_str(&n.to_string());
                     content.push(' ');
                 }
                 TokenType::Comma => content.push_str(", "),
                 // Handle other punctuation
                 _ => {}
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
             actors.push(Actor { role, primary: is_primary });
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
        Ok(Context { domain, location, phase })
    }

    fn parse_rules(&mut self) -> Result<Vec<Rule>, String> {
        let mut rules = Vec::new();
        while self.match_token(TokenType::Minus) {
             self.consume(TokenType::If, "Expected IF")?;
             // IF condition, ... THEN action
             // This parser is tricky without line breaks. Let's assume IF <text> - THEN <text>
             // But lexer eats line breaks. 
             // We need to look for THEN token.
             let mut condition = String::new();
             while !self.check(TokenType::Then) && !self.is_at_end() {
                  // Collect condition
                   condition.push_str(&self.parse_text_chunk()?);
                   condition.push(' ');
             }
             
             // Issue: 'parse_text_chunk' above needs to advance 1 token
             
             if self.match_token(TokenType::Then) {
                 let action = self.parse_text_line()?;
                 rules.push(Rule { condition: condition.trim().to_string(), action });
             } else {
                 // Maybe Error or just skip
             }
             
             // If we are at a Minus, loop continues.
        }
        Ok(rules)
    }
    
    // Simplification for parser_rules logic above:
    // Actually, structure is:
    // - IF condition
    // - THEN action
    // This implies two list items per rule or one complex item?
    // Grammar says:
    // - IF <Condition>
    // - THEN <Action>
    // This means they are separate lines (separate list items).
    // Let's adjust logic.
    
    fn parse_constraints(&mut self) -> Result<Vec<Constraint>, String> {
         let mut list = Vec::new();
         while self.match_token(TokenType::Minus) {
             let kind = if self.match_token(TokenType::Legal) { "Legal" }
             else if self.match_token(TokenType::Ethical) { "Ethical" }
             else if self.match_token(TokenType::Technical) { "Technical" }
             else { "Unknown" };
             
             if kind != "Unknown" {
                self.consume(TokenType::Colon, ":")?;
             }
             
             let desc = self.parse_text_line()?;
             list.push(Constraint { kind: kind.to_string(), description: desc });
         }
         Ok(list)
    }

    fn parse_impacts(&mut self) -> Result<Vec<Impact>, String> {
         let mut list = Vec::new();
         while self.match_token(TokenType::Minus) {
             let kind = if self.match_token(TokenType::Benefit) { "Benefit" }
             else if self.match_token(TokenType::Risk) { "Risk" }
             else if self.match_token(TokenType::TradeOff) { "TradeOff" }
             else { "Unknown" };
             
             if kind != "Unknown" {
                self.consume(TokenType::Colon, ":")?;
             }
             
             let desc = self.parse_text_line()?;
             list.push(Impact { kind: kind.to_string(), description: desc });
         }
         Ok(list)
    }

    fn parse_traces(&mut self) -> Result<Vec<Trace>, String> {
         let mut list = Vec::new();
         while self.match_token(TokenType::Minus) {
             let kind = if self.match_token(TokenType::Moral) { "Moral" }
             else if self.match_token(TokenType::Regulation) { "Regulation" }
             else if self.match_token(TokenType::Evidence) { "Evidence" }
             else { "Unknown" };
             
             if kind != "Unknown" {
                 self.consume(TokenType::Colon, ":")?;
             }
             
             let link = self.parse_text_line()?;
             list.push(Trace { kind: kind.to_string(), link });
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
                 _ => Ok("".to_string())
          }
    }

    fn is_section_header(&self, t: &Token) -> bool {
        matches!(t.token_type, 
            TokenType::Intent | TokenType::Actor | TokenType::Context | 
            TokenType::Rule | TokenType::Constraint | TokenType::Impact | 
            TokenType::Trace | TokenType::Review | TokenType::Minus | TokenType::EOF)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        self.previous()
    }

    fn match_token(&mut self, t: TokenType) -> bool {
        if self.check(t) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
             return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&t)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.pos - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn consume(&mut self, t: TokenType, msg: &str) -> Result<&Token, String> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(format!("{} at line {}", msg, self.peek().line))
        }
    }
}

