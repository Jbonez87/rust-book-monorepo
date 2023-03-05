#[derive(Debug)]
pub enum Message {
  Quit,
  Move { x: i64, y: i64 },
  Write(String),
  ChangeColor(i64, i64, i64)
}

impl Message {
  pub fn call(&self){
    println!("{:?}", self);
  }
}

pub fn test_message() {
  let m: Message = Message::Write(String::from("Hi!"));
  m.call();
}
