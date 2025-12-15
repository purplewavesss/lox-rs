use std::env;
use std::io::Write;
use std::process;
use std::fs;
use std::str;
use std::io;

use crate::scanning::scanner::Scanner;
use crate::scanning::token::Token;

pub mod scanning;

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
                error(1, "Invalid UTF8 input was given to prompt.");
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

    for token in tokens {
        println!("{}", token);
    }

    0
}

/// Throws a Lox error.
fn error(line: usize, message: &str) {
    report(line, "", message);
}

/// Reports a Lox error.
fn report(line: usize, at: &str, message: &str) {
    eprintln!("[{line}] Error{at}: {message}");
}