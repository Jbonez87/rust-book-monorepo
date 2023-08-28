pub fn option_test() {
  let some_number: Option<i32> = Some(5);
  let some_char = Some('e');

  let absent_number: Option<i32> = None;

  println!("Some number: {:?}, some char: {:?}, absent number: {:?}", some_number, some_char, absent_number);

  let some_value: Option<i32> = Some(22);

  match some_value {
      Some(22) => println!("Twenty-two"),
      _ => ()
  }

  if let Some(22) = some_value {
    println!("Twenty-two");
  }
}