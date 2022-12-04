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

use crate::day03::{day03_part1, day03_part2};
mod day03;
fn day03() {
    assert_eq!(7_763, day03_part1());
    assert_eq!(2_569, day03_part2());
}

use crate::day04::{day04_part1, day04_part2};
mod day04;
fn day04() {
    assert_eq!(507, day04_part1());
    assert_eq!(897, day04_part2());
}

fn main() {
    day01();
    day02();
    day03();
    day04();
}
