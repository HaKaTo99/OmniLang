use crate::lexer::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            chars: input.chars().peekable(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.chars.next().unwrap_or('\0');
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EqEq
                } else if self.peek_char() == '>' {
                    self.read_char();
                    Token::FatArrow
                } else {
                    Token::Eq
                }
            }
            '+' => Token::Plus,
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Star,
            '/' => {
                if self.peek_char() == '/' {
                    self.read_comment();
                    return self.next_token();
                } else {
                    Token::Slash
                }
            }
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '.' => Token::Dot,
            '\0' => Token::Eof,
            '@' => self.read_annotation(),
            '"' => self.read_string(),
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Not
                }
            }
            _ => {
                if is_letter(self.ch) {
                    return self.read_identifier();
                } else if is_digit(self.ch) {
                    return self.read_number();
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while is_letter(self.ch) || is_digit(self.ch) {
            self.read_char();
        }
        let literal = &self.input[position..self.position];
        // self.read_position is ahead by 1, so we need to step back for the main loop to consume the next char correctly?? 
        // No, standard loop moves forward. But here we consumed until non-letter. 
        // The main loop calls read_char() at the end. We need to NOT consume if we are already at the next token start.
        // Actually, the standard pattern is: current char is valid, peek next.
        // My read_identifier loop consumes until ch is NOT a letter.
        // So ch is now the first non-letter.
        // Returning from here, next_token will call read_char(), skipping this non-letter.
        // This is a bug in common implementations if not careful.
        // Correct approach: The loop consumes the identifier. `ch` ends up being the separator.
        // We should return immediately without advancing `read_char` in `next_token`?
        // Let's adjust: simply return, and do NOT call read_char() at the end of next_token for this path.
        // I will fix the structure in next_token to handle this.
        
        // Wait, the `next_token` calls `read_char()` at the end. 
        // If I return here, I must allow `next_token` to NOT advance, or I advance here and `ch` becomes next.
        // If `ch` is the separator (e.g., whitespace or parens), we want `next_token` to process it in the NEXT call.
        // So `ch` should be the separator.
        // But `read_identifier` advances `position`.
        // If I return the token, the caller `next_token` finishes. 
        // But `next_token` has `self.read_char()` at the end.
        // Implicitly, this would skip the separator `ch`.
        // So, `read_identifier` should NOT consume the separator?
        // With `while is_letter(self.ch) { self.read_char(); }`, `ch` is the separator.
        // If I return, `next_token` proceeds to `self.read_char()`, consuming the separator.
        // This is WRONG if the separator is meaningful (like `)`).
        // It is OK if it is whitespace.
        // So, I should NOT call `read_char` at the end of `next_token` if I entered this block?
        // Refactoring `next_token` to handle early returns.
        
        match literal {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "int" => Token::TypeInt,
            "string" => Token::TypeString,
            _ => Token::Ident(literal.to_string()),
        }
    }
    
    fn read_number(&mut self) -> Token {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        let literal = &self.input[position..self.position];
        Token::Int(literal.parse().unwrap_or(0))
    }

    fn read_string(&mut self) -> Token {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        let literal = &self.input[position..self.position];
        Token::String(literal.to_string())
    }

    fn read_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
        self.skip_whitespace();
    }

    fn read_annotation(&mut self) -> Token {
        self.read_char(); // skip '@'
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        let literal = &self.input[position..self.position];
        match literal {
            "mesh" => Token::AtMesh,
            "oracle" => Token::AtOracle,
            "quantum" => Token::AtQuantum,
            _ => Token::Illegal,
        }
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
             // Peekable chars
             // This logic of mixing chars iterator and manual index is tricky.
             // Simplification: just use chars iterator?
             // Or just recreate peek based on input string slice?
             self.input.chars().nth(self.read_position).unwrap_or('\0')
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_numeric()
}
