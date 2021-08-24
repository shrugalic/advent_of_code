use crate::day01::{day1_part1, day1_part2};
use crate::day02::{day2_part1, day2_part2};
use crate::day03::{day3_part1, day3_part2};

mod day01;
mod day02;
mod day03;

fn main() {
    day01();
    day02();
    day03();
}

fn day01() {
    assert_eq!(1144, day1_part1());
    assert_eq!(1194, day1_part2());
}

fn day02() {
    assert_eq!(47136, day2_part1());
    assert_eq!(250, day2_part2());
}

fn day03() {
    assert_eq!(326, day3_part1());
    assert_eq!(363010, day3_part2());
}
