mod iterators_practice;
mod closures_practice;

use iterators_practice::iterators_example;
use closures_practice::{closures_example};

fn main() {
    closures_example();

    iterators_example();

    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
        .zip(&buffer[i - 12..i])
        .map(|(&c, &s)| c * s as i64)
        .sum::<i64>() >> qlp_shift;

        let delta = buffer[i]
        buffer[i] = prediction as i32 + delta;

    }
}
