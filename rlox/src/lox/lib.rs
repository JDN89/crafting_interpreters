use std::io::{BufRead, Write};
use std::{fs, io, process};

use interpreter::Interpreter;
pub use lox_error::*;
use parser::Parser;
use scanner::Scanner;

mod ast;
mod ast_printer;
mod environment;
mod interpreter;
mod lox_error;
mod parser;
mod scanner;
mod token;
mod token_type;

// possible need of converson to Box<dyn Error>
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
            }
        }
    }

    Ok(())
}

fn run(source: &String, interpreter: &mut Interpreter) -> Result<(), LoxError> {
    let mut scanner = Scanner::build_scanner(source);
    // println!("{:?}",source);
    let tokens = scanner.scan_tokens()?;
    // for token in &tokens {
    //     println!("{}",token);
    // }
    let mut parser = Parser::build_parser(tokens.clone());
    let statements: Vec<ast::Stmt> = parser.parse()?;
    // for ast in &statements {
    //     println!("{:?}",ast);
    // }
    interpreter.interpret(statements)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_whitespace(input: &str) -> String {
        input.lines().map(|line| line.trim()).collect::<Vec<&str>>().join("")
    }
    #[test]
    fn test_run_with_valid_source() {
        let code_block_test = String::from(
            r#"var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;"#,
        );

        let mut interpreter = Interpreter::new();

        // Second test
        let _ = run(&code_block_test, &mut interpreter);
        let output = interpreter.get_outpout();
        let expected = r#" inner a
    outer b
    global c
    outer a
    outer b
    global c
    global a
    global b
    global c
    "#;
        let processed_expected = remove_whitespace(expected);

        let output_str = String::from_utf8_lossy(&output)
            .lines()
            .map(|line| line)
            .collect::<Vec<&str>>()
            .join(" ");

        println!("Actual Output:\n{}", output_str);
        println!("Expected Output:\n{}", processed_expected);

        assert_eq!(output_str, processed_expected.trim());
    }
}
