use std::{collections::HashMap, fmt::{self, Display}, rc::Rc};
use crate::{interpreter::environment::Environment, types::{token::Token, token_type::TokenType, values::{Value, callable::LoxCallable}}};

#[derive(Clone, Debug, PartialEq)]
pub struct LoxClass {
    name: String,
    superclass: Option<Rc<LoxClass>>,
    methods: HashMap<String, LoxCallable>,
    env: Environment
}

impl LoxClass {
    pub fn new(name: String, superclass: Option<Rc<LoxClass>>, methods: HashMap<String, LoxCallable>, env: Environment) -> Self {
        Self { name, superclass, methods, env }
    }

    pub fn find_method(&self, name: &String) -> Option<LoxCallable> {
        match self.methods.get(name) {
            Some(method) => Some(method.clone()),
            None => {
                match &self.superclass {
                    Some(class) => class.find_method(name),
                    None => None
                }
            }
        }
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