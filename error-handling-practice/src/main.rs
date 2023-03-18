/*
    Uncomment any of these modules below and call them
    in the `main` function to trigger and test the errors.
 */

// mod panic_example;
mod recoverable_errors;
mod error_handling_examples;

use std::{error::Error, fs::File};
// use panic_example::test_panic;
use recoverable_errors::{
    // file_error, 
    // specific_file_error, 
    // matchless_file_error, 
    // unwrap_file_error, 
    // expect_file_error, 
    // read_username_from_file, 
    // concise_read_username_from_file,
    // short_read_username_from_file,
    last_char_of_first_line,
};

use error_handling_examples::{non_error_example};

fn main() -> Result<(), Box<dyn Error>> {
    // test_panic();

    // In this example, the panic can be found with a backtrace
    // let v = vec![1, 2, 3];
    /* 
     Since this index is out of the memory bounds of the Vector, 
     this is called a buffer overread. It will attempt to retrieve
     whatever value is at this address in memory even if it's not
     in the Vector itself. This is a security issue that Rust attempts
     to prevent with a panic! macro.
    */ 
    // v[99];

    // In order to see the backtrace, run this command: RUST_BACKTRACE=1 cargo run

    // file_error();
    // specific_file_error();
    // matchless_file_error();
    // unwrap_file_error();
    // expect_file_error();
    // read_username_from_file();
    // concise_read_username_from_file();
    // short_read_username_from_file();
    non_error_example();
    
    let word = "Test";

    let word = last_char_of_first_line(word);

    println!("Word is: {:?}", word);

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
