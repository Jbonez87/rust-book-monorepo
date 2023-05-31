use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    /*
        The previous new method still relied on a panic! invocation
        to handle errors. Our new build method will use a Result type.
     */
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         /* This is a more user friendly error message. */
    //         panic!("Not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config {query, file_path}
    // }
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

/*
    This is the original implementation for minigrep. It parsed
    terminal arguments, but since it returns a new instance of
    Config, it is no longer needed. The new method on the Config
    struct replaces it.
 */

// fn parse_args(args: &[String]) -> Config {
//   let query = args[1].clone();
//   let file_path = args[2].clone();

//   Config {query, file_path}
// }

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
