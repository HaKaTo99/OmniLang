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
    Else,
    Return,
    Let,
    Mut,
    True,
    False,
    Legal,
    Ethical,
    Technical,
    Benefit,
    Risk,
    TradeOff,
    Moral,
    Regulation,
    Evidence,

    // Looping
    For,
    While,
    In,

    // Pattern Matching & Functions
    Match,
    Arrow,     // =>
    Pipe,      // |

    // Programming Language Keywords
    Module,
    Fn,
    Struct,
    Trait,
    Impl,
    Const,
    Enum,

    // Identifiers & Literals
    Ident(String),
    Number(f64),
    String(String),

    // Symbols
    Plus,      // +
    Minus,     // -
    Mul,       // *
    Div,       // /
    Assign,    // =
    Eq,        // ==
    Neq,       // !=
    Gt,        // >
    Lt,        // <
    Gte,       // >=
    Lte,       // <=
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Comma,     // ,
    Dot,       // .
    Colon,     // :
    Semicolon, // ;
    Ampersand, // &
    Percent,   // %
    RArrow,    // ->
    And,       // &&
    Or,        // ||
    At,        // @
    Bang,      // !

    Eof,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub lexeme: String,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
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
                    self.advance();
                }
                '/' if self.peek() == '/' => {
                    // Comment
                    while self.pos < self.input.len() && self.current_char() != '\n' {
                        self.advance();
                    }
                }
                '+' => { let c=self.column; self.add_token(&mut tokens, TokenType::Plus, "+", c) }
                '-' => {
                    let c=self.column;
                    if self.peek() == '>' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::RArrow, "->", c);
                    } else {
                        self.add_token(&mut tokens, TokenType::Minus, "-", c);
                    }
                }
                '*' => { let c=self.column; self.add_token(&mut tokens, TokenType::Mul, "*", c) }
                '/' => { let c=self.column; self.add_token(&mut tokens, TokenType::Div, "/", c) }
                '=' => {
                    let c=self.column;
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Eq, "==", c);
                    } else if self.peek() == '>' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Arrow, "=>", c);
                    } else {
                        self.add_token(&mut tokens, TokenType::Assign, "=", c);
                    }
                }
                '!' => {
                    let c=self.column;
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Neq, "!=", c);
                    } else {
                        self.add_token(&mut tokens, TokenType::Bang, "!", c);
                    }
                }
                '<' => {
                    let c=self.column;
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Lte, "<=", c);
                    } else {
                        self.add_token(&mut tokens, TokenType::Lt, "<", c);
                    }
                }
                '>' => {
                    let c=self.column;
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Gte, ">=", c);
                    } else {
                        self.add_token(&mut tokens, TokenType::Gt, ">", c);
                    }
                }
                '(' => { let c=self.column; self.add_token(&mut tokens, TokenType::LParen, "(", c) }
                ')' => { let c=self.column; self.add_token(&mut tokens, TokenType::RParen, ")", c) }
                '{' => { let c=self.column; self.add_token(&mut tokens, TokenType::LBrace, "{", c) }
                '}' => { let c=self.column; self.add_token(&mut tokens, TokenType::RBrace, "}", c) }
                '[' => { let c=self.column; self.add_token(&mut tokens, TokenType::LBracket, "[", c) }
                ']' => { let c=self.column; self.add_token(&mut tokens, TokenType::RBracket, "]", c) }
                ',' => { let c=self.column; self.add_token(&mut tokens, TokenType::Comma, ",", c) }
                '.' => { let c=self.column; self.add_token(&mut tokens, TokenType::Dot, ".", c) }
                ':' => { let c=self.column; self.add_token(&mut tokens, TokenType::Colon, ":", c) }
                ';' => { let c=self.column; self.add_token(&mut tokens, TokenType::Semicolon, ";", c) }
                '&' => {
                    let c=self.column;
                    if self.peek() == '&' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::And, "&&", c);
                    } else {
                        self.add_token(&mut tokens, TokenType::Ampersand, "&", c);
                    }
                }
                '%' => { let c=self.column; self.add_token(&mut tokens, TokenType::Percent, "%", c) }
                '|' => {
                    let c=self.column;
                    if self.peek() == '|' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Or, "||", c);
                    } else {
                        self.add_token(&mut tokens, TokenType::Pipe, "|", c);
                    }
                }
                '@' => { let c=self.column; self.add_token(&mut tokens, TokenType::At, "@", c) }
                '"' => {
                    let start_col = self.column;
                    let start_line = self.line;
                    let s = self.read_string()?;
                    tokens.push(Token {
                        token_type: TokenType::String(s.clone()),
                        line: start_line,
                        column: start_col,
                        lexeme: s,
                    });
                }
                c if c.is_ascii_digit() => {
                    let start_col = self.column;
                    let start_line = self.line;
                    let (n, s) = self.read_number();
                    tokens.push(Token {
                        token_type: TokenType::Number(n),
                        line: start_line,
                        column: start_col,
                        lexeme: s,
                    });
                }
                c if c.is_alphabetic() || c == '_' => {
                    let start_col = self.column;
                    let start_line = self.line;
                    let s = self.read_identifier();
                    let s_lower = s.to_lowercase();
                    let next_non_ws = self.peek_non_whitespace();
                    let token_type = match s_lower.as_str() {
                        // Section headers (case-insensitive) only when followed by ':'
                        "intent" if matches!(next_non_ws, Some(':')) => TokenType::Intent,
                        "actor" if matches!(next_non_ws, Some(':')) => TokenType::Actor,
                        "context" if matches!(next_non_ws, Some(':')) => TokenType::Context,
                        "assumption" if matches!(next_non_ws, Some(':')) => TokenType::Assumption,
                        "rule" if matches!(next_non_ws, Some(':')) => TokenType::Rule,
                        "constraint" if matches!(next_non_ws, Some(':')) => TokenType::Constraint,
                        "impact" if matches!(next_non_ws, Some(':')) => TokenType::Impact,
                        "trace" if matches!(next_non_ws, Some(':')) => TokenType::Trace,
                        "review" if matches!(next_non_ws, Some(':')) => TokenType::Review,

                        // Sub-keys (case-insensitive, tolerate hyphen variants)
                        "primary" => TokenType::Primary,
                        "secondary" => TokenType::Secondary,
                        "domain" => TokenType::Domain,
                        "lokasi" => TokenType::Lokasi,
                        "fase" => TokenType::Fase,
                        "if" => TokenType::If,
                        "then" => TokenType::Then,
                        "else" => TokenType::Else,
                        "return" => TokenType::Return,
                        "let" => TokenType::Let,
                        "mut" => TokenType::Mut,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "legal" => TokenType::Legal,
                        "ethical" => TokenType::Ethical,
                        "technical" => TokenType::Technical,
                        "benefit" => TokenType::Benefit,
                        "risk" => TokenType::Risk,
                        "trade-off" | "tradeoff" => TokenType::TradeOff,
                        "moral" => TokenType::Moral,
                        "regulation" => TokenType::Regulation,
                        "evidence" => TokenType::Evidence,

                        "for" => TokenType::For,
                        "while" => TokenType::While,
                        "in" => TokenType::In,
                        "match" => TokenType::Match,
                        "module" => TokenType::Module,
                        "fn" => TokenType::Fn,
                        "struct" => TokenType::Struct,
                        "trait" => TokenType::Trait,
                        "impl" => TokenType::Impl,
                        "const" => TokenType::Const,
                        "enum" => TokenType::Enum,

                        _ => TokenType::Ident(s.clone()),
                    };
                    tokens.push(Token {
                        token_type,
                        line: start_line,
                        column: start_col,
                        lexeme: s,
                    });
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at line {}",
                        ch, self.line
                    ));
                }
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            column: self.column,
            lexeme: "".to_string(),
        });

        Ok(tokens)
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            if self.input[self.pos] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.pos += 1;
        }
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

    fn add_token(&mut self, tokens: &mut Vec<Token>, token_type: TokenType, lexeme: &str, col: usize) {
        tokens.push(Token {
            token_type,
            line: self.line,
            column: col,
            lexeme: lexeme.to_string(),
        });
        self.advance();
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.advance(); // Skip opening quote
        let start = self.pos;
        while self.pos < self.input.len() && self.current_char() != '"' {
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
        while self.pos < self.input.len()
            && (self.current_char().is_ascii_digit() || self.current_char() == '.')
        {
            self.advance();
        }

        // Handle scientific notation (e.g., 5.972e24 or 6.674e-11)
        if self.pos < self.input.len() && (self.current_char() == 'e' || self.current_char() == 'E') {
            self.advance();
            if self.pos < self.input.len() && (self.current_char() == '+' || self.current_char() == '-') {
                self.advance();
            }
            while self.pos < self.input.len() && self.current_char().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme_end = self.pos;
        // Consume optional unit/label suffix (e.g., "1m", "1km") but ignore for numeric value
        while self.pos < self.input.len() && self.current_char().is_alphabetic() && 
              self.current_char() != 'e' && self.current_char() != 'E' {
            self.advance();
        }
        
        let full_lexeme: String = self.input[start..self.pos].iter().collect();
        let num_str: String = self.input[start..lexeme_end].iter().collect();
        
        (num_str.parse().unwrap_or(0.0), full_lexeme)
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.current_char().is_alphanumeric()
                || self.current_char() == '_'
                || self.current_char() == '-')
        {
            self.advance();
        }
        self.input[start..self.pos].iter().collect()
    }

    fn peek_non_whitespace(&self) -> Option<char> {
        let mut idx = self.pos;
        while idx < self.input.len() {
            let c = self.input[idx];
            if c.is_whitespace() {
                idx += 1;
                continue;
            }
            return Some(c);
        }
        None
    }
}
