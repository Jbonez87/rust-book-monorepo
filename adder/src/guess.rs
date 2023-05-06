pub struct Guess {
  value: i64
}

impl Guess {
  pub fn new(value: i64) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100! Got {}", value);
    }
    Guess { value }
  }
}