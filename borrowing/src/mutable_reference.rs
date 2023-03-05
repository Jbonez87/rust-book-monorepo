pub fn run() {
    let mut s: String = String::from("Hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}