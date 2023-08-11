// Fibonacci is called in main
fn main() {
    println!("Hello, world!");
    let f: i64 = fibonacci(6);
    println!("{f}");
}

fn fibonacci(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
