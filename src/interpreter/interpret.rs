use crate::LoxError;
use crate::interpreter::environment::Environment;
use crate::types::{{expr::Expr, statement::Statement, value::Value, token::Token, token_type::TokenType::*}};

/// Generates codes for calculations
macro_rules! calculate {
    ($left:ident, $op_token:ident, $right:ident, $env:ident, $op:tt) => {
        match $left {
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Int(l_value $op r_value)),
                Value::Float(r_value) => Ok(Value::Float(l_value as f64 $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(*name)?), $env),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Float(l_value $op r_value as f64 )),
                Value::Float(r_value) => Ok(Value::Float(l_value $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(*name)?), $env),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Identifier(name) => interpret_expr(build_binary_expr($env.get(*name)?, $op_token, $right), $env),
            _ => Err(LoxError::ValueError($left, String::from("Not a number.")))
        }
    }
}

/// Generates code for comparisons
macro_rules! compare {
    ($left:ident, $op_token:ident, $right:ident, $env:ident, $op:tt) => {
        match $left {
            Value::Identifier(name) => interpret_expr(build_binary_expr($env.get(*name)?, $op_token, $right), $env),
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Float(r_value) =>  {
                    let l_value = l_value as f64;
                    Ok(Value::Bool(l_value $op r_value))
                }
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(*name)?), $env),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value as f64)),
                Value::Float(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(*name)?), $env),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::None() => {
                Err(LoxError::ValueError($left, String::from("You cannot compare a value to a keyword.")))
            },
            _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
        }
    }
}

/// Generates code for determining equality
macro_rules! equal {
    ($left:ident, $op_token:ident, $right:ident, $env:ident, $op:tt) => {
        match $left {
            Value::Identifier(name) => interpret_expr(build_binary_expr($env.get(*name)?, $op_token, $right), $env),
            Value::Str(l_value) => {
                if let Value::Str(r_value) = $right {
                    return Ok(Value::Bool(l_value $op r_value))
                }

                Ok(Value::Bool(false))
            },
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Float(r_value) => Ok(Value::Bool(l_value as f64 $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(*name)?), $env),
                _ => Ok(Value::Bool(false))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value as f64)),
                Value::Float(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(*name)?), $env),
                _ => Ok(Value::Bool(false))
            },
            Value::Bool(l_value) => {
                if let Value::Bool(r_value) = $right {
                    return Ok(Value::Bool(l_value $op r_value))
                }

                Ok(Value::Bool(false))
            },
            Value::Nil() => Ok(Value::Bool($left $op $right)),
            Value::None() => {
                Err(LoxError::ValueError($left, String::from("You cannot compare a value to a keyword.")))
            }
        }
    }
}

/// Interprets a list of statements
pub fn interpret(program: Vec<Statement>, env: &mut Environment) -> Result<(), LoxError> {
    for stmt in program {
        interpret_statement(stmt, env)?
    }

    Ok(())
}

fn interpret_statement(stmt: Statement, env: &mut Environment) -> Result<(), LoxError> {
    match stmt {
        Statement::Block(statements) => Ok(interpret_block(statements, env)?),
        Statement::Expression(expr) => {
            Ok(_ = interpret_expr(expr, env)?)
        },
        Statement::If(cond, then, els) => Ok(interpret_if(cond, then, els, env)?),
        Statement::Print(print_expr) => Ok(println!("{}", interpret_expr(print_expr, env)?)),
        Statement::Var(name, identifier) => Ok(interpret_declaration(name, identifier, env)?),
        Statement::While(cond, body) => Ok(interpret_while(cond, *body, env)?)
    }
}

fn interpret_block(statements: Box<Vec<Statement>>, env: &mut Environment) -> Result<(), LoxError> {
    let mut block_env: Environment = Environment::get_block_env(env);

    interpret(*statements, &mut block_env)?;

    // Add new assignments to current environment
    Ok(env.add_assignments(&mut block_env))
}

fn interpret_expr(ast: Expr, env: &mut Environment) -> Result<Value, LoxError> {
    match ast {
        Expr::Literal(value) => Ok(value),
        Expr::Grouping(expr) => interpret_expr(*expr, env),
        Expr::Unary(op, expr) => {
            let expr_value: Value = interpret_expr(*expr, env)?;

            match op.token_type {
                Minus => {
                    match expr_value {
                        Value::Float(n) => Ok(Value::Float(-n)),
                        Value::Int(n) => Ok(Value::Int(-n)),
                        Value::Identifier(name) => interpret_expr(Expr::Unary(op, Box::new(Expr::Literal(env.get(*name)?))), env), // Recurses with a new expression containing the literal instead of the identifier
                        _ => Err(LoxError::ValueError(expr_value, String::from("Negation operator cannot be used here.")))
                    }
                }

                Not => {
                    match expr_value {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        Value::Identifier(name) => interpret_expr(Expr::Unary(op, Box::new(Expr::Literal(env.get(*name)?))), env),
                        _ => Err(LoxError::ValueError(expr_value, String::from("Not operator cannot be used here.")))
                    }
                }

                _ => Err(LoxError::CompilerBug(Statement::Expression(Expr::Literal(expr_value)), format!("Unary operator was created with invalid token type {}", op.token_type)))
            }
        },
        Expr::Binary(left, op, right) => {
            let left: Value = interpret_expr(*left, env)?;
            let right: Value = interpret_expr(*right, env)?;

            match op.token_type {
                Minus => calculate!(left, op, right, env, -),
                // Plus does not use the calculate macro, as it uses
                Plus => match left {
                    Value::Int(l_value) => match right {
                        Value::Int(r_value) => Ok(Value::Int(l_value + r_value)),
                        Value::Float(r_value) => Ok(Value::Float(l_value as f64 + r_value)),
                        Value::Identifier(name) => interpret_expr(build_binary_expr(left, op, env.get(*name)?), env),
                        _ => Err(LoxError::ValueError(right, String::from("Not a number.")))
                    },
                    Value::Float(l_value) => match right {
                        Value::Int(r_value) => Ok(Value::Float(l_value + r_value as f64 )),
                        Value::Float(r_value) => Ok(Value::Float(l_value + r_value)),
                        Value::Identifier(name) => interpret_expr(build_binary_expr(left, op, env.get(*name)?), env),
                        _ => Err(LoxError::ValueError(right, String::from("Not a number.")))
                    },
                    Value::Identifier(name) => interpret_expr(build_binary_expr(env.get(*name)?, op, right), env),
                    Value::Str(l_value) => {
                        match right {
                            Value::Str(r_value) => Ok(Value::Str(l_value + &r_value)),
                            _ => Err(LoxError::ValueError(right, String::from("Not a string.")))
                        }
                    }
                    _ => Err(LoxError::ValueError(left, String::from("Not a number.")))
                },
                Slash => calculate!(left, op, right, env, /),
                Asterisk => calculate!(left, op, right, env, *),
                Mod => calculate!(left, op, right, env, %),
                EqualEqual => equal!(left, op, right, env, ==),
                Greater => compare!(left, op, right, env, >),
                GreaterEqual => compare!(left, op, right, env, >=),
                Less => compare!(left, op, right, env, <),
                NotEqual => equal!(left, op, right, env, !=),
                LessEqual => compare!(left, op, right, env, <=),
                _ => Err(LoxError::ValueError(left, String::from("Does not have an interpretable value.")))
            }
        },
        Expr::Logical(left, op, right) => {
            match op.token_type {
                And => {
                    let left_value: Value = interpret_expr(*left, env)?;
                    let right_value: Value = interpret_expr(*right, env)?;
                    Ok(Value::Bool(get_value_truth(left_value, env)? && get_value_truth(right_value, env)?))
                },
                Or => {
                    let left_value: Value = interpret_expr(*left, env)?;
                    let right_value: Value = interpret_expr(*right, env)?;
                    Ok(Value::Bool(get_value_truth(left_value, env)? || get_value_truth(right_value, env)?))
                },
                _ => Err(LoxError::CompilerBug(Statement::Expression(Expr::Logical(left.clone(), op, right.clone())),
                                               String::from("Invalid operator used in Expr::Logical")))
            }
        }
        Expr::Variable(name) => env.get(name),
        Expr::Assign(name, exp) => {
            let value: Value = interpret_expr(*exp, env)?;
            env.assign(name, &value)?;
            Ok(value)
        }
    }
}

fn interpret_if(cond: Expr, then: Box<Statement>, els: Box<Option<Statement>>, env: &mut Environment) -> Result<(), LoxError> {
    let cond_value: Value = interpret_expr(cond, env)?;

    if get_value_truth(cond_value, env)? {
        interpret_statement(*then, env)
    }

    else {
        match *els {
            None => Ok(()),
            Some(else_case) => interpret_statement(else_case, env)
        }
    }
}

fn interpret_declaration(name: Token, identifier: Option<Expr>, env: &mut Environment) -> Result<(), LoxError> {
    match identifier {
        None => Ok(env.define(name.lexeme, Value::Nil())),
        Some(exp) => {
            match interpret_expr(exp, env) {
                Ok(value) => Ok(env.define(name.lexeme, value)),
                Err(error) => Err(error)
            }
        }
    }
}

fn interpret_while(condition: Expr, body: Statement, env: &mut Environment) -> Result<(), LoxError> {
    while get_value_truth(interpret_expr(condition.clone(), env)?, env)? {
        interpret_statement(body.clone(), env)?;
    }

    Ok(())
}

fn get_value_truth(value: Value, env: &Environment) -> Result<bool, LoxError> {
    match value {
        Value::Bool(truth) => Ok(truth),
        Value::Identifier(token) => get_value_truth(env.get(*token)?, env),
        Value::Nil() => Ok(false),
        _ => Err(LoxError::ValueError(value, String::from("Value has no truthiness!")))
    }
}

fn build_binary_expr(left: Value, op: Token, right: Value) -> Expr {
    Expr::Binary(Box::new(Expr::Literal(left)),
                 op,
                 Box::new(Expr::Literal(right)))
}
