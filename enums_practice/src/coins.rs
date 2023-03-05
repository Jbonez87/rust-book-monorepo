#[derive(Debug)]
pub enum UsState {
  Alabama,
  Alaska,
  Arizona,
  Arkansas,
  California,
  Colorado,
  Connecticut,
  Delaware,
  Florida,
  Georgia,
  Hawaii,
  Idaho,
  Illinois,
  Indiana,
  Iowa,
  Kansas,
  Kentucky,
  Louisiana,
  Maine,
  Maryland,
  Massachussetts,
  Michigan,
  Minnesota,
  Mississippi,
  Missouri,
  Montana,
  Nebraska,
  Nevada,
  NewHampshire,
  NewJersey,
  NewMexico,
  NewYork,
  NorthCarolina,
  NorthDakota,
  Ohio,
  Oklahoma,
  Oregon,
  Pennsylvania,
  RhodeIsland,
  SouthCarolina,
  SouthDakota,
  Tennessee,
  Texas,
  Utah,
  Vermont,
  Virginia,
  Washington,
  WestVirginia,
  Wisconsin,
  Wyoming
}

pub enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

pub fn value_in_cents(coin: Coin) -> i64 {
  match coin {
    Coin::Penny => {
      println!("Lucky Penny!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // Coin::Quarter => 25
    Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
  }
}