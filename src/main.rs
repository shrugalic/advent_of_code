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

mod day10;
use crate::day10::{day10_part1, day10_part2};
fn day10() {
    assert_eq!(212, day10_part1());
    assert_eq!("96de9657665675b51cd03f0b3528ba26", day10_part2());
}

mod day11;
use crate::day11::{day11_part1, day11_part2};
fn day11() {
    assert_eq!(722, day11_part1());
    assert_eq!(1551, day11_part2());
}

mod day12;
use crate::day12::{day12_part1, day12_part2};
fn day12() {
    assert_eq!(239, day12_part1());
    assert_eq!(215, day12_part2());
}

mod day13;
use crate::day13::{day13_part1, day13_part2};
fn day13() {
    assert_eq!(748, day13_part1());
    assert_eq!(3873662, day13_part2());
}

mod day14;
use crate::day14::{day14_part1, day14_part2};
fn day14() {
    assert_eq!(8190, day14_part1());
    assert_eq!(1134, day14_part2());
}

mod day15;
use crate::day15::{day15_part1, day15_part2};
fn day15() {
    assert_eq!(638, day15_part1());
    assert_eq!(343, day15_part2());
}

mod day16;
use crate::day16::{day16_part1, day16_part2};
fn day16() {
    assert_eq!("olgejankfhbmpidc", day16_part1());
    assert_eq!("gfabehpdojkcimnl", day16_part2());
}

mod day17;
use crate::day17::{day17_part1, day17_part2};
fn day17() {
    assert_eq!(180, day17_part1());
    assert_eq!(13326437, day17_part2());
}

mod day18;
use crate::day18::{day18_part1, day18_part2};
fn day18() {
    assert_eq!(2951, day18_part1());
    assert_eq!(7366, day18_part2());
}

mod day19;
use crate::day19::{day19_part1, day19_part2};
fn day19() {
    assert_eq!("MKXOIHZNBL", day19_part1());
    assert_eq!(17872, day19_part2());
}

mod day20;
use crate::day20::{day20_part1, day20_part2};
fn day20() {
    assert_eq!(0, day20_part1());
    assert_eq!(707, day20_part2());
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
    day10();
    day11();
    day12();
    day13();
    day14();
    day15();
    day16();
    day17();
    day18();
    day19();
    day20();
}
