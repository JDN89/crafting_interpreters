use ::std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // First argument is the path of the executable
    println!(
        "Searching for: {} in file: {}",
        config.query, config.file_path
    );

    // we don't need to call unwrap_or_else because run returns void -> if let
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1)
    }
}
