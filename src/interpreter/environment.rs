use std::{collections::HashMap};
use crate::{LoxError, scanning::token::{Token, Value}};

pub struct Environment {
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values[&name.lexeme].clone())
        }

        else {
            let token_name: String = name.lexeme;
            Err(LoxError::NameError(token_name.clone(), format!("Undefined variable {}.", token_name)))
        }
    }
}