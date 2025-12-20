use std::{collections::HashMap};
use crate::{LoxError, scanning::token::{Token, Value}};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
    assignments: Vec<String>,
    is_block_env: bool
}

impl Environment {
    pub fn new() -> Self {
        Self { 
            values: HashMap::new(), 
            enclosing: None, 
            assignments: Vec::new(), 
            is_block_env: false
        }
    }

    /// Creates an environment from an existing environment, with assignments wiped.
    pub fn get_block_env(env: &Self) -> Self {
        let mut env: Self = env.clone();
        env.assignments = Vec::new();
        env.is_block_env = true;
        env
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
            self.values.insert(name.lexeme.clone(), value.clone());
            
            if self.is_block_env {
                self.assignments.push(name.lexeme);
            }

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

    pub fn add_assignments(&mut self, block_env: Environment) {
        for assignment in block_env.assignments {
            if self.values.contains_key(&assignment) {
                let block_assignment: Value = block_env.values.get(&assignment).unwrap().clone();
                self.values.insert(assignment, block_assignment);
            }
        }
    }
}