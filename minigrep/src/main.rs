use std::env;
use std::process;
use minigrep::Config;

fn main() {
    /*
        This is similar to `process.argv.slice(2)` in NodeJS.
     */
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    /*
        The `run` function abstracts the read_file logic out of main.
        We can run this conditionally to handle any errors it might
        throw. We let the `main` function decide how it wants to handle
        an error from the `run` function.
     */
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}