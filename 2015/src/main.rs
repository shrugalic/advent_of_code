use crate::day01::{day1_part1, day1_part2};
use crate::day02::{day2_part1, day2_part2};

mod day01;
fn day01() {
    assert_eq!(280, day1_part1());
    assert_eq!(1797, day1_part2());
}

mod day02;
fn day02() {
    assert_eq!(1588178, day2_part1());
    assert_eq!(3783758, day2_part2());
}

fn main() {
    day01();
    day02();
}
