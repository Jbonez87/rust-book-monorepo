mod reference_example;
mod mutable_reference;
mod multiple_mutable_references;
mod mutable_immutable_test;
mod slices_example;

fn main() {
    println!("Hello, world!");
    reference_example::run();
    mutable_reference::run();
    multiple_mutable_references::run();
    mutable_immutable_test::run();
    bad_slice();
    good_slice();

    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole

    let word = slices_example::first_word(&my_string[0..6]);
    println!("{word}");
    let word = slices_example::first_word(&my_string[..]);
    println!("{word}");
    // `slices_example::first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s

    let word = slices_example::first_word(&my_string);
    println!("{word}");
    let my_string_literal = "hello world";

    // `slices_example::first_word` works on slices of string literals, whether partial or whole
    let word = slices_example::first_word(&my_string_literal[0..6]);
    println!("{word}");
    let word = slices_example::first_word(&my_string_literal[..]);
    println!("{word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = slices_example::first_word(my_string_literal);
    println!("{word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    println!("{:?}", slice);

    assert_eq!(slice, &[2, 3]);
}

fn bad_slice() {
    let mut s: String = String::from("Hello World!");

    let word = slices_example::first_word(&s); // word will get the value 5

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("First word is: {}", hello);
    println!("Second word is: {}", world);

    println!("{word}");

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn good_slice() {
    let s: String = String::from("Hello");

    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[..2];
    println!("{slice}");

    let len = s.len();

    let slice = &s[3..len];
    println!("{slice}");
    let slice = &s[3..];
    println!("{slice}");

    let slice = &s[0..len];
    println!("{slice}");
    let slice = &s[..];
    println!("{slice}");
}