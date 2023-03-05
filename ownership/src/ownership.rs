pub fn run() {
    /*
        ------- Ownership Rules -------
        1. Each value in Rust has a variable that's called its owner
        2. There can only be one owner at a time
        3. When the owner goes out of scope, the value will be dropped
    */
    {  // s is not valid here, it's not yet declared
        let s = "Hello"; // s is valid from this point forward
        // do stuff with s
        println!("{s}");
    } // the scope is now over. s is no longer valid.

    let mut s: String = String::from("Hello");

    s.push_str(", World!");

    println!("{}", s);

    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("{s}");
        // do stuff with s
    } // this scope is now over, and s is no longer valid

}