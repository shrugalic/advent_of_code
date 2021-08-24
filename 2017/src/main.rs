use crate::day01::{day1_part1, day1_part2};
use crate::day02::{day2_part1, day2_part2};

mod day01;
mod day02;

fn main() {
    day01();
    day02();
}

fn day01() {
    assert_eq!(1144, day1_part1());
    assert_eq!(1194, day1_part2());
}
fn day02() {
    assert_eq!(47136, day2_part1());
    assert_eq!(250, day2_part2());
}
