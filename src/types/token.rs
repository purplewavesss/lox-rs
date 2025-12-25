use std::fmt;
use crate::types::{token_type::TokenType, values::value::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Value,
    pub line: usize,
    pub has_literal: bool
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Value, line: usize) -> Token {
        let has_literal: bool = literal != Value::None();
        Token { token_type, lexeme, literal, line, has_literal }
    }

    pub fn from_value(name: &str, literal: Value, line: usize) -> Self {
        Self {
            token_type: TokenType::Identifier,
            lexeme: String::from(name),
            literal,
            line,
            has_literal: true
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}