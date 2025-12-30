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

    // Looping
    For,
    While,
    In,

    // Pattern Matching & Functions
    Match,
    Arrow,     // =>
    Pipe,      // |

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

    Eof,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
                    } else if self.peek() == '>' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Arrow, "=>");
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
                '<' => {
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Lte, "<=");
                    } else {
                        self.add_token(&mut tokens, TokenType::Lt, "<");
                    }
                }
                '>' => {
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(&mut tokens, TokenType::Gte, ">=");
                    } else {
                        self.add_token(&mut tokens, TokenType::Gt, ">");
                    }
                }
                '(' => self.add_token(&mut tokens, TokenType::LParen, "("),
                ')' => self.add_token(&mut tokens, TokenType::RParen, ")"),
                '{' => self.add_token(&mut tokens, TokenType::LBrace, "{"),
                '}' => self.add_token(&mut tokens, TokenType::RBrace, "}"),
                '[' => self.add_token(&mut tokens, TokenType::LBracket, "["),
                ']' => self.add_token(&mut tokens, TokenType::RBracket, "]"),
                ',' => self.add_token(&mut tokens, TokenType::Comma, ","),
                '.' => self.add_token(&mut tokens, TokenType::Dot, "."),
                ':' => self.add_token(&mut tokens, TokenType::Colon, ":"),
                ';' => self.add_token(&mut tokens, TokenType::Semicolon, ";"),
                '&' => self.add_token(&mut tokens, TokenType::Ampersand, "&"),
                '%' => self.add_token(&mut tokens, TokenType::Percent, "%"),
                '"' => {
                    let s = self.read_string()?;
                    tokens.push(Token {
                        token_type: TokenType::String(s.clone()),
                        line: self.line,
                        lexeme: s,
                    });
                }
                c if c.is_ascii_digit() => {
                    let (n, s) = self.read_number();
                    tokens.push(Token {
                        token_type: TokenType::Number(n),
                        line: self.line,
                        lexeme: s,
                    });
                }
                c if c.is_alphabetic() || c == '_' => {
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

                        _ => TokenType::Ident(s.clone()),
                    };
                    // println!("DEBUG LEXER: '{}' -> {:?}", s, token_type); 
                    tokens.push(Token {
                        token_type,
                        line: self.line,
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
        while self.pos < self.input.len()
            && (self.current_char().is_ascii_digit() || self.current_char() == '.')
        {
            self.advance();
        }
        // Consume optional unit/label suffix (e.g., "1m", "1km") but ignore for numeric value
        let lexeme_end = self.pos; // Mark end of number part before suffix
        while self.pos < self.input.len() && self.current_char().is_alphabetic() {
            self.advance();
        }
        let full_lexeme: String = self.input[start..self.pos].iter().collect();
        let num_str: String = self.input[start..lexeme_end]
            .iter()
            .filter(|c| c.is_ascii_digit() || **c == '.')
            .collect();
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
