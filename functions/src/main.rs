fn main() {
    another_function(55);
    print_labeled_measurements(6, 'h');
    expression_test();
    let x: i64 = five();
    println!("The value of x is: {x}");

    let x: i64 = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i64) {
    println!("The value of x is: {:?}", x);
}

fn print_labeled_measurements(value: i64, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression_test() {
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i64 {
    5
}

fn plus_one(x: i64) -> i64 {
    // It's important to not add a semicolon on the end.
    // It will evaluate as a statement rather than an expression.
    x + 1
}
