pub fn iterators_example() {
  /*
    Iterators are lazy by default and don't do anything unless consumed
    by calling one of their methods.
   */
  let v1: Vec<i32> = vec![1,2,3];
  let v1_iter: std::slice::Iter<'_, i32> = v1.iter();

  println!("{:?}", v1_iter);

  // We can use a variable for the iter() in a for loop.
  for val in v1_iter {
    println!("Got {}", val);
  }

  let v2: Vec<i32> = vec![4,5,6];
  let mapped_vec: Vec<_> = v2.iter().map(|x| x * 3).collect();

  println!("mapped_vec value is: {:?}", mapped_vec);
  assert_eq!(v2, vec![12, 15, 18]);

  let v1: Vec<i32> = vec![3,6,9];
  v1.iter().map(|x: &i32| x + 1);
}

/*
  All iterators implement the Iterator trait defined in the
  standard library. Below is the trait definition:

  pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item> 
  }

  They have methods with default implementations elided.
*/

