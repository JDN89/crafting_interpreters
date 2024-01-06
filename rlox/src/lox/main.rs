use std::env::args;
use std::{io, process};
use rulox::user_interface::{run_file, run_prompt};

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
