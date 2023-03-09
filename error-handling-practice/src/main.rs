// mod panic_example;
mod recoverable_errors;

// use panic_example::test_panic;
use recoverable_errors::{file_error, specific_file_error, alternative_file_error_handling};

fn main() {
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
    specific_file_error();
    alternative_file_error_handling()
}
