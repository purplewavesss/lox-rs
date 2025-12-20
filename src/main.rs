use std::env;
use std::io::Write;
use std::process;
use std::fs;
use std::str;
use std::io;
use crate::expr::Expr;
use crate::interpreter::interpret::interpret;
use crate::parser::Parser;
use crate::scanning::scanner::Scanner;
use crate::scanning::token::Token;
use crate::scanning::token::Value;
use crate::scanning::token_type::TokenType;
use crate::statement::Statement;
use thiserror::Error;
use crate::interpreter::environment::Environment;

pub mod scanning;
pub mod expr;
pub mod parser;
pub mod interpreter;
pub mod statement;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len: usize = args.len();

    // Invalid arg number
    if arg_len > 2 {
        println!("Usage: lox-rs [script]");
        process::exit(64);
    }

    // name and file
    else if arg_len == 2 {
        // Exits with the error code given from run
        process::exit(run_file(&args[1]));
    }

    // REPL
    else {
        run_prompt();
    }
}

/// Runs a given file in the Lox interpreter.
/// 
/// Returns the error code of the Lox program.
fn run_file(path: &String) -> i32 {
    let file = fs::read(path);

    match file {
        Ok(bytes) =>  {
            let result = run(str::from_utf8(&bytes).unwrap().to_string(), &mut Environment::new());
            result
        },
        Err(_) => {
            println!("{path} could not be read.");
            io::stdout().flush().unwrap();
            70
        }
    }
}

/// Runs an interactive REPL prompt where code can be continuously executed.
fn run_prompt() {
    let stdin: io::Stdin = io::stdin();
    let mut env = Environment::new();

    loop {
        // Write prompt to screen
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        
        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                throw(String::from("1"), "Invalid UTF8 input was given to prompt.");
                continue;
            }
        };

        if input == "\n" {
            break;
        }

        run(input, &mut env);
    }
}

/// Runs a Lox program.
///
/// This function takes in a starting environment, so an environment can have state throughout programs
/// in the REPL.
fn run(source: String, env: &mut Environment) -> i32 {
    let mut scanner = Scanner::new(&source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let parse_results: Vec<Result<Statement, LoxError>> = parser.parse();
    
    match to_statements(parse_results) {
        Ok(stmts) => match interpret(stmts, env) {
            Ok(_) => 0,
            Err(err) => report_error_type(err)
        },

        Err(errs) => {
            // Report all syntax errors
            _ = errs.into_iter().for_each(|err| { report_error_type(err); });
            
            // Only a parse error can be thrown at this stage, so we return a parse error value
            1
        }
    }
}

/// Converts a parser result into a vec of statements, if possible
fn to_statements(results: Vec<Result<Statement, LoxError>>) -> Result<Vec<Statement>, Vec<LoxError>> {
    let mut statements: Vec<Statement> = Vec::new();
    let mut errors: Vec<LoxError> = Vec::new();
    let mut seen_error: bool = false;

    for result in results {
        match result {
            Ok(stmt) => {
                if !seen_error {
                    statements.push(stmt);
                }
            }

            Err(err) => {
                seen_error = true;
                errors.push(err);
            }
        }
    };

    if seen_error {
        Err(errors)
    }

    else {
        Ok(statements)
    }
}

/// Throws a Lox error.
pub fn throw(identifier: String, message: &str) {
    report(identifier, "", message);
}

/// Reports a Lox error.
fn report(identifier: String, at: &str, message: &str) {
    eprintln!("[{identifier}] Error{at}: {message}");
    io::stderr().flush().unwrap();
}

fn report_error_type(error: LoxError) -> i32 {
    match error {
        LoxError::ParseError(token, msg) => {
            if token.token_type == TokenType::End {
                report(token.line.to_string(), " at end", &msg);
            }

            else {
                let at: String = format!(" at '{}'", token.lexeme);
                report(token.line.to_string(), &at, &msg);
            }
            1
        },
        LoxError::RuntimeError(expr, msg) => {
            let at: String = format!(" in '{expr}'");
            report(String::from("Runtime Error"), &at, &msg);
            2
        },
        LoxError::ValueError(value, msg) => {
            let at: String = format!(" for '{value}'");
            report(String::from("Value Error"), &at, &msg);
            3
        },
        LoxError::CompilerBug(expr, msg) => {
            let at: String = format!(" in '{expr}'");
            report(String::from("Compiler Bug"), &at, &msg);
            4
        },
        LoxError::NameError(name, msg) => {
            let at: String = format!(" for '{name}'");
            report(String::from("Name Error"), &at, &msg);
            5
        }

    }
}

#[derive(Error, Debug, Clone)]
pub enum LoxError {
    #[error("Syntax error: ")]
    ParseError(Token, String),
    #[error("Runtime error: ")]
    RuntimeError(Expr, String),
    #[error("Value error: ")]
    ValueError(Value, String),
    #[error("Compiler bug: ")]
    CompilerBug(Statement, String),
    #[error("Name error: ")]
    NameError(String, String)
}
