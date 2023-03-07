use std::fs::File;

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