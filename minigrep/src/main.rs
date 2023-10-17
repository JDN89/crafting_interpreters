use ::std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // First argument is the path of the executable
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for: {} in file: {}", query, file_path);
}
