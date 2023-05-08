mod rectangle;
mod guess;
mod add;
mod greeting;
mod print_and_return;

use rectangle::{Rectangle};
use guess::{Guess};
use add::{add, add_two};
use greeting::{greeting, bad_greeting};
use print_and_return::{prints_and_returns_10};

#[cfg(test)]
mod tests {
    use super::*;
    /*
        To run only a specific test, you can pass the name
        of the test function as an argument when running 
        `cargo test`. For example, to test the function below:
        `cargo test exploration`
     */
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    /*
        We can also run multiple tests by filtering. If we wanted
        to run tests that contain the word `add`, we could run
        `cargo test add`. This would run two tests containing the
        word `add` in the function name.
     */
    #[test]
    fn it_works() -> Result<(), String> {
        if add(2,2) == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal 4!"))
        }
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
    /*
        If we wanted to ignore this test, we could add the following:
        #[ignore]
     */
    #[test]
    #[ignore]
    /*
        If we wanted to run only the ignored tests, we would run the 
        following: `cargo test -- --ignored`.
        If we wanted to run all tests whether they are ignored or not,
        we could run: `cargo test -- --include-ignored`.
     */
    fn this_test_will_be_ignored() {
        panic!("I will never panic because I am ignored");
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("John");
        assert!(result.contains("John"));
    }
    // #[test]
    // fn bad_greeting_contains_name() {
    //     let result = bad_greeting("John");
    //     assert!(
    //         result.contains("John"),
    //         "bad_greeting did not contain name, value was `{}`",
    //         result
    //     );
    // }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn more_than_100() {
        Guess::precise_new(200);
    }
    /*
        If we run this test using cargo test -- --show-output,
        we will see the `println!` output.
    */
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(5);
    //     assert_eq!(value, 8);
    // }
}
