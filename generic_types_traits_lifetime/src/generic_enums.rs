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