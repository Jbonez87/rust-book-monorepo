mod function_abstraction;
mod generics_example;

use function_abstraction::{code_duplication, non_duplicate_code};
use generics_example::{generics_practice};
fn main() {
    code_duplication();
    non_duplicate_code();
    generics_practice();
}
