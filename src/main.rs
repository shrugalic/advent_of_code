use crate::day01::{day01_part1, day01_part2};
mod day01;
fn day01() {
    assert_eq!(72_240, day01_part1());
    assert_eq!(210_957, day01_part2());
}

fn main() {
    day01();
}
