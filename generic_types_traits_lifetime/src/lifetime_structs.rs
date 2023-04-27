/*
  This enables us to define a struct with a reference 
  using a lifetime annotation.
 */
#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
  pub part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
  pub fn level(&self) -> i32 {
    3
  }
}