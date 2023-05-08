/*
  We don't actually need to use lifetime annotations
  here, but it's important to understand the reason why.
  The Rust Compiler is smart enough to determine
  the lifetimes of the parameter and return type here.
  This works because we only have one input lifetime parameter.
 */
pub fn first_word<'a>(s: &'a str) -> &'a str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}