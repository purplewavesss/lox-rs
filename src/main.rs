use std::env;
use std::io::Write;
use std::process;
use std::fs;
use std::str;
use std::io;
use crate::expr::Expr;
use crate::interpreter::interpret;
use crate::parser::Parser;
use crate::scanning::scanner::Scanner;
use crate::scanning::token::Token;
use crate::scanning::token::Value;
use crate::scanning::token_type::TokenType;
use thiserror::Error;

pub mod scanning;
pub mod expr;
pub mod parser;
pub mod interpreter;

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
            let result = run(str::from_utf8(&bytes).unwrap().to_string());
            result
        },
        Err(_) => {
            println!("{path} could not be read.");
            70
        }
    }
}

/// Runs an interactive REPL prompt where code can be continuously executed.
fn run_prompt() {
    let stdin: io::Stdin = io::stdin();

    loop {
        // Write prompt to screem
        print!("> ");
        io::stdout().flush().expect("Something really bad has happened if this program can't flush its own input");
        
        let mut input = String::new();
        
        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                throw(String::from("1"), "Invalid UTF8 input was given to prompt.");
                continue;
            }
        };

        if input == "" {
            break;
        }

        run(input);
    }
}

/// Runs a Lox program.
fn run(source: String) -> i32 {
    let mut scanner = Scanner::new(&source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(tree) => {
            let value: Result<Value, LoxError> = interpret(tree);
            match value {
                Ok(value) => {
                    println!("{value}");
                    0
                },
                Err(error) => report_error_type(error)
            }
        }

        Err(error) => report_error_type(error),
    }
}

/// Throws a Lox error.
pub fn throw(identifier: String, message: &str) {
    report(identifier, "", message);
}

/// Reports a Lox error.
fn report(identifier: String, at: &str, message: &str) {
    eprintln!("[{identifier}] Error{at}: {message}");
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
        }
    }
}

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("Syntax error: ")]
    ParseError(Token, String),
    #[error("Runtime error: ")]
    RuntimeError(Expr, String),
    #[error("Value error: ")]
    ValueError(Value, String),
    #[error("Compiler bug: ")]
    CompilerBug(Expr, String)
}
