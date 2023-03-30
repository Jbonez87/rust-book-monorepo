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
  will be able to use this method.
 */
impl SingleTypePoint<f32> {
  pub fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}