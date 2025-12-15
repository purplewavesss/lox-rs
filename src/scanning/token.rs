use std::fmt;
use crate::scanning::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    pub lexeme: String,
    literal: TokenValue,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: TokenValue, line: usize) -> Token {
        Token { token_type, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Clone)]
pub enum TokenValue {
    Identifier(String),
    Str(String),
    Int(i64),
    Float(f64),
    None()
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(name) => write!(f, "{name}"),
            Self::Str(string) => write!(f, "{string}"),
            Self::Int(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::None() => write!(f, "")
        }
    }
}