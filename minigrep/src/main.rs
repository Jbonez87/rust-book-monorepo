use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    /*
        This is similar to `process.argv.slice(2)` in NodeJS.
     */
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    
    // let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    /*
        Alternative build method using iterators
     */
    let config: Config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    /*
        The `run` function abstracts the read_file logic out of main.
        We can run this conditionally to handle any errors it might
        throw. We let the `main` function decide how it wants to handle
        an error from the `run` function.
     */
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}