mod ownership;
mod move_example;
mod cloning_example;
mod copy_example;
mod functional_ownership;
mod tuple_ownership;

/*
  ------- Ownership Rules -------
  1. Each value in Rust has a variable that's called its owner
  2. There can only be one owner at a time
  3. When the owner goes out of scope, the value will be dropped
 */

use functional_ownership::calculate_length;

fn main() {
    ownership::run();
    move_example::run();
    cloning_example::run();
    copy_example::run();
    functional_ownership::run();
    tuple_ownership::run();

    let test: String = String::from("test");
    let length = calculate_length(&test);
    println!("The length of test is: {}", length);
    

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);    // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);    // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

