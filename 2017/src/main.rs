use crate::day01::{day1_part1, day1_part2};

mod day01;

fn main() {
    day01();
}

fn day01() {
    assert_eq!(1144, day1_part1());
    assert_eq!(1194, day1_part2());
}
