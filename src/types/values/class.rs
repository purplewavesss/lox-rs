use std::fmt::{self, Display};
use crate::types::{token::Token, token_type::TokenType, values::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct LoxClass {
    pub name: String
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name_token(&self) -> Token {
        Token::new(TokenType::Identifier, self.name.clone(), Value::Nil(), 0)
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}