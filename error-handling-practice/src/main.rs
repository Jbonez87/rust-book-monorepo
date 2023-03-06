// mod panic_example;

// use panic_example::test_panic;

fn main() {
    // test_panic();

    // In this example, the panic can be found with a backtrace
    let v = vec![1, 2, 3];
    /* 
     Since this index is out of the memory bounds of the Vector, 
     this is called a buffer overread. It will attempt to retrieve
     whatever value is at this address in memory even if it's not
     in the Vector itself. This is a security issue that Rust attempts
     to prevent with a panic! macro.
    */ 
    v[99];
}
