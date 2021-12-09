use crate::day01::{day01_part1, day01_part2};
mod day01;
fn day01() {
    assert_eq!(1475, day01_part1());
    assert_eq!(1516, day01_part2());
}

use crate::day02::{day02_part1, day02_part2};
mod day02;
fn day02() {
    assert_eq!(2322630, day02_part1());
    assert_eq!(2105273490, day02_part2());
}

use crate::day03::{day03_part1, day03_part2};
mod day03;
fn day03() {
    assert_eq!(284 * 3811, day03_part1());
    assert_eq!(486 * 2784, day03_part2());
}

use crate::day04::{day04_part1, day04_part2};
mod day04;
fn day04() {
    assert_eq!(640 * 46, day04_part1());
    assert_eq!(267 * 52, day04_part2());
}

use crate::day05::{day05_part1, day05_part2};
mod day05;
fn day05() {
    assert_eq!(5197, day05_part1());
    assert_eq!(18605, day05_part2());
}

use crate::day06::{day06_part1, day06_part2};
mod day06;
fn day06() {
    assert_eq!(345_387, day06_part1());
    assert_eq!(1_574_445_493_136, day06_part2());
}

use crate::day07::{day07_part1, day07_part2};
mod day07;
fn day07() {
    assert_eq!(348_996, day07_part1());
    assert_eq!(98_231_647, day07_part2());
}

use crate::day08::{day08_part1, day08_part2};
mod day08;
fn day08() {
    assert_eq!(272, day08_part1());
    assert_eq!(1_007_675, day08_part2());
}

use crate::day09::{day09_part1, day09_part2};
mod day09;
fn day09() {
    assert_eq!(564, day09_part1());
    assert_eq!(1_038_240, day09_part2());
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
}
