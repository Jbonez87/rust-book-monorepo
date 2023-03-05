pub fn run() {
  let rect1: (i64, i64) = (30, 50);

  println!("The area of the rectangle is {} square pixels", area(rect1));
}

pub fn area(dimensions: (i64, i64)) -> i64 {
  dimensions.0 * dimensions.1
}