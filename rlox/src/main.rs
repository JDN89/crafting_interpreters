use std::env::args;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process;

// lox is a scripting language -> executes directly from source.
// run code through a command-line interface (CLI) or by providing a path to a script file.
fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    match args.len() {
        arg_len if arg_len as usize > 1 => {
            eprintln!("Usage: rlox [script]");
            process::exit(64);
        }
        1 => run_file(&args[0])?,
        _ => run_prompt()?,
    }
    // return ok void
    Ok(())
}

// possible need of converson to Box<dyn Error>
fn run_file(file_path: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(file_path)?;
    run(contents);
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

        run(&buf);
    }

    Ok(())
}
