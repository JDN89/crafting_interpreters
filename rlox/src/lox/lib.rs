use std::io::{BufRead, Write};
use std::{fs, io, process};

use ast_printer::AstPrinter;
pub use lox_error::*;
use scanner::Scanner;
use parser::Parser;

mod ast_printer;
mod expr;
mod lox_error;
mod parser;
mod scanner;
mod token;
mod token_type;

// possible need of converson to Box<dyn Error>
pub fn run_file(file_path: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(file_path)?;
    if let Err(e) = run(&contents) {
        e.report();
        process::exit(65);
    }
    Ok(())
}

// REPL: print eval read -> interactive prompt
pub fn run_prompt() -> Result<(), io::Error> {
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

        if let Err(e) = run(&buf) {
            e.report()
        }
    }

    Ok(())
}

fn run(source: &String) -> Result<(), LoxError> {
    let mut scanner = Scanner::build_scanner(source);
    let tokens = scanner.scan_tokens()?;
    let mut  parser = Parser::build_parser(tokens.clone());
    let ast = parser.parse()?;
    let ast_printer = AstPrinter{};
    let expr =ast_printer.print(&ast);
    println!("{:?}",expr);

    for token in tokens {
        println!("{:?}", token); // Use {:?} to format the token
    }
    Ok(())
}
