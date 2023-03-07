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
}