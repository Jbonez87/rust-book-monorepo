pub fn iterators_example() {
  let v1: Vec<i32> = vec![1,2,3];
  let v1_iter: std::slice::Iter<'_, i32> = v1.iter();

  println!("{:?}", v1_iter);

  for val in v1_iter {
    println!("Got {}", val);
  }
}

/*
  All iterators implement the Iterator trait defined in the
  standard library. Below is the trait definition:

  pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item> 
  }

  Methods with default implementations elided.
*/