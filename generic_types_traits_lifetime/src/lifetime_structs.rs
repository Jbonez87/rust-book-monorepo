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
  pub fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}