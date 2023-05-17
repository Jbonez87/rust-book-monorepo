use std::env;
use std::fs;

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

fn parse_args(args: &[String]) -> (&str, &str) {
  let query = &args[1];
  let file_path = &args[2];

  (query, file_path)
}
