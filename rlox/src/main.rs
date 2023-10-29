use std::env::args;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process;

mod lox_error;
mod scanner;
mod token;
mod token_type;
use crate::lox_error::LoxError;
use crate::scanner::Scanner;

// lox is a scripting language -> executes directly from source.
// run code through a command-line interface (CLI) or by providing a path to a script file.
fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    // we passed [0] program name, [1] path to file [x > 1] argumets to many
    if args.len() > 2 {
        eprintln!("Usage: rlox [script]");
        process::exit(64);
        // arg[0] is the programs name and arg[1] is the file_path we'll pass it
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

// possible need of converson to Box<dyn Error>
fn run_file(file_path: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(file_path)?;
    if let Err(e) = run(&contents) {
        e.report();
        process::exit(65);
    }
    Ok(())
}

// REPL: print eval read -> interactive prompt
fn run_prompt() -> Result<(), io::Error> {
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
    let mut scanner = Scanner::build_scanner(source.to_string());
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token); // Use {:?} to format the token
    }
    Ok(())
}
