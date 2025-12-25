#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Section Headers
    Intent,
    Actor,
    Context,
    Assumption,
    Rule,
    Constraint,
    Impact,
    Trace,
    Review,

    // Sub-Keywords
    Primary,
    Secondary,
    Domain,
    Lokasi,
    Fase,
    If,
    Then,
    Legal,
    Ethical,
    Technical,
    Benefit,
    Risk,
    TradeOff,
    Moral,
    Regulation,
    Evidence,

    // Identifiers & Literals
    Ident(String),
    Number(f64),
    String(String),

    // Symbols
    Plus,       // +
    Minus,      // -
    Gt,         // >
    Lt,         // <
    Colon,      // :
    Comma,      // ,
    
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: String,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            let ch = self.current_char();

            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek() == '/' => {
                    // Comment
                    while self.pos < self.input.len() && self.current_char() != '\n' {
                        self.advance();
                    }
                }
                '+' => self.add_token(&mut tokens, TokenType::Plus, "+"),
                '-' => {
                    if self.peek() == '>' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::RArrow, "->");
                    } else {
                        self.add_token(&mut tokens, TokenType::Minus, "-");
                    }
                }
                '*' => self.add_token(&mut tokens, TokenType::Mul, "*"),
                '/' => self.add_token(&mut tokens, TokenType::Div, "/"),
                '=' => {
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Eq, "==");
                    } else {
                        self.add_token(&mut tokens, TokenType::Assign, "=");
                    }
                }
                '!' => {
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Neq, "!=");
                    } else {
                         return Err(format!("Unexpected character '!' at line {}", self.line));
                    }
                }
                '<' => self.add_token(&mut tokens, TokenType::Lt, "<"),
                '>' => self.add_token(&mut tokens, TokenType::Gt, ">"),
                '(' => self.add_token(&mut tokens, TokenType::LParen, "("),
                ')' => self.add_token(&mut tokens, TokenType::RParen, ")"),
                '{' => self.add_token(&mut tokens, TokenType::LBrace, "{"),
                '}' => self.add_token(&mut tokens, TokenType::RBrace, "}"),
                ',' => self.add_token(&mut tokens, TokenType::Comma, ","),
                '.' => self.add_token(&mut tokens, TokenType::Dot, "."),
                ':' => self.add_token(&mut tokens, TokenType::Colon, ":"),
                ';' => self.add_token(&mut tokens, TokenType::Semicolon, ";"),
                '&' => self.add_token(&mut tokens, TokenType::Ampersand, "&"),
                '"' => {
                    let s = self.read_string()?;
                    tokens.push(Token {
                        token_type: TokenType::String(s.clone()),
                        line: self.line,
                        lexeme: s,
                    });
                }
                c if c.is_digit(10) => {
                    let (n, s) = self.read_number();
                    tokens.push(Token {
                        token_type: TokenType::Number(n),
                        line: self.line,
                        lexeme: s,
                    });
                }
                c if c.is_alphabetic() || c == '_' => {
                    let s = self.read_identifier();
                    let token_type = match s.as_str() {
                        "INTENT" => TokenType::Intent,
                        "ACTOR" => TokenType::Actor,
                        "CONTEXT" => TokenType::Context,
                        "ASSUMPTION" => TokenType::Assumption,
                        "RULE" => TokenType::Rule,
                        "CONSTRAINT" => TokenType::Constraint,
                        "IMPACT" => TokenType::Impact,
                        "TRACE" => TokenType::Trace,
                        "REVIEW" => TokenType::Review,
                        
                        // Sub-keys (Case insensitive matching likely needed, but keeping simple first)
                        "Primary" => TokenType::Primary,
                        "Secondary" => TokenType::Secondary,
                        "Domain" => TokenType::Domain,
                        "Lokasi" => TokenType::Lokasi,
                        "Fase" => TokenType::Fase,
                        "IF" => TokenType::If,
                        "THEN" => TokenType::Then,
                        "Legal" => TokenType::Legal,
                        "Ethical" => TokenType::Ethical,
                        "Technical" => TokenType::Technical,
                        "Benefit" => TokenType::Benefit,
                        "Risk" => TokenType::Risk,
                        "Trade-off" => TokenType::TradeOff,
                        "Moral" => TokenType::Moral,
                        "Regulation" => TokenType::Regulation,
                        "Evidence" => TokenType::Evidence,
                        
                        _ => TokenType::Ident(s.clone()),
                    };
                    tokens.push(Token {
                        token_type,
                        line: self.line,
                        lexeme: s,
                    });
                }
                _ => {
                    return Err(format!("Unexpected character '{}' at line {}", ch, self.line));
                }
            }
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            line: self.line,
            lexeme: "".to_string(),
        });

        Ok(tokens)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current_char(&self) -> char {
        self.input[self.pos]
    }

    fn peek(&self) -> char {
        if self.pos + 1 < self.input.len() {
            self.input[self.pos + 1]
        } else {
            '\0'
        }
    }

    fn add_token(&mut self, tokens: &mut Vec<Token>, token_type: TokenType, lexeme: &str) {
        tokens.push(Token {
            token_type,
            line: self.line,
            lexeme: lexeme.to_string(),
        });
        self.advance();
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.advance(); // Skip opening quote
        let start = self.pos;
        while self.pos < self.input.len() && self.current_char() != '"' {
             if self.current_char() == '\n' {
                 self.line += 1;
             }
             self.advance();
        }
        
        if self.pos >= self.input.len() {
            return Err("Unterminated string literal".to_string());
        }

        let s: String = self.input[start..self.pos].iter().collect();
        self.advance(); // Skip closing quote
        Ok(s)
    }

    fn read_number(&mut self) -> (f64, String) {
        let start = self.pos;
        while self.pos < self.input.len() && (self.current_char().is_digit(10) || self.current_char() == '.') {
            self.advance();
        }
        let s: String = self.input[start..self.pos].iter().collect();
        (s.parse().unwrap_or(0.0), s)
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        self.input[start..self.pos].iter().collect()
    }
}
