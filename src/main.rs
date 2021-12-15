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

use crate::day10::{day10_part1, day10_part2};
mod day10;
fn day10() {
    assert_eq!(319_329, day10_part1());
    assert_eq!(3_515_583_998, day10_part2());
}

use crate::day11::{day11_part1, day11_part2};
mod day11;
fn day11() {
    assert_eq!(1721, day11_part1());
    assert_eq!(298, day11_part2());
}

use crate::day12::{day12_part1, day12_part2};
mod day12;
fn day12() {
    assert_eq!(3708, day12_part1());
    assert_eq!(93_858, day12_part2());
}

use crate::day13::{day13_part1, day13_part2};
mod day13;
fn day13() {
    assert_eq!(781, day13_part1());
    assert_eq!("PERCGJPB", day13_part2());
}

use crate::day14::{day14_part1, day14_part2};
mod day14;
fn day14() {
    assert_eq!(2068, day14_part1());
    assert_eq!(2_158_894_777_814, day14_part2());
}

use crate::day15::{day15_part1, day15_part2};
mod day15;
fn day15() {
    assert_eq!(2068, day15_part1());
    assert_eq!(2_158_894_777_815, day15_part2());
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
}
