use crate::day01::{day01_part1, day01_part2};
mod day01;
fn day01() {
    assert_eq!(1475, day01_part1());
    assert_eq!(1516, day01_part2());
}

fn main() {
    day01();
}
