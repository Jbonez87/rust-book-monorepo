mod parse_config;

use std::env;
use std::fs;
use parse_config::parse_args;

fn main() {
    /*
        This is similar to `process.argv.slice(2)` in NodeJS.
     */
    let args: Vec<String> = env::args().collect();
    
    let (query, file_path) = parse_args(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file!");

    println!("With text:\n{contents}");
}
