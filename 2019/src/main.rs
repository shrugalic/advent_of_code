#[macro_use]
extern crate lazy_static;

use crate::day01::day1_part2;
mod day01;
fn day01() {
    assert_eq!(4943994, day1_part2());
}

use crate::day02::{day2_part1, day2_part2};
mod day02;
fn day02() {
    assert_eq!(3516593, day2_part1());
    assert_eq!(7749, day2_part2());
}

use crate::day03::{day3_part1, day3_part2};
mod day03;
fn day03() {
    assert_eq!(266, day3_part1());
    assert_eq!(19242, day3_part2());
}

use crate::day04::day4_part2;
mod day04;
fn day04() {
    assert_eq!(1135, day4_part2());
}

mod day05;
use crate::day05::{day5_part1, day5_part2};
fn day05() {
    assert_eq!(day5_part1(), Some(11049715));
    assert_eq!(day5_part2(), Some(2140710));
}

mod day06;
use crate::day06::{day6_part1, day6_part2, OrbitCount};
fn day06() {
    assert_eq!(day6_part1(), OrbitCount::from(1605, 252842));
    assert_eq!(day6_part2(), 445);
}

mod day07;
use crate::day07::{day7_part1, day7_part2};
fn day07() {
    assert_eq!(day7_part1(), 87138);
    assert_eq!(day7_part2(), 17279674);
}

mod day08;
use crate::day08::{day8_part1, day8_part2};
fn day08() {
    assert_eq!(day8_part1(), 2048);
    assert_eq!(day8_part2(), "100101111010001011001001010010100001000110010101001111011100010101001011000100101000000100111101010010010100000010010010101001001010000001001001010010");
}

mod day09;
use crate::day09::{day9_part1, day9_part2};
fn day09() {
    assert_eq!(day9_part1(), Some(3518157894));
    assert_eq!(day9_part2(), Some(80379));
}

mod day10;
use crate::day10::{day10_part1, day10_part2, Point};
fn day10() {
    assert_eq!(day10_part1(), 253);
    assert_eq!(day10_part2(), Point(8, 15));
}

mod day11;
use crate::day11::{day11_part1, day11_part2};
fn day11() {
    assert_eq!(day11_part1(), 2373);
    assert_eq!(day11_part2(), 249);
}

mod day12;
use crate::day12::{day12_part1, day12_part2};
fn day12() {
    assert_eq!(day12_part1(), 14907);
    assert_eq!(day12_part2(), 467_081_194_429_464);
}

mod day13;
use crate::day13::{day13_part1, day13_part2};
fn day13() {
    assert_eq!(day13_part1(), 265);
    assert_eq!(day13_part2(), 26947); // Score 13331
}

mod day14;
use crate::day14::{day14_part1, day14_part2};
fn day14() {
    assert_eq!(day14_part1(), 158482);
    assert_eq!(day14_part2(), 7993831);
}

mod day16;
use crate::day16::{day16_part1, day16_part2};
fn day16() {
    assert_eq!(day16_part1(), "78009100");
    assert_eq!(day16_part2(), "37717791");
}

mod day18;
use crate::day18::{day18_part1, day18_part2};
fn day18() {
    assert_eq!(3270, day18_part1()); // Slow, ~25s
    assert_eq!(1628, day18_part2()); // Very slow, ~9min
}

mod day19;
use crate::day19::{day19_part1, day19_part2};
fn day19() {
    assert_eq!(118, day19_part1());
    assert_eq!(18651593, day19_part2());
}

mod day20;
use crate::day20::{day20_part1, day20_part2};
fn day20() {
    assert_eq!(686, day20_part1());
    assert_eq!(8384, day20_part2());
}

mod day21;
use crate::day21::{day21_part1, day21_part2};
fn day21() {
    assert_eq!(19355364, day21_part1());
    assert_eq!(1142530574, day21_part2());
}

mod day22;
use crate::day22::{day22_part1, day22_part2};
fn day22() {
    assert_eq!(2519, day22_part1());
    assert_eq!(58966729050483, day22_part2());
}

mod day23;
use crate::day23::{day23_part1, day23_part2};
fn day23() {
    assert_eq!(20764, day23_part1());
    assert_eq!(14805, day23_part2());
}

mod day24;
use crate::day24::{day24_part1, day24_part2};
fn day24() {
    assert_eq!(27777901, day24_part1());
    assert_eq!(2047, day24_part2());
}

mod day25;
use crate::day25::day25_part1;
fn day25() {
    assert_eq!(35717128, day25_part1());
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
    // 15 is in its own project
    day16();
    // 17 is in its own project
    day18();
    day19();
    day20();
    day21();
    day22();
    day23();
    day24();
    day25();
}
