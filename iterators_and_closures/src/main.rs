mod iterators_practice;
mod closures_practice;

use iterators_practice::iterators_example;
use closures_practice::{closures_example, ShirtColor, Inventory, Rectangle};
use std::{thread, time::Duration};

fn main() {
    closures_example();

    iterators_example();

}
