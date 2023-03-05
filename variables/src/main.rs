use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn numbers() {
    println!("numbers function executed!");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}

fn number_types() {
    let a = 98_322; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)
    let f: u8 = 255;

    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");
    println!("The value of d is: {d}");
    println!("The value of e is: {e}");
    println!("The value of f is: {f}");
}

fn equations() {
    println!("equations function executed!");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("The sum is {}", sum);
    println!("The difference is {}", difference);
    println!("The product is {}", product);
    println!("The quotient is {}", quotient);
    println!("The floored is {}", floored);
    println!("The remainder is {}", remainder);
}

fn booleans() {
    println!("booleans function executed!");

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t: {t} and f: {f}");
}

fn chars() {
    println!("chars function executed!");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}, {} and {} are all chars", c, z, heart_eyed_cat);
}

fn tuples() {
    println!("tuples function executed!");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");

    let t: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = t.0;

    let six_point_four = t.1;

    let one = t.2;

    println!("This is an alternate method accessing elements in tuples.
    {five_hundred} {six_point_four} {one}");
}

fn arrays() {
    println!("arrays function executed!");

    let a: [i64; 5] = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("The value of months is: {:?}", months);

    let b: [i64; 5] = [3; 5];

    println!("The value of b is {:?}", b);

    let first = a[0];

    let second = a[1];

    println!("The value of first and second is {} & {}", first, second);    
}

fn indexes() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn test_function(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("This block of code represents an inner scope and shadowing example!");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    numbers();
    number_types();
    equations();
    booleans();
    chars();
    tuples();
    arrays();
    indexes();

    let sum = test_function(55, 27);

    println!("The value of sum is: {sum}");
}

