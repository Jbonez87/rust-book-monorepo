#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    White
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

fn main() {
    println!("Hello, world!");
}
