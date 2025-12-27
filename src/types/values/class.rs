use std::{collections::HashMap, fmt::{self, Display}};
use crate::types::{token::Token, token_type::TokenType, values::{Value, callable::LoxCallable}};

#[derive(Clone, Debug, PartialEq)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LoxCallable>
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxCallable>) -> Self {
        Self { name, methods }
    }

    pub fn find_method(&self, name: &String) -> Option<&LoxCallable> {
        self.methods.get(name)
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