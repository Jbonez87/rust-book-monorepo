//! # Cargo Practice
//!
//! `cargo_practice` is a collection of utilities to make performing certain
//! calculations more convenient. It is also an example crate that shows the 
//! different cargo options available to Rust developers!
 
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_practice::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}