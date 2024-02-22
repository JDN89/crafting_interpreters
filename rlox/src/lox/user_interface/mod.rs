use crate::frontend::scanner::Scanner;
use crate::tree_walker::interpreter::Interpreter;
use crate::tree_walker::parser::{Parser, Stmt};
use crate::LoxError;
use std::io::{BufRead, Write};
use std::{fs, io, process};

pub fn run_file(file_path: &str) -> Result<(), io::Error> {
    // initialize the interpreter, which contains the environment field, so that we can hold on to the state of the program one we run it
    let mut interpreter = Interpreter::new();

    let contents = fs::read_to_string(file_path)?;

    if let Err(e) = run(&contents, &mut interpreter) {
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
            LoxError::Return(_) => todo!(),
        }
    }
    Ok(())
}

// REPL: print eval read -> interactive prompt
pub fn run_prompt() -> Result<(), io::Error> {
    // initialize the interpreter, which contains the environment field, so that we can hold on to the state of the program one we run it
    let mut interpreter = Interpreter::new();

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

        if let Err(e) = run(&buf, &mut interpreter) {
            match e {
                LoxError::Interpreter(e) => e.report(),
                LoxError::ParserError(e) => e.report(),
                LoxError::ScannerError(e) => e.report(),
                LoxError::Runtime(e) => e.report(),
                LoxError::Return(_) => todo!(),
            }
        }
    }

    Ok(())
}

// run shouldn't be pub but for the moment I'm using it in my integration tests
pub fn run(source: &String, interpreter: &mut Interpreter) -> Result<(), LoxError> {
    let mut scanner = Scanner::build_scanner(source);
    let tokens = scanner.scan_tokens()?;
    let binding = tokens.clone();
    let mut parser = Parser::build_parser(&binding);
    let statements: Vec<Stmt> = parser.parse()?;
    interpreter.interpret(statements)?;
    Ok(())
}
