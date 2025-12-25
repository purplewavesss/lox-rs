use std::fmt::{self, Display};
use crate::{LoxError, interpreter::{environment::Environment, interpret::interpret}, types::{expr::Expr, statement::Statement, token::Token, values::value::Value}};
use crate::types::token_type::TokenType;

// Callables have to derive PartialEq because they are inside of the Value enum. In practice, they should not be compared to each other anywhere in the code.
#[derive(Clone, Debug, PartialEq)]
pub enum LoxCallable {
    Native(String, fn(Vec<Value>) -> Result<Value, LoxError>, usize),
    Closure(String, Vec<Token>, Box<Vec<Statement>>, Environment)
}

impl LoxCallable {
    /// Borrows and calls the callable's inner function.
    pub fn call(self, arg_values: Vec<Value>, globals_env: &Environment) -> Result<Value, LoxError> {
        match self {
            LoxCallable::Native(_, func, _) => func(arg_values),
            LoxCallable::Closure(_, arg_names, body, env) => {
                let args_env = Environment::build(&arg_names, &arg_values);
                let mut env = env + args_env;
                env.add_globals(globals_env);
                interpret(*body, &mut env)
            }
        }
    }

    pub fn check_arity(&self, other: &Vec<Expr>) -> bool {
        match self {
            Self::Native(_, _, arity) => *arity == other.len(),
            Self::Closure(_, tokens, _, _) => tokens.len() == other.len()
        }
    }
    
    pub fn get_name(&self) -> Token {
        match self {
            Self::Native(name, _, _) => Token::new(TokenType::Identifier, name.clone(), Value::Nil(), 0),
            Self::Closure(name, _, _, _) => Token::new(TokenType::Identifier, name.clone(), Value::Nil(), 0)
        }
    }
}

impl Display for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Native(name, _, _) => write!(f, "{name}"),
            Self::Closure(name, _, _, _) => write!(f, "{name}")
        }
    }
}