pub struct SingleTypePoint<T> {
  pub x: T,
  pub y: T
}

impl<T> SingleTypePoint<T> {
  pub fn x(&self) -> &T {
    &self.x
  }
}

/*
  This example uses a concrete type `f32` meaning
  only instances of `SingleTypePoint` that use `f32`
  will be able to use this method. Additionally, 
  since we're using a concrete type, we don't need
  to specify `impl<T>` when defining the method.
 */
impl SingleTypePoint<f64> {
  pub fn y(&self) -> f64 {
    self.y
  }
  pub fn distance_from_origin(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

pub struct DoubleTypePoint<X1, Y1> {
  pub x: X1,
  pub y: Y1
}

impl<X1, Y1> DoubleTypePoint<X1, Y1> {
  pub fn mixup<X2, Y2>(self, other: DoubleTypePoint<X2, Y2>) -> DoubleTypePoint<X1, Y2> {
    DoubleTypePoint {
      x: self.x,
      y: other.y,
    }
  }
}