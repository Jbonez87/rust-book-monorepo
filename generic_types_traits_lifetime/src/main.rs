mod function_abstraction;
mod generics_example;
mod generic_structs;

use function_abstraction::{code_duplication, non_duplicate_code};
use generics_example::{generics_practice};
use generic_structs::{Point};

fn main() {
    code_duplication();
    non_duplicate_code();
    generics_practice();
    let integer = Point { x: 5, y: 4 };
    println!("Integer is: {:?}", integer);

    let float = Point { x: 7.5, y: 3.8 };
    println!("Float is: {:?}", float);

    /* 
      This will not work due to the mismatched integer and float types.
      In the struct definition, <T> lets the compiler know that both
      `x` and `y` should be the same type.
    */
    /* let wont_work = Point { x: 3, y: 7.5 }; */

    let both_integer = Point { x: 3, y: 7 };
    let both_float = Point { x: 7.8, y: 3.3 };
    let integer_and_float = Point { x: 4, y: 3.4 };
}
