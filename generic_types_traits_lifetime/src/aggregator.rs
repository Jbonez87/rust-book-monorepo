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
  // We can add a default implementation of `summarize`.
  fn summarize(&self) -> String {
    String::from("Read more...")
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String
}

/*
  Note that when implementing a trait on a type, we need to 
  put the trait after the `impl` and then we need to use the `for`
  keyword followed by the type.
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

/*
  If we were to leave this `impl` block emoty for our `Tweet`
  struct, `Tweet` would use `Summary's` default implementation
  of the `summarize` method.
 */
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}