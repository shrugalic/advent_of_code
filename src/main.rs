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

use crate::day05::{day05_part1, day05_part2};
mod day05;
fn day05() {
    assert_eq!("RTGWZTHLD", day05_part1());
    assert_eq!("STHGRZZFR", day05_part2());
}

use crate::day06::{day06_part1, day06_part2};
mod day06;
fn day06() {
    assert_eq!(1_876, day06_part1());
    assert_eq!(2_202, day06_part2());
}

use crate::day07::{day07_part1, day07_part2};
mod day07;
fn day07() {
    assert_eq!(1_543_140, day07_part1());
    assert_eq!(1_117_448, day07_part2());
}

use crate::day08::{day08_part1, day08_part2};
mod day08;
fn day08() {
    assert_eq!(1_832, day08_part1());
    assert_eq!(157_320, day08_part2());
}

use crate::day09::{day09_part1, day09_part2};
mod day09;
fn day09() {
    assert_eq!(6_311, day09_part1());
    assert_eq!(2_482, day09_part2());
}

use crate::day10::{day10_part1, day10_part2, PART2_RESULT_IMAGE};
mod day10;
fn day10() {
    assert_eq!(14_780, day10_part1());
    assert_eq!(PART2_RESULT_IMAGE, day10_part2());
}

use crate::day11::{day11_part1, day11_part2};
mod day11;
fn day11() {
    assert_eq!(54_054, day11_part1());
    assert_eq!(14_314_925_001, day11_part2());
}

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    day09();
    day10();
    day11();
}
