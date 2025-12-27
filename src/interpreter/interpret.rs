use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::LoxError;
use crate::interpreter::environment::Environment;
use crate::types::values::object::LoxObject;
use crate::types::values::{callable::LoxCallable, Value, class::LoxClass};
use crate::types::{{expr::Expr, statement::Statement, token::Token, token_type::TokenType::*}};

/// Generates codes for calculations
macro_rules! calculate {
    ($left:ident, $op_token:ident, $right:ident, $env:ident, $op:tt) => {
        match $left {
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Int(l_value $op r_value)),
                Value::Float(r_value) => Ok(Value::Float(l_value as f64 $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(&*name)?), $env),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Float(l_value $op r_value as f64 )),
                Value::Float(r_value) => Ok(Value::Float(l_value $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(&*name)?), $env),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Identifier(name) => interpret_expr(build_binary_expr($env.get(&*name)?, $op_token, $right), $env),
            _ => Err(LoxError::ValueError($left, String::from("Not a number.")))
        }
    }
}

/// Generates code for comparisons
macro_rules! compare {
    ($left:ident, $op_token:ident, $right:ident, $env:ident, $op:tt) => {
        match $left {
            Value::Identifier(name) => interpret_expr(build_binary_expr($env.get(&*name)?, $op_token, $right), $env),
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Float(r_value) =>  {
                    let l_value = l_value as f64;
                    Ok(Value::Bool(l_value $op r_value))
                }
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(&*name)?), $env),
                _ => Err(LoxError::ValueError($right, String::from("Not a number.")))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value as f64)),
                Value::Float(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(&*name)?), $env),
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
            Value::Identifier(name) => interpret_expr(build_binary_expr($env.get(&*name)?, $op_token, $right), $env),
            Value::Str(l_value) => {
                if let Value::Str(r_value) = $right {
                    return Ok(Value::Bool(l_value $op r_value))
                }

                Ok(Value::Bool(false))
            },
            Value::Int(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Float(r_value) => Ok(Value::Bool(l_value as f64 $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(&*name)?), $env),
                _ => Ok(Value::Bool(false))
            },
            Value::Float(l_value) => match $right {
                Value::Int(r_value) => Ok(Value::Bool(l_value $op r_value as f64)),
                Value::Float(r_value) => Ok(Value::Bool(l_value $op r_value)),
                Value::Identifier(name) => interpret_expr(build_binary_expr($left, $op_token, $env.get(&*name)?), $env),
                _ => Ok(Value::Bool(false))
            },
            Value::Bool(l_value) => {
                if let Value::Bool(r_value) = $right {
                    return Ok(Value::Bool(l_value $op r_value))
                }

                Ok(Value::Bool(false))
            },
            Value::Nil() => Ok(Value::Bool($left $op $right)),
            _ => {
                Err(LoxError::ValueError($left, String::from("Value is not comparable.")))
            }
        }
    }
}

/// Interprets a list of statements
pub fn interpret(program: Vec<Statement>, env: &mut Environment) -> Result<Value, LoxError> {
    for stmt in program {
        let result = interpret_statement(stmt, env)?;

        if result != Value::Nil() {
            return Ok(result)
        }
    }

    Ok(Value::Nil())
}

/// Matches and interprets each type of statement
fn interpret_statement(stmt: Statement, env: &mut Environment) -> Result<Value, LoxError> {
    match stmt {
        Statement::Block(statements) => interpret_block(statements, env),
        Statement::Class(name, methods) => {
            interpret_class(name, *methods, env)?;
            Ok(Value::Nil())
        },
        Statement::FunDeclaration(name, args, body) => {
            interpret_closure(name, args, body, env)?;
            Ok(Value::Nil())
        },
        Statement::Expression(expr) => {
            interpret_expr(expr, env)?;
            Ok(Value::Nil())
        },
        Statement::If(cond, then, els) => interpret_if(cond, then, els, env),
        Statement::Print(print_expr) => {
            println!("{}", interpret_expr(print_expr, env)?);
            Ok(Value::Nil())
        },
        Statement::Return(return_expr) => interpret_expr(return_expr, env),
        Statement::Var(name, identifier) => {
            interpret_declaration(name, identifier, env)?;
            Ok(Value::Nil())
        },
        Statement::While(cond, body) => interpret_while(cond, *body, env)
    }
}

// Interprets a block in a new block environment.
fn interpret_block(statements: Box<Vec<Statement>>, env: &mut Environment) -> Result<Value, LoxError> {
    let mut block_env: Environment = Environment::get_block_env(env);
    let result: Value = interpret(*statements, &mut block_env)?;

    // Add new assignments to current environment
    env.add_assignments(&mut block_env);

    Ok(result)
}

// Interprets the value of an expression.
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
                        Value::Identifier(name) => interpret_expr(Expr::Unary(op, Box::new(Expr::Literal(env.get(&*name)?))), env), // Recurses with a new expression containing the literal instead of the identifier
                        _ => Err(LoxError::ValueError(expr_value, String::from("Negation operator cannot be used here.")))
                    }
                }

                Not => {
                    match expr_value {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        Value::Identifier(name) => interpret_expr(Expr::Unary(op, Box::new(Expr::Literal(env.get(&*name)?))), env),
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
                // Plus does not use the calculate macro, as it can be used for calculation and concatenation
                Plus => match left {
                    Value::Int(l_value) => match right {
                        Value::Int(r_value) => Ok(Value::Int(l_value + r_value)),
                        Value::Float(r_value) => Ok(Value::Float(l_value as f64 + r_value)),
                        Value::Identifier(name) => interpret_expr(build_binary_expr(left, op, env.get(&*name)?), env),
                        Value::Str(r_value) => Ok(Value::Str(format!("{l_value}{r_value}"))),
                        _ => Err(LoxError::ValueError(right, String::from("Not a number.")))
                    },
                    Value::Float(l_value) => match right {
                        Value::Int(r_value) => Ok(Value::Float(l_value + r_value as f64 )),
                        Value::Float(r_value) => Ok(Value::Float(l_value + r_value)),
                        Value::Identifier(name) => interpret_expr(build_binary_expr(left, op, env.get(&*name)?), env),
                        Value::Str(r_value) => Ok(Value::Str(format!("{l_value}{r_value}"))),
                        _ => Err(LoxError::ValueError(right, String::from("Not a number.")))
                    },
                    Value::Identifier(name) => interpret_expr(build_binary_expr(env.get(&*name)?, op, right), env),
                    Value::Bool(l_value) => {
                        match right {
                            Value::Str(r_value) => Ok(Value::Str(format!("{l_value}{r_value}"))),
                            _ => Err(LoxError::ValueError(right, String::from("Not a string.")))
                        }
                    }
                    Value::Str(ref l_value) => {
                        match right {
                            Value::Int(r_value) => Ok(Value::Str(format!("{l_value}{r_value}"))),
                            Value::Float(r_value) => Ok(Value::Str(format!("{l_value}{r_value}"))),
                            Value::Identifier(name) => interpret_expr(build_binary_expr(left, op, env.get(&*name)?), env),
                            Value::Str(r_value) => Ok(Value::Str(format!("{l_value}{r_value}"))),
                            Value::Bool(r_value) => Ok(Value::Str(format!("{l_value}{r_value}"))),
                            _ => Err(LoxError::ValueError(right, String::from("Value cannot be concatenated to a string.")))
                        }
                    },
                    _ => Err(LoxError::ValueError(left, String::from("Not a number.")))
                },
                // TODO: Handle divisions by zero
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
        Expr::Variable(name) | Expr::This(name) => env.get(&name),
        Expr::Assign(name, exp) => {
            let value: Value = interpret_expr(*exp, env)?;
            env.assign(name, value)?;
            Ok(Value::Nil())
        },
        Expr::Call(name, args) => interpret_call(name, *args, env),
        Expr::Get(object, property) => {
            let object: Rc<RefCell<LoxObject>> = get_object(*object, env)?;
            let object: Ref<LoxObject> = object.borrow();
            object.get(&property)
        },
        Expr::Set(object, property, value) => {
            let object: Rc<RefCell<LoxObject>> = get_object(*object, env)?;
            let mut object: RefMut<LoxObject> = object.borrow_mut();
            object.set(property, interpret_expr(*value, env)?);
            Ok(Value::Nil())
        }
    }
}

// Interprets an if statement in a new block environment.
fn interpret_if(cond: Expr, then: Box<Statement>, els: Box<Option<Statement>>, env: &mut Environment) -> Result<Value, LoxError> {
    let cond_value: Value = interpret_expr(cond, env)?;

    if get_value_truth(cond_value, env)? {
        interpret_statement(*then, env)
    }

    else {
        match *els {
            None => Ok(Value::Nil()),
            Some(else_case) => interpret_statement(else_case, env)
        }
    }
}

// Interprets a declaration and adds it to the current environment.
fn interpret_declaration(name: Token, identifier: Option<Expr>, env: &mut Environment) -> Result<(), LoxError> {
    match identifier {
        None => Ok(env.define_local(name.lexeme, Value::Nil())),
        Some(exp) => {
            match interpret_expr(exp, env) {
                Ok(value) => Ok(env.define_local(name.lexeme, value)),
                Err(error) => Err(error)
            }
        }
    }
}


// Interprets a while loop in a block environment.
fn interpret_while(condition: Expr, body: Statement, env: &mut Environment) -> Result<Value, LoxError> {
    while get_value_truth(interpret_expr(condition.clone(), env)?, env)? {
        let result = interpret_statement(body.clone(), env)?;

        if result != Value::Nil() {
            return Ok(result)
        }
    }

    Ok(Value::Nil())
}

// Adds a closure to the environment.
fn interpret_closure(name: Token, args: Vec<Token>, body: Box<Vec<Statement>>, env: &mut Environment) -> Result<(), LoxError> {
    if env.is_global() {
        env.define_global(name.lexeme.clone(),
                                 Value::Callable(LoxCallable::Closure(name.lexeme, args, body, Environment::get_block_env(&env))))
    }

    else {
        Ok(env.define_local(name.lexeme.clone(),
                                   Value::Callable(LoxCallable::Closure(name.lexeme, args, body, Environment::get_block_env(&env)))))
    }
}

// Interprets and calls a callable
fn interpret_call(name: Box<Expr>, args: Vec<Expr>, env: &mut Environment) -> Result<Value, LoxError> {
    let called: Value = interpret_expr(*name.clone(), env)?;
    
    match called {
        Value::Callable(callable) => {
            // Interpret arguments
            if !callable.check_arity(&args) {
                return Err(LoxError::ArgumentError(Statement::Expression(Expr::Variable(callable.get_name())), String::from("Invalid arity for function.")));
            }

            let args = interpret_args(args, env)?;

            callable.call(args, env)
        },
        Value::Class(class) => {
            if args.len() != 0 {
                return Err(LoxError::ArgumentError(Statement::Expression(Expr::Variable(class.get_name_token())), String::from("Invalid arity for constructor.")));
            }

            let args = interpret_args(args, env)?;

            Ok(Value::Instance(Rc::new(
                               RefCell::new(
                               LoxObject::new(class)))))
        },
        _ => Err(LoxError::ValueError(called, String::from("Value is not callable!")))
    }
}

// Interprets a vector of function arguments.
fn interpret_args(args: Vec<Expr>, env: &mut Environment) -> Result<Vec<Value>, LoxError> {
    let mut interpreted_args: Vec<Value> = Vec::new();

    for arg in args {
        interpreted_args.push(interpret_expr(arg, env)?);
    }

    Ok(interpreted_args)
}

fn interpret_class(name: Token, methods: Vec<Statement>, env: &mut Environment) -> Result<(), LoxError> {
    let mut class_methods: HashMap<String, LoxCallable> = HashMap::new();

    for method in methods {
        let method_declaration: (Token, Vec<Token>, Box<Vec<Statement>>) = method.into_fun_declaration().unwrap();
        let name = method_declaration.0.lexeme;
        let method: LoxCallable = LoxCallable::Closure(name.clone(), method_declaration.1, method_declaration.2, env.clone());
        class_methods.insert(name, method);
    }
    
    let class = LoxClass::new(name.lexeme.clone(), class_methods);
    
    if env.is_global() {
        env.define_global(name.lexeme, Value::Class(Rc::new(class)))
    }

    else {
        Ok(env.define_local(name.lexeme, Value::Class(Rc::new(class))))
    }
}

// Gets the truthiness of a value
fn get_value_truth(value: Value, env: &Environment) -> Result<bool, LoxError> {
    match value {
        Value::Bool(truth) => Ok(truth),
        Value::Identifier(token) => get_value_truth(env.get(&*token)?, env),
        Value::Nil() => Ok(false),
        _ => Err(LoxError::ValueError(value, String::from("Value has no truthiness!")))
    }
}

// Builds a new binary expression from given values.
fn build_binary_expr(left: Value, op: Token, right: Value) -> Expr {
    Expr::Binary(Box::new(Expr::Literal(left)),
                 op,
                 Box::new(Expr::Literal(right)))
}

// Extracts a LoxObject out of an expression.
fn get_object(obj: Expr, env: &mut Environment) -> Result<Rc<RefCell<LoxObject>>, LoxError> {
    let object: Value = interpret_expr(obj, env)?;
    let object: Result<Rc<RefCell<LoxObject>>, Value> = object.into_instance();

    match object {
        Ok(obj) => Ok(obj),
        Err(value) => Err(LoxError::ValueError(value, String::from("Not an object!")))
    }
}