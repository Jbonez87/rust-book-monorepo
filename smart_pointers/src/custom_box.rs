use std::ops::Deref;
pub struct MyBox<T>(T);

// this implements the Deref trait with our MyBox struct
impl<T> Deref for MyBox<T> {
    type Target = T;

    // our deref method returns self.0 which is the first element in our tuple.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
