/*
  Traits are similar to `interfaces` in other languages
  like TypeScript, Go or Java. There are some subtle
  differences though.
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

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{} by {}, ({})", self.headline, self.author, self.location)
  }
}