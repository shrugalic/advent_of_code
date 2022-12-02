use crate::day01::{day01_part1, day01_part2};
mod day01;
fn day01() {
    assert_eq!(72_240, day01_part1());
    assert_eq!(210_957, day01_part2());
}

use crate::day02::{day02_part1, day02_part2};
mod day02;
fn day02() {
    assert_eq!(12_276, day02_part1());
    assert_eq!(9_975, day02_part2());
}

fn main() {
    day01();
    day02();
}
