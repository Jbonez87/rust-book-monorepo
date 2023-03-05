mod user;
mod calculator;
mod better_calculator;

use user::{build_user, run, mutable_user, inject_user, spread_inject_user, colors_points};

use better_calculator::{print_structs, debug_rect, struct_method_test, test_can_hold};

fn main() {
    println!("Hello, world!");
    run();
    mutable_user();

    let email: String = String::from("John@john.com");
    let username: String = String::from("Johnny");

    build_user(email, username);

    inject_user();

    spread_inject_user();

    colors_points();

    calculator::run();

    better_calculator::run();

    print_structs();

    debug_rect();

    struct_method_test();

    test_can_hold();
}
