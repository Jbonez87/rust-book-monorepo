use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub file_path: String,
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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}