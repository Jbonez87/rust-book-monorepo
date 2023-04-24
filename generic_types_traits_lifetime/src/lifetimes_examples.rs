/*
  This function will not work because `x` is only defined and
  referenced in the inner scope. `r` is defined in the outer scope
  and lives longer than `x` which causes the compiler to throw an error.
*/
/*
pub fn dangling_reference_example() {
  let r;

  {
    let x = 10;
    r = &x;
  }

  println!("The value of r is: {}", r);
}
*/

pub fn correct_reference() {
  let r;

  let x = 10;
  r = &x;

  println!("The value of r is: {}", r);
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

/*
  The following version of `longest` will compile since
  lifetime 'a is in the function signature and in the `x`
  parameter reference as well as the return type. Since it
  returns `x`, the `y` parameter is not included in the
  lifetime annotation.

  pub fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
  }
 */

/*
  However, this version of longest will fail at compile time due
  to the fact that it will create a dangling reference. The result
  variable is not related to the lifetime annotation value at all.

  pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("This is a very long string");
    result.as_str()
  }
 */
