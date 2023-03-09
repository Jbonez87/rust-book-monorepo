use std::fs::File;
use std::io::ErrorKind;

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