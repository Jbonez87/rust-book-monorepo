use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

/*
    An example of a closure defined with traits.
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
        f: FnOnce() -> T 
        {
            match self {
                Some(x) => x,
                None => f()
            }
        }
    }
*/

pub fn closures_example() {
    let store: Inventory = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);

    let giveaway1: ShirtColor = store.giveaway(user_pref1);
    
    println!(
        "The user with preference {:?} gets {:?}!",
        user_pref1, giveaway1
    );

    let user_pref2: Option<ShirtColor> = None;
    
    let giveaway2: ShirtColor = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}!",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: i64| -> i64 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("This closure is: {}", expensive_closure(34));

    let example_closure = |x| x;

    let s = example_closure(String::from("Hello"));
    /*
      This would not work since the compiler already inferred
      example_closure takes in a String
    */
    // let t = example_closure(5);

    let nums: Vec<i32> = vec![6, 4, 11, 23, 19];

    let max_el: Option<&i32> = nums.iter().max();

    match max_el {
        Some(&max) => println!("The maximum element is: {}", max),
        None => println!("The vector is empty.")
    }

    let list: Vec<i32> = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();

    println!("After calling closure: {:?}", list);

    // Immutable closure example
    let mut list: Vec<i32> = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || list.push(9);
    borrow_mutably();

    println!("After calling closure: {:?}", list);

    let list: Vec<i32> = vec![3, 6, 9];

    println!("Before defining closure: {:?}", list);

    /*
        The move keyword allows the closure to take ownership
        of the list vector and spawn a new thread to log it.
    */
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list: [Rectangle; 3] = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort_by_key takes in a closure
    list.sort_by_key(|r: &Rectangle| r.width);
    println!("{:#?}", list);

    /*
        The example below will not compile since sort_by_key
        is using the FnMut trait for its closure and not
        FnOnce.
     */
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);

    let mut num_sort_operations: i32 = 0;

    list.sort_by_key(|r: &Rectangle| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}", list);
}
