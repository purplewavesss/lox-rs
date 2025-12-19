use crate::LoxError;
use crate::expr::Expr;
use crate::interpreter::environment::Environment;
use crate::scanning::token::Value;
use crate::scanning::token_type::TokenType::*;
use crate::statement::Statement;

macro_rules! calculate {
    ($left:ident, $right:ident, $op:tt) => {
        match $left {
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Int(l_value $op r_value)),
                Value::Float(r_value) => Ok(Value::Float(l_value as f64 $op r_value)),
                Value::Identifier(_) => todo!(),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Float(l_value $op r_value as f64 )),
                Value::Float(r_value) => Ok(Value::Float(l_value $op r_value)),
                Value::Identifier(_) => todo!(),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Identifier(_) => todo!(),
            _ => Err(LoxError::ValueError($left, String::from("Not a number.")))
        }
    }
}

macro_rules! compare {
    ($left:ident, $right:ident, $op:tt) => {
        match $left {
            Value::Identifier(_) => todo!(),
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value)),
                    Value::Float(r_value) =>  {
                        let l_value = l_value as f64;
                        Ok(Value::Bool(l_value $op r_value))
                    }
                    Value::Identifier(_) => todo!(),
                    _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Float(l_value) => match $right {
                    Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value as f64)),
                    Value::Float(r_value) => Ok(Value::Bool(l_value $op r_value)),
                    Value::Identifier(_) => todo!(),
                    _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::None() => {
                Err(LoxError::ValueError($left, String::from("You cannot compare a value to a keyword.")))
            },
            _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
        }
    }
}

macro_rules! equal {
    ($left:ident, $right:ident, $op:tt) => {
        match $left {
            Value::Identifier(_) => todo!(),
            Value::Str(l_value) => {
                if let Value::Str(r_value) = $right {
                    return Ok(Value::Bool(l_value $op r_value))
                }

                Ok(Value::Bool(false))
            },
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Float(r_value) => Ok(Value::Bool(l_value as f64 $op r_value)),
                Value::Identifier(_) => todo!(),
                _ => Ok(Value::Bool(false))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value as f64)),
                Value::Float(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Identifier(_) => todo!(),
                _ => Ok(Value::Bool(false))
            },
            Value::Bool(l_value) => {
                if let Value::Bool(r_value) = $right {
                    return Ok(Value::Bool(l_value $op r_value))
                }

                Ok(Value::Bool(false))
            },
            Value::Nil() => {
                if matches!($right, Value::Nil()) {
                    return Ok(Value::Bool($left $op $right))
                }

                Ok(Value::Bool(false))
            }
            Value::None() => {
                Err(LoxError::ValueError($left, String::from("You cannot compare a value to a keyword.")))
            }
        }
    }
}

pub fn interpret(program: Vec<Statement>, env: &mut Environment) -> Result<(), LoxError> {
    for stmt in program {
        match stmt {
            Statement::Expression(expr) => {
                interpret_expr(expr, env)?;
                // Rust's rules are so funny sometimes
                ()
            }
            Statement::Print(print_expr) => println!("{}", interpret_expr(print_expr, env)?),
            Statement::Var(name, identifier) => interpret_declaration(Statement::Var(name, identifier), env)?
        };
    }

    Ok(())
}

fn interpret_expr(ast: Expr, env: &mut Environment) -> Result<Value, LoxError> {
    match ast {
        Expr::Literal(value) => Ok(value),
        Expr::Grouping(expr) => interpret_expr(*expr, env),
        Expr::Unary(op, expr) => {
            let error_expr = expr.clone();
            let expr_value: Value = interpret_expr(*expr, env)?;

            match op.token_type {
                Minus => {
                    match expr_value {
                        Value::Float(n) => Ok(Value::Float(-n)),
                        Value::Int(n) => Ok(Value::Int(-n)),
                        Value::Identifier(name) => interpret_expr(Expr::Unary(op, Box::new(Expr::Literal(env.get(*name)?))), env), // Recurses with a new expression containing the literal instead of the identifier
                        _ => Err(LoxError::RuntimeError(*error_expr, String::from("Negation operator cannot be used here.")))
                    }
                }

                Not => {
                    match expr_value {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        Value::Identifier(name) => interpret_expr(Expr::Unary(op, Box::new(Expr::Literal(env.get(*name)?))), env),
                        _ => Err(LoxError::RuntimeError(*error_expr, String::from("Not operator cannot be used here.")))
                    }
                }

                _ => Err(LoxError::CompilerBug(Statement::Expression(*error_expr), format!("Unary operator was created with invalid token type {}", op.token_type)))
            }
        },
        Expr::Binary(left, op, right) => {
            let left: Value = interpret_expr(*left, env)?;
            let right: Value = interpret_expr(*right, env)?;
            
            match op.token_type {
                Minus => calculate!(left, right, -),
                Plus => match left {
                    Value::Int(l_value) => match right {
                        Value::Int(r_value) => Ok(Value::Int(l_value + r_value)),
                        Value::Float(r_value) => Ok(Value::Float(l_value as f64 + r_value)),
                        Value::Identifier(name) => interpret_expr(Expr::Binary(Box::new(Expr::Literal(left)), op, Box::new(Expr::Literal(env.get(*name)?))), env),
                        _ => Err(LoxError::ValueError(right, String::from("Not a number.")))
                    },
                    Value::Float(l_value) => match right {
                        Value::Int(r_value) => Ok(Value::Float(l_value + r_value as f64 )),
                        Value::Float(r_value) => Ok(Value::Float(l_value + r_value)),
                        Value::Identifier(name) => interpret_expr(Expr::Binary(Box::new(Expr::Literal(left)), op, Box::new(Expr::Literal(env.get(*name)?))), env),
                     _ => Err(LoxError::ValueError(right, String::from("Not a number.")))
                    },
                    Value::Identifier(name) => interpret_expr(Expr::Binary(Box::new(Expr::Literal(env.get(*name)?)), op, Box::new(Expr::Literal(right))), env),
                    Value::Str(l_value) => {
                        match right {
                            Value::Str(r_value) => Ok(Value::Str(l_value + &r_value)),
                            _ => Err(LoxError::ValueError(right, String::from("Not a string.")))
                        }
                    }
                    _ => Err(LoxError::ValueError(left, String::from("Not a number.")))
                },
                Slash => calculate!(left, right, /),
                Asterisk => calculate!(left, right, *),
                Mod => calculate!(left, right, %),
                EqualEqual => equal!(left, right, ==),
                Greater => compare!(left, right, >),
                GreaterEqual => compare!(left, right, >=),
                Less => compare!(left, right, <),
                NotEqual => equal!(left, right, !=),
                LessEqual => compare!(left, right, <=),
                _ => Err(LoxError::ValueError(left, String::from("Does not have an interpretable value.")))
            }
        },
        Expr::Variable(name) => env.get(name),
        Expr::Assign(name, exp) => {
            let value: Value = interpret_expr(*exp, env)?;
            env.assign(name, &value)?;
            Ok(value)
        }
    }
}

fn interpret_declaration(dec: Statement, env: &mut Environment) -> Result<(), LoxError> {
    if let Statement::Var(name, identifier) = dec {
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

    else {
        Err(LoxError::CompilerBug(dec, String::from("Non-identifier value passed in.")))
    }
}