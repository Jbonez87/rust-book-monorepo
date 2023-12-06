mod list;

use list::List::{Cons, Nil};

fn main() {
    let b: Box<i32> = Box::new(12);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
