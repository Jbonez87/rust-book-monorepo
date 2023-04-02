/*
  Traits are similar to `interfaces` in other languages
  like TypeScript, Go or Java. There are some subtle
  differences though.
 */

pub trait Summary {
  pub fn summarize(&self) -> String;
}