use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue
}

pub struct Inventory {
    shirts: Vec<ShirtColor>,
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

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_pref1);
    
    println!(
        "The user with preference {:?} gets {:?}!",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    
    let giveaway2 = store.giveaway(user_pref2);

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

    let nums = vec![6, 4, 11, 23, 19];

    let max_el = nums.iter().max();

    match max_el {
        Some(&max) => println!("The maximum element is: {}", max),
        None => println!("The vector is empty.")
    }

    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();

    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || list.push(9);
    borrow_mutably();

    println!("After calling closure: {:?}", list);

    println!("End of closures")


}
