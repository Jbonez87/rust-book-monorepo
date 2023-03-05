#[derive(Debug)]
pub struct Rectangle {
    width: i64,
    height: i64,
}

impl Rectangle {
  fn area(&self) -> i64 {
      self.width * self.height
  }
  fn width(&self) -> bool {
      self.width > 0
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
  }
  fn square(size: i64) -> Self {
      Self {
        width: size,
        height: size,
      }
  }
}

pub fn run() {
    let rect1: Rectangle = Rectangle { 
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let rect2 = Rectangle::square(25);

    println!("rect2's size is: {:#?}", rect2)
}

pub fn area(rectangle: &Rectangle) -> i64 {
    rectangle.width * rectangle.height
}

pub fn print_structs() {
    let rect1: Rectangle = Rectangle { width: 30, height: 50 };
    
    println!("The value of rect1 is {:#?}", rect1);
}

pub fn debug_rect() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

pub fn struct_method_test() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
      "The width of the rectangle is greater than 0 pixels: {}.", 
      rect1.width()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

pub fn test_can_hold() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}