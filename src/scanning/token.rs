use std::fmt;
use crate::scanning::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenValue,
    pub line: usize,
    pub has_literal: bool
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: TokenValue, line: usize) -> Token {
        let has_literal: bool = literal != TokenValue::None();
        Token { token_type, lexeme, literal, line, has_literal }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Identifier(String),
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Nil(),
    None()
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Identifier(name) => write!(f, "{name}"),
            Self::Str(string) => write!(f, "{string}"),
            Self::Int(value) => write!(f, "{value}"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Bool(bool) => write!(f, "{bool}"),
            Self::Nil() => write!(f, ""),
            Self::None() => write!(f, "")
        }
    }
}