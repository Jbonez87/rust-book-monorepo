mod list;

use list::List;

fn main() {
    let b: Box<i32> = Box::new(12);
    println!("b = {}", b);
}
