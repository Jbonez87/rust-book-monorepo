pub fn if_let_example() {
  let config_max: Option<i64> = Some(3i64);
  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  }
}