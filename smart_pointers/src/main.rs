mod list;

use list::List::{Cons, Nil};

fn main() {
    let b: Box<i32> = Box::new(12);
    println!("b = {}", b);

    let list: list::List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list is: {:?}", list);
}
