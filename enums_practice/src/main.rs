mod ips;
mod messenger_test;
mod option_example;
mod coins;
mod option_matching;
mod if_let_practice;

use ips::{IpAddrKind, test_ips};
use messenger_test::{test_message};
use option_example::{option_test};
use coins::{Coin, UsState, value_in_cents};
use option_matching::plus_one;
use if_let_practice::if_let_example;

fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("Sum is: {sum}");
    test_ips();
    let ip_kind = IpAddrKind::V4;
    route(ip_kind);
    test_message();
    option_test();
    let quarter: Coin = Coin::Quarter(UsState::Hawaii);
    let penny: Coin = Coin::Penny;
    value_in_cents(quarter);
    value_in_cents(penny);
    let five: Option<i64> = Some(5);
    let negative_five: Option<i64> = Some(-5);
    plus_one(five);
    plus_one(None);
    plus_one(negative_five);
    if_let_example();
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}
