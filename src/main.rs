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

mod day04;
use crate::day04::{day04_part1, day04_part2};
fn day04() {
    assert_eq!(158835, day04_part1());
    assert_eq!(993, day04_part2());
}

mod day05;
use crate::day05::{day05_part1, day05_part2};
fn day05() {
    assert_eq!("801b56a7", day05_part1());
    assert_eq!("424a0197", day05_part2());
}

mod day06;
use crate::day06::{day06_part1, day06_part2};
fn day06() {
    assert_eq!("qtbjqiuq", day06_part1());
    assert_eq!("akothqli", day06_part2());
}

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
}
