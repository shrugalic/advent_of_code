mod day01;
use crate::day01::{day01_part1, day01_part2};
fn day01() {
    assert_eq!(230, day01_part1());
    assert_eq!(154, day01_part2());
}

mod day02;
use crate::day02::{day02_part1, day02_part2};
fn day02() {
    assert_eq!("99332", day02_part1());
    assert_eq!("DD483", day02_part2());
}

mod day03;
use crate::day03::{day03_part1, day03_part2};
fn day03() {
    assert_eq!(1050, day03_part1());
    assert_eq!(1921, day03_part2());
}

fn main() {
    day01();
    day02();
    day03();
}
