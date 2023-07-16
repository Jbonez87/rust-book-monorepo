pub struct Guess {
  value: i64
}
// implementing the guess struct allows
// us to write methods to it.
impl Guess {
  pub fn new(value: i64) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100! Got {}", value);
    }
    Guess { value }
  }
  pub fn precise_new(value: i64) -> Guess {
    if value < 1 {
        panic!(
            "Guess value must be greater than or equal to 1, got {}.",
            value
        );
    } else if value > 100 {
        panic!(
            "Guess value must be less than or equal to 100, got {}.",
            value
        );
    }
    Guess { value }
  }
}
