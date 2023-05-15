use std::env;

fn main() {
    /*
        This is similar to `process.argv.slice(2)` in NodeJS.
     */
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
