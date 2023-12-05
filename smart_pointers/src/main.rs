mod list;

use list::List::{Cons, Nil};

fn main() {
    let b: Box<i32> = Box::new(12);
    println!("b = {}", b);

    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
