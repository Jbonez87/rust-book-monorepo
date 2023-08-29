use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant: Asparagus = Asparagus {};
    println!("I'm growing {:?}!", plant);
}