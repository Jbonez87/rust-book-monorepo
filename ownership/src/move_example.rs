pub fn run() {
    //these variables are on the stack
    let x: i64 = 5;
    let y = x;
    println!("{y}");

    // these variables are on the heap
    let s1 = String::from("hello");
    let s2 = s1; // this is a move
    println!("{s2}");
}