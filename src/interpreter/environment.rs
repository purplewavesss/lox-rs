use std::{collections::{HashMap, HashSet}, ops};
use crate::{LoxError, interpreter::stdlib, types::{token::Token, values::Value}};
use crate::types::expr::Expr;
use crate::types::statement::Statement;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    locals: HashMap<String, Value>,
    globals: HashMap<String, Value>,
    assignments: HashMap<String, Value>,
    declared_here_in_block: HashSet<String>,
    is_block_env: bool
}

impl Environment {
    pub fn new() -> Self {
        Self { 
            locals: HashMap::new(),
            globals: stdlib::get_stdlib(),
            assignments: HashMap::new(),
            declared_here_in_block: HashSet::new(),
            is_block_env: false
        }
    }

    pub fn from(env: Environment) -> Self {
        Self { 
            locals: env.locals,
            globals: env.globals,
            assignments: env.assignments,
            declared_here_in_block: env.declared_here_in_block,
            is_block_env: env.is_block_env
        }
    }

    /// Constructs a new environment from a vector of tokens and values. This does not perform arity checks: it simply returns when a vector is drained.
    pub fn build(names: &Vec<Token>, values: &Vec<Value>) -> Self {
        let mut build_values: HashMap<String, Value> = HashMap::new();
        let mut names = names.iter();
        let mut values = values.iter();

        loop {
            let next_name: Option<&Token> = names.next();
            let next_value: Option<&Value> = values.next();

            if next_name.is_none() || next_value.is_none() {
                break;
            }

            let next_name: Token = next_name.unwrap().clone();
            let next_value: Value = next_value.unwrap().clone();
            build_values.insert(next_name.lexeme, next_value);
        }

        Self {
            locals: build_values,
            globals: HashMap::new(),
            assignments: HashMap::new(),
            declared_here_in_block: HashSet::new(),
            is_block_env: false
        }
    }

    /// Creates an environment from an existing environment, with assignments wiped.
    pub fn get_block_env(env: &Environment) -> Self {
        Self {
            locals: env.locals.clone(),
            globals: env.globals.clone(),
            assignments: HashMap::new(),
            declared_here_in_block: HashSet::new(),
            is_block_env: true
        }
    }

    /// Adds the globals from another environment.
    pub fn add_globals(&mut self, other: &Environment) {
        self.globals.extend(other.globals.clone());
    }

    /// Defines a local environment binding
    pub fn define_local(&mut self, name: String, value: Value) {
        if self.is_block_env {
            self.declared_here_in_block.insert(name.clone());
        }
        
        self.locals.insert(name, value);
    }

    /// Defines a global (exists in all function calls) environment binding
    pub fn define_global(&mut self, name: String, value: Value) -> Result<(), LoxError> {
        if self.is_block_env {
            Err(LoxError::CompilerBug(Statement::Expression(Expr::Literal(Value::Str(name))),
                String::from("Globals cannot be defined in local scope.")))
        }
        
        else {
            self.globals.insert(name.clone(), value);
            Ok(())
        }
    }
    
    /// Retrieves an envrionment binding
    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        if self.locals.contains_key(&name.lexeme) {
            Ok(self.locals[&name.lexeme].clone())
        }
        
        else if self.globals.contains_key(&name.lexeme) {
            Ok(self.globals[&name.lexeme].clone())
        }

        else {
            let token_name: String = name.lexeme.clone();
            Err(LoxError::NameError(token_name, String::from("Undefined variable.")))
        }
    }

    /// Reassigns an existing environment binding.
    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), LoxError> {
        // Locals case
        if self.locals.contains_key(&name.lexeme) {
            if self.is_block_env && !self.declared_here_in_block.contains(&name.lexeme) {
                self.assignments.insert(name.lexeme.clone(), value.clone());
            }

            self.locals.insert(name.lexeme, value);

            Ok(())
        }

        // Globals case
        else if self.globals.contains_key(&name.lexeme) {
            // Shadow the value in locals if declared in block
            if self.is_block_env {
                self.locals.insert(name.lexeme, value);
                Ok(())
            }

            else {
                self.globals.insert(name.lexeme, value);
                Ok(())
            }
        }

        else {
            let token_name: String = name.lexeme.clone();
            Err(LoxError::NameError(name.lexeme, format!("Undefined variable {}.", token_name)))
        }
    }

    /// Adds assignments from block environments to current environment
    pub fn add_assignments(&mut self, block_env: &mut Environment) {
        for (name, value) in block_env.assignments.drain() {
            // Propagate values forward if this is a block environment
            if self.is_block_env && !self.declared_here_in_block.contains(&name) {
                self.assignments.insert(name.clone(), value.clone());
            }

            self.locals.insert(name, value);
        }
    }

    /// Returns whether an environment is defined in global or local scope
    pub fn is_global(&self) -> bool {
        !self.is_block_env
    }
}

impl ops::Add<Environment> for Environment {
    type Output = Environment;

    fn add(mut self, _rhs: Environment) -> Environment {
        self.locals.extend(_rhs.locals);
        self.globals.extend(_rhs.globals);
        self
    }
}