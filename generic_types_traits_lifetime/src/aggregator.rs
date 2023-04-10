use std::fmt::{Display, Debug};

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
  fn summarize_author(&self) -> String;
  // We can add a default implementation of `summarize`.
  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
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
  fn summarize_author(&self) -> String {
      format!("{}", self.author)
  }
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
  of the `summarize` method. The output of `Tweet.summarize()` 
  would then be "Read more...".
 */
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
  /*
    Commenting this out will allow Tweet to use the default
    implementation of summarize which calls summarize_author.
   */
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

/*
  This function uses a reference to the `Summary` trait as a parameter.
 */
pub fn trait_parameter_notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

/*
  This function uses Trait Bound Syntax using the `Summary` trait
  and a dynamic type T as a reference.
 */
pub fn trait_bound_notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

/*
  If we wanted to use multiple parameters using trait parameter
  syntax, we could define notify like this:

  pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
 */

/*
  If we wanted to use multiple parameters using trait bound syntax,
  we could define notify like this:

  pub fn notify<T: Summary>(item1: &T, item2: &T) {}
 */

pub fn multiple_trait_bound_notify(item: &(impl Summary + Display)) {
  println!("Breaking news! {}", item.summarize());
}

pub fn alt_multiple_trait_notify<T: Summary + Display>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

/*
  We can also use the where clause for trait bound function signatures.
  This is useful when we're trying to ensure the function signature is
  readable.
 */

fn where_clause_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{
  5
}

/*
  We can set the return type to implement a trait as well.
 */

pub fn return_summarizable() -> impl Summary {
  NewsArticle {
        headline: String::from("UCONN wins NCAA Championship!"),
        location: String::from("Storrs, CT, USA"),
        author: String::from("John Castrillon"),
        content: String::from("The UCONN Huskies won their 5th national championship!")
  }
}