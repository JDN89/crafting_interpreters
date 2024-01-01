use std::io::{BufRead, Write};
use std::{fs, io, process};

use interpreter::Interpreter;
pub use lox_error::*;
use parser::Parser;
use scanner::Scanner;

mod ast_printer;
mod environment;
mod expr;
mod interpreter;
mod lox_error;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod ast;

// possible need of converson to Box<dyn Error>
pub fn run_file(file_path: &str) -> Result<(), io::Error> {
    // initialize the interpreter, which contains the environment field, so that we can hold on to the state of the program one we run it
    let interpreter = Interpreter::new();

    let contents = fs::read_to_string(file_path)?;

    if let Err(e) = run(&contents, &interpreter) {
        match e {
            LoxError::ScannerError(e) => {
                e.report();
                process::exit(65)
            }
            LoxError::ParserError(e) => {
                e.report();
                process::exit(66)
            }
            LoxError::Interpreter(e) => {
                e.report();
                process::exit(70)
            }
            LoxError::Runtime(e) => {
                e.report();
                process::exit(1)
            }
        }
    }
    Ok(())
}

// REPL: print eval read -> interactive prompt
pub fn run_prompt() -> Result<(), io::Error> {
    // initialize the interpreter, which contains the environment field, so that we can hold on to the state of the program one we run it
    let interpreter = Interpreter::new();

    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());

    loop {
        print!("> ");
        io::stdout().flush()?; // Ensure the prompt is displayed.

        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        if buf.is_empty() {
            // Check for an empty line and break the loop if encountered.
            break;
        }

        if let Err(e) = run(&buf, &interpreter) {
            match e {
                LoxError::Interpreter(e) => e.report(),
                LoxError::ParserError(e) => e.report(),
                LoxError::ScannerError(e) => e.report(),
                LoxError::Runtime(e) => e.report(),
            }
        }
    }

    Ok(())
}

fn run(source: &String, interpreter: &Interpreter) -> Result<(), LoxError> {
    let mut scanner = Scanner::build_scanner(source);
    // println!("{:?}",source);
    let tokens = scanner.scan_tokens()?;
    // for token in &tokens {
    //     println!("{}",token);
    // }
    let mut parser = Parser::build_parser(tokens.clone());
    let statements: Vec<stmt::Stmt> = parser.parse()?;
    // for ast in &statements {
    //     println!("{:?}",ast);
    // }
    interpreter.interpret(statements)?;
    Ok(())
}
