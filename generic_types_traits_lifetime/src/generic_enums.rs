/*
  This is an example of an enum that uses generics. The standard library
  already provides this type, but this demonstrates how the `Result` type
  is defined.
 */

pub enum Result<T, E> {
  Ok(T),
  Err(E)
}

pub enum Option<T> {
  Some(T),
  None
}

/*
  The following are examples of how the Rust compiler performs
  monomorphization, turning generic types into concrete types at compile time.
  This allows us to use generic types without affecting performance.
 */

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}

pub fn generic_to_concrete() {
  let integer = Option_i32::Some(5);
  let float = Option_f64::Some(5.0);
  println!("integer = {:?}, float = {:?}", integer, float);
}


