fn main() {
    println!("This is a regular loop");
    loop {
        println!("again!");
        break;
    }

    test_loop();
    loop_label_test();
    while_loop_test();
    bad_while_loop_test();
    for_loop_test();
    countdown();

    for number in 1..4 {
        println!("Number is: {number}");
    }
}

fn test_loop() {
    println!("This is also a regular loop");
    let mut counter: i64 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_label_test() {
    println!("These are labelled and nested loops");
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop_test() {
    println!("This is a while loop");
    let mut number: i64 = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn bad_while_loop_test() {
    println!("This is a bad while loop example");
    let a: [i64; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop_test() {
    println!("This is a for loop example");
    let a: [i128; 5] = [10, 20, 30, 40, 50];

    for item in a {
        println!("The value is: {item}");
    }
}

fn countdown() {
    println!("This is a for loop using rev");
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
