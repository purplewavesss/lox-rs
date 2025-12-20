use std::{collections::HashMap};
use crate::{LoxError, scanning::token::{Token, Value}};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), enclosing: None }
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
            match &self.enclosing {
                None => {
                    let token_name: String = name.lexeme.clone();
                    Err(LoxError::NameError(name.lexeme, format!("Undefined variable {}.", token_name)))
                }

                Some(env) => env.get(name)
            }
        }
    }

    /// Reassigns an existing environment binding.
    pub fn assign(&mut self, name: Token, value: &Value) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.clone());
            Ok(())
        }

        else {
            match self.enclosing {
                None => {
                    let token_name: String = name.lexeme.clone();
                    Err(LoxError::NameError(name.lexeme, format!("Undefined variable {}.", token_name)))
                }

                Some(ref mut env) => env.assign(name, value)
            }
        }
    }
}