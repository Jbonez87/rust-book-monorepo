fn main() {
     let number: i64 = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    check_num();
    multiple_nums();
    conditional_let();
}

fn check_num() {
     let number: i64 = 3;

    if number != 0 {
        println!("number was something other than zero");
    }   
}

fn multiple_nums() {
    let number: i32 = 13;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    println!("number is not divisible by 4, 3, or 2");
}

fn conditional_let() {
    let condition = true;
    let number: i128 = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

