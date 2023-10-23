use ::std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // First argument is the path of the executable
    println!(
        "Searching for: {} in file: {}",
        config.query, config.file_path
    );

    // we don't need to call unwrap_or_else because run returns void -> if let
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
