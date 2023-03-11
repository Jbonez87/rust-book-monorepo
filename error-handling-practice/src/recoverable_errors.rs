use std::fs::File;
use std::io::{ErrorKind, self, Read};

pub fn file_error() {
  /*
    File::open returns a Result type:
    enum Result<T, E> {
      Ok(T),
      Err(E)
    }
  */
  let greeting = File::open("hello.txt");

  /*
    One way to handle this could be handled is with a match statement
  */
  let greeting_file = match greeting {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error)
  };
}

pub fn specific_file_error() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(File) => File,
    // error is of type io::Error which has a method called kind()
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
          Ok(fc) => fc,
          Err(e) => panic!("Problem creating the file: {:?}", e)
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    },
  };
}

pub fn matchless_file_error() {
  let greeting_file = File::open("hello_again.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello_again.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}

pub fn unwrap_file_error() {
  let goodbye = File::open("goodbye.txt").unwrap();
}

pub fn expect_file_error() {
  let greeting_file = File::open("good_day.txt")
        .expect("hello.txt should be included in this project");
}

pub fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("username.txt");

  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e)
  }
}

/*
  The ? operator is similar to a match statement. If the value of the 
  Result is an Ok, then it will return the value inside of the Ok.
  Or else it will return the Err. The main difference between ? and match
  is that Errors returned in the ? operator use the `from` function defined in 
  the From trait from the standard library. This trait converts one type to 
  another. 
 */
pub fn concise_read_username_from_file() -> Result<String, io::Error> {
  let mut username_file = File::open("username.txt")?;
  let mut username = String::new();

  username_file.read_to_string(&mut username)?;
  Ok(username)
}