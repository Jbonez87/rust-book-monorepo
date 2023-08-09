use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

pub fn guessing_game_basic_error_handling() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
          This if statement will only check if the number is out
          of the given range of 1 to 100, but checking this every
          time could have a negative impact on performance.
         */
        if guess < 1 || guess > 100 {
          println!("The secret number will be between 1 and 100");
          continue;
        }

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
          Ordering::Less => println!("{}", "Too small!".red()),
          Ordering::Greater => println!("{}","Too big!".red()),
          Ordering::Equal => {
            println!("{}", "You win!".cyan());
            break;
          }
        }
    }
}

pub struct Guess {
  value: i32
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}", value)
    }

    Guess { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}