mod day01;
use crate::day01::{day1_part1, day1_part2};
fn day01() {
    assert_eq!(1144, day1_part1());
    assert_eq!(1194, day1_part2());
}

mod day02;
use crate::day02::{day2_part1, day2_part2};
fn day02() {
    assert_eq!(47136, day2_part1());
    assert_eq!(250, day2_part2());
}

mod day03;
use crate::day03::{day3_part1, day3_part2};
fn day03() {
    assert_eq!(326, day3_part1());
    assert_eq!(363010, day3_part2());
}

mod day04;
use crate::day04::{day4_part1, day4_part2};
fn day04() {
    assert_eq!(451, day4_part1());
    assert_eq!(223, day4_part2());
}

mod day05;
use crate::day05::{day5_part1, day5_part2};
fn day05() {
    assert_eq!(375042, day5_part1());
    assert_eq!(28707598, day5_part2());
}

mod day06;
use crate::day06::{day6_part1, day6_part2};
fn day06() {
    assert_eq!(3156, day6_part1());
    assert_eq!(1610, day6_part2());
}

mod day07;
use crate::day07::{day7_part1, day7_part2};
fn day07() {
    assert_eq!("eqgvf", day7_part1());
    assert_eq!(757, day7_part2());
}

mod day08;
use crate::day08::{day8_part1, day8_part2};
fn day08() {
    assert_eq!(4902, day8_part1());
    assert_eq!(7037, day8_part2());
}

mod day09;
use crate::day09::{day9_part1, day9_part2};
fn day09() {
    assert_eq!(16827, day9_part1());
    assert_eq!(7298, day9_part2());
}

fn main() {
    day01();
    day02();
    day03();
    day04();
    day04();
    day05();
    day06();
    day07();
    day08();
    day09();
}
