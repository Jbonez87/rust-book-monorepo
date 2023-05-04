mod rectangle;

use rectangle::Rectangle;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn another_test() {
        panic!("This test should fail!");
    }
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
}
