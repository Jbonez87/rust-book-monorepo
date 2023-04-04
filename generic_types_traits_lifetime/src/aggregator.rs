/*
  Traits are similar to `interfaces` in other languages
  like TypeScript, Go or Java. There are some subtle
  differences though.
 */

/*
  The `summarize` method can be used on any type that uses
  the `Summary` trait.
 */
pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String
}

/*
  Note that when implementing a trait on a type, we need to 
  put the trait after the `impl`.
 */
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{} by {}, ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}