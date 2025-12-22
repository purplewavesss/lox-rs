use crate::{LoxError, interpreter::{environment::Environment, interpret::interpret}, types::{expr::Expr, statement::Statement, token::Token, value::Value}};

// Callables have to derive PartialEq because they are inside of the Value enum. In practice, they should not be compared to each other anywhere in the code.
#[derive(Clone, Debug, PartialEq)]
pub enum LoxCallable {
    Native(fn(Vec<Value>) -> Result<Value, LoxError>, usize),
    Closure(Vec<Token>, Box<Vec<Statement>>, Environment)
}

impl LoxCallable {
    /// Borrows and calls the callable's inner function.
    pub fn call(self, arg_values: Vec<Value>) -> Result<Value, LoxError> {
        match self {
            LoxCallable::Native(func, _) => func(arg_values),
            LoxCallable::Closure(arg_names, body, env) => {
                let args_env = Environment::build(&arg_names, &arg_values);
                let mut env = env + args_env;
                interpret(*body, &mut env)
            }
        }
    }

    pub fn check_arity(&self, other: &Vec<Expr>) -> bool {
        match self {
            Self::Native(_, arity) => *arity == other.len(),
            Self::Closure(tokens, _, _) => tokens.len() == other.len()
        }
    }
}