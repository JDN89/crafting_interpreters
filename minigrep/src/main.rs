use ::std::env;
use ::std::fs;

// fix 4 problems:
// 1 single responsability
// 2 query and file_path are config vars contents is used to preform programs logic -> group config
// vars in on struct
// 3 expect used to print same error always
// error handling code in one place

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    // First argument is the path of the executable
    println!(
        "Searching for: {} in file: {}",
        config.query, config.file_path
    );
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n {contents}");
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
