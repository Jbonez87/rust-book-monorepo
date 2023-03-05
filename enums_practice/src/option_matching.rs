pub fn plus_one(x: Option<i64>) -> Option<i64> {
  match x {
    None => None,
    Some(i) => {
      println!("Num: {}", i + 1);
      Some(i + 1)
    }
  }
}