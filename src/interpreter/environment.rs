use std::collections::{HashMap, HashSet};
use crate::{LoxError, scanning::token::{Token, Value}};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    assignments: HashMap<String, Value>,
    declared_here: HashSet<String>,
    is_block_env: bool
}

impl Environment {
    pub fn new() -> Self {
        Self { 
            values: HashMap::new(),
            assignments: HashMap::new(),
            declared_here: HashSet::new(),
            is_block_env: false
        }
    }

    /// Creates an environment from an existing environment, with assignments wiped.
    pub fn get_block_env(env: &Environment) -> Self {
        Self {
            values: env.values.clone(),
            assignments: HashMap::new(),
            declared_here: HashSet::new(),
            is_block_env: true
        }
    }

    /// Defines an environment binding
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name.clone(), value);
        self.declared_here.insert(name);
    }
    
    /// Retrieves an envrionment binding
    pub fn get(&self, name: Token) -> Result<Value, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values[&name.lexeme].clone())
        }

        else {
            let token_name: String = name.lexeme.clone();
            Err(LoxError::NameError(name.lexeme, format!("Undefined variable {}.", token_name)))
        }
    }

    /// Reassigns an existing environment binding.
    pub fn assign(&mut self, name: Token, value: &Value) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());

            if self.is_block_env && !self.declared_here.contains(&name.lexeme) {
                self.assignments.insert(name.lexeme, value.clone());
            }

            Ok(())
        }

        else {
            let token_name: String = name.lexeme.clone();
            Err(LoxError::NameError(name.lexeme, format!("Undefined variable {}.", token_name)))
        }
    }

    pub fn add_assignments(&mut self, block_env: &mut Environment) {
        for (name, value) in block_env.assignments.drain() {
            // Propagate values forward if this is a block environment
            if self.is_block_env && !self.declared_here.contains(&name) {
                self.assignments.insert(name.clone(), value.clone());
            }

            self.values.insert(name, value);
        }
    }
}