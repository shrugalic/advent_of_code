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

mod day07;
use crate::day07::{day07_part1, day07_part2};
fn day07() {
    assert_eq!(105, day07_part1());
    assert_eq!(258, day07_part2());
}

mod day08;
use crate::day08::{day08_part1, day08_part2};
fn day08() {
    assert_eq!(110, day08_part1());
    assert_eq!("ZJHRKCPLYJ", day08_part2());
}

mod day09;
use crate::day09::{day09_part1, day09_part2};
fn day09() {
    assert_eq!(112_830, day09_part1());
    assert_eq!(10_931_789_799, day09_part2());
}

mod day10;
use crate::day10::{day10_part1, day10_part2};
fn day10() {
    assert_eq!(86, day10_part1());
    assert_eq!(67 * 11 * 31, day10_part2());
}

mod day11;
use crate::day11::{day11_part1, day11_part2};
fn day11() {
    assert_eq!(37, day11_part1());
    assert_eq!(61, day11_part2());
}

mod day12;
use crate::day12::{day12_part1, day12_part2};
fn day12() {
    assert_eq!(318_003, day12_part1());
    assert_eq!(9_227_657, day12_part2());
}

mod day13;
use crate::day13::{day13_part1, day13_part2};
fn day13() {
    assert_eq!(92, day13_part1());
    assert_eq!(124, day13_part2());
}

mod day14;
use crate::day14::{day14_part1, day14_part2};
fn day14() {
    assert_eq!(23_890, day14_part1());
    assert_eq!(22_696, day14_part2());
}

mod day15;
use crate::day15::{day15_part1, day15_part2};
fn day15() {
    assert_eq!(376_777, day15_part1());
    assert_eq!(3_903_937, day15_part2());
}

mod day16;
use crate::day16::{day16_part1, day16_part2};
fn day16() {
    assert_eq!("10100011010101011", day16_part1());
    assert_eq!("01010001101011001", day16_part2());
}

mod day17;
use crate::day17::{day17_part1, day17_part2};
fn day17() {
    assert_eq!("DDRUDLRRRD", day17_part1());
    assert_eq!(398, day17_part2());
}

mod day18;
use crate::day18::{day18_part1, day18_part2};
fn day18() {
    assert_eq!(2016, day18_part1());
    assert_eq!(19_998_750, day18_part2());
}

mod day19;
use crate::day19::{day19_part1, day19_part2};
fn day19() {
    assert_eq!(1_815_603, day19_part1());
    assert_eq!(1_410_630, day19_part2());
}

mod day20;
use crate::day20::{day20_part1, day20_part2};
fn day20() {
    assert_eq!(22_887_907, day20_part1());
    assert_eq!(109, day20_part2());
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
