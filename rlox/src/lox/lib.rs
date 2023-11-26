use std::io::{BufRead, Write};
use std::{fs, io, process};

pub use lox_error::*;
use scanner::Scanner;

use crate::token::Token;
use crate::token_type::TokenType;
use crate::{expr::*, LoxError};

mod ast_printer;
mod expr;
mod lox_error;
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
    for token in tokens {
        println!("{:?}", token); // Use {:?} to format the token
    }
    Ok(())
}
// test ast printer
pub fn test_ast_printer() {
    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: token::Literal::Integer(123.0),
                line: 1,
            },
            right: Box::new(Expr::Literal(LiteralExpr {
                value: token::Literal::Integer(123.0),
            })),
        })),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: token::Literal::Integer(0.0),
            line: 1,
        },
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: token::Literal::Integer(45.67),
            })),
        })),
    });

    println!("{}", ast_printer::AstPrinter {}.print(&expression).unwrap());
}
