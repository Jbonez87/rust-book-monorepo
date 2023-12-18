mod custom_box;
mod list;

use custom_box::MyBox;
use list::List::{Cons, Nil};

fn main() {
    /*
        You would never use a Box to store a number on the heap in normal situations.
        It's more efficient to use the stack, but for this example we used a Box.
    */
    let b: Box<i32> = Box::new(12);
    println!("b = {}", b);

    /*
        Cons in our List enum allows us to recursively set a new instance of itself
        until the last value is nil.
    */
    let list: list::List<i32> = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let str_list: list::List<&str> = Cons(
        "test",
        Box::new(Cons("cons", Box::new(Cons("example.", Box::new(Nil))))),
    );

    println!("list is: {:?}", list);
    println!("str_list is: {:?}", str_list);

    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x: i32 = 8;
    let y: MyBox<i32> = MyBox::new(x);

    assert_eq!(5, x);
    // this dereferences the y variable.
    assert_eq!(5, *y);
}
