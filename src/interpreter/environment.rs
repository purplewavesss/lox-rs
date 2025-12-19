use std::{collections::HashMap};
use crate::{LoxError, scanning::token::{Token, Value}};

pub struct Environment {
    values: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    /// Defines an environment binding
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    /// Retrieves an envrionment binding
    pub fn get(&self, name: Token) -> Result<Value, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values[&name.lexeme].clone())
        }

        else {
            let token_name: String = name.lexeme;
            Err(LoxError::NameError(token_name.clone(), format!("Undefined variable {}.", token_name)))
        }
    }

    /// Reassigns an existing environment binding.
    pub fn assign(&mut self, name: Token, value: &Value) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.clone());
            Ok(())
        }

        else {
            let token_name: String = name.lexeme.clone();
            Err(LoxError::NameError(name.lexeme, format!("Undefined variable {}.", token_name)))
        }
    }
}