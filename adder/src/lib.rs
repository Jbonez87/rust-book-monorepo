mod rectangle;
mod guess;

use rectangle::{Rectangle};
use guess::{Guess};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i64) -> i64 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn bad_greeting(name: &str) -> String {
    String::from("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // #[test]
    // fn another_test() {
    //     panic!("This test should fail!");
    // }
    /*
        Rust has a built in assert macro
     */
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 9,
            height: 8
        };

        let smaller = Rectangle {
            width: 7,
            height: 6
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 4,
            height: 2
        };
        let larger = Rectangle {
            width: 7,
            height: 3
        };
        assert!(!smaller.can_hold(&larger));
    }
    /*
        In other languages, the arguments in assert_eq
        may be called expected and actual. In Rust,
        these are called right and left.
     */
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn it_only_adds_two() {
        assert_ne!(5, add_two(2));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("John");
        assert!(result.contains("John"));
    }
    #[test]
    fn bad_greeting_contains_name() {
        let result = bad_greeting("John");
        assert!(
            result.contains("John"),
            "bad_greeting did not contain name, value was `{}`",
            result
        );
    }
}
