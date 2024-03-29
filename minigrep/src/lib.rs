use std::error::Error;
use std::{fs, env};

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

// impl Config {
//     pub fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("Not enough arguments");
//         }

//         let query: String = args[1].clone();
//         let file_path: String = args[2].clone();

//         let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

//         Ok(Config { 
//           query,
//           file_path,
//           ignore_case,
//         })
//     }
// }

/*
  Alternative config implementation using the Iterator trait.
*/
impl Config {
  pub fn build(
    mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
        args.next();

        let query: String = match args.next() {
          Some(args) => args,
          None => return Err("Didn't get a query string.")
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results: Vec<&str> = if config.ignore_case {
      search_case_insensitive(&config.query, &contents)
    } else {
      search(&config.query, &contents)
    };

    for line in results {
      println!("{}", line);
    }

    Ok(())
}

/*
  Search and Search Case Insensitive can probably be changed to use
  or implement a common trait.
 */
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//   let mut results: Vec<&str> = Vec::new();
  
//   for line in contents.lines() {
//     if line.contains(query) {
//       results.push(line);
//     }
//   }
//   results
// }

/*
  Alternative search function using Iterators
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
  .lines()
  .filter(|line: &&str| line.contains(query))
  .collect()
}

// pub fn search_case_insensitive<'a>(
//   query: &str, contents: &'a str
// ) -> Vec<&'a str> {
//   let query: String = query.to_lowercase();
//   let mut results: Vec<&str> = Vec::new();

//   for line in contents.lines() {
//       if line.to_lowercase().contains(&query) {
//           results.push(line);
//       }
//   }

//   results
// }

/*
  Alternative search_case_insensitive using Iterators
*/
pub fn search_case_insensitive<'a>(
  query: &str, contents: &'a str
) -> Vec<&'a str> {
  contents
  .lines()
  .filter(|line: &&str| line.to_lowercase().contains(&query.to_lowercase()))
  .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query: &str = "duct";
    let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_sensitive() {
    let query: &str = "duct";
    let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query: &str = "rUsT";
    let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
  }
}