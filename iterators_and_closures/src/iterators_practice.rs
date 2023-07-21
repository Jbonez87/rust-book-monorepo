pub fn iterators_example() {
  let v1: Vec<i32> = vec![1,2,3];
  let v1_iter: std::slice::Iter<'_, i32> = v1.iter();

  println!("{:?}", v1_iter);
}