#[derive(Debug)]
pub struct Rectangle {
    width: i64,
    height: i64,
}

impl Rectangle {
  // fn area(&self) -> i64 {
  //     self.width * self.height
  // }
  // fn width(&self) -> bool {
  //     self.width > 0
  // }
  fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
  }
  // fn square(size: i64) -> Self {
  //     Self {
  //       width: size,
  //       height: size,
  //     }
  // }
}