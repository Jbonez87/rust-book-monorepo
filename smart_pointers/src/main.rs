mod list;

use list::List::{Cons, Nil};

fn main() {
    /* 
        You would never use a Box to store a number on the heap in normal situations.
        It's more efficient to use the stack, but for the example we used a Box.
    */
    let b: Box<i32> = Box::new(12);
    println!("b = {}", b);

    let list: list::List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list is: {:?}", list);

    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
