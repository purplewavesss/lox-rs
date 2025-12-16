use std::num::{ParseFloatError, ParseIntError};
use crate::scanning::token::{Token, Value};
use crate::scanning::token_type::TokenType::{self, *};
use crate::throw;
use ternop::ternary;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    loc: Location
}

impl Scanner {
    /// Returns a new scanner
    pub fn new(source: &String) -> Scanner {
        Scanner { 
            source: source.chars().collect(), 
            tokens: Vec::new(),
            loc: Location::new()
        }
    }

    /// Scans all tokens given to the scanner
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // Scan while there are more tokens
        while !self.is_at_end() {
            self.loc.start = self.loc.current;
            self.scan_token();
        }

        // Append end token
        self.tokens.push(Token::new(TokenType::End, 
                                    String::new(), 
                                    Value::None(), 
                                    self.loc.line));

        // Drain and return tokens
        self.tokens.drain(..).collect()
    }

    /// Scans and parses a single token
    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            // Single-char tokens
            '(' => self.add_token(LeftParen, None),
            ')' => self.add_token(RightParen, None),
            '{' => self.add_token(LeftBrace, None),
            '}' => self.add_token(RightBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '-' => self.add_token(Minus, None),
            '+' => self.add_token(Plus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Asterisk, None),
            '%' => self.add_token(Mod, None),

            // One or more char tokens
            '!' => {
                let next_is_equal = self.match_char('=');
                self.add_token(ternary!(next_is_equal, NotEqual, Not), None)
            },
            '=' => {
                let next_is_equal = self.match_char('=');
                self.add_token(ternary!(next_is_equal, EqualEqual, Equal), None)
            },
            '<' => {
                let next_is_equal = self.match_char('=');
                self.add_token(ternary!(next_is_equal, LessEqual, Less), None)
            },
            '>' => {
                let next_is_equal = self.match_char('=');
                self.add_token(ternary!(next_is_equal, GreaterEqual, Greater), None)
            },
            '/' => {
                // Comment case
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }

                // Division op case
                else {
                    self.add_token(Slash, None);
                }
            },

            // Whitespace
            ' ' | '\r' | '\t' => (),
            '\n' => self.loc.line += 1,

            // Literals
            '"' => self.consume_string(),
            _ => {
                // Numbers
                if c.is_digit(DIGIT_RADIX) {
                    self.consume_number();
                }

                // Identifiers
                else if c.is_alphabetic() {
                    self.consume_identifier();
                }

                else {
                    throw(self.loc.line.to_string(), "Unexpected character.");
                }
            }
        }
    }

    /// Checks if the end of the source array has been reached
    fn is_at_end(&self) -> bool {
        self.loc.current >= self.source.len()
    }

    /// Advances to next character in source array
    fn advance(&mut self) -> char {
        self.loc.current += 1;
        self.source[self.loc.current - 1]
    }

    /// Adds token to source array
    fn add_token(&mut self, token_type: TokenType, literal: Option<Value>) {
        // Substring of source
        let text: String = self.source[self.loc.start..self.loc.current].iter().collect();
        let literal = literal.unwrap_or(Value::None());
        
        self.tokens.push(Token::new(token_type, text, literal, self.loc.line));
    }

    /// Matches current char to expected char
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.loc.current] != expected {
            return false;
        }

        self.loc.current += 1;
        true
    }

    /// Peeks at current character
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }

        return self.source[self.loc.current];
    }

    /// Parses and consumes a string from the array
    fn consume_string(&mut self) {
        // Capture string contents
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.loc.line += 1;
            }

            self.advance();
        }

        // Unterminated string case
        if self.is_at_end() {
            throw(self.loc.line.to_string(), "Unterminated string.");
            return;
        }

        // Consume closing quote
        self.advance();

        // Trim surrounding quotes
        let value: String = self.get_token_string(self.loc.start + 1, self.loc.current - 1);
        self.add_token(Str, Some(Value::Str(value)));
    }

    /// Parses and consumes a number from the array
    fn consume_number(&mut self) {
        while self.peek().is_digit(DIGIT_RADIX) {
            self.advance();
        }

        // Float case
        if self.peek() == '.' { // Trailing decimal points are allowed in this version of Lox
            // Consume the digit
            self.advance();

            while self.peek().is_digit(DIGIT_RADIX) {
                self.advance();
            }

            // Cast to float
            let float: Result<f64, ParseFloatError> = self.get_token_string(self.loc.start, self.loc.current).parse::<f64>();
            let float: f64 = float.unwrap_or_else(|_| {
                throw(self.loc.line.to_string(), "Float larger than 1.7976931348623157E+308");
                0.0
            });

            return self.add_token(Float, Some(Value::Float(float)));
        }

        // Int case
        let int: Result<i64, ParseIntError> = self.get_token_string(self.loc.start, self.loc.current).parse::<i64>();
        let int: i64 = int.unwrap_or_else(|_| {
            throw(self.loc.line.to_string(), "Integer larger than 9,223,372,036,854,775,807");
            0
        });

        return self.add_token(Float, Some(Value::Int(int)));
    }

    /// Parses and consumes an identifier from the array
    fn consume_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        self.add_token(self.match_keyword(), None);
    }

    /// Returns a string corresponding to the given range
    fn get_token_string(&self, start: usize, current: usize) -> String {
        self.source[start..current].iter().collect()
    }

    fn match_keyword(&self) -> TokenType {
        let keyword: &str = &self.get_token_string(self.loc.start, self.loc.current)[..];

        match keyword {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => Identifier
        }
    }
}

#[derive(Copy, Clone)]
struct Location {
    pub start: usize,
    pub current: usize,
    pub line: usize
}

impl Location {
    fn new() -> Self {
        Self { start: 0, current: 0, line: 1 }
    }
}

const DIGIT_RADIX: u32 = 10;