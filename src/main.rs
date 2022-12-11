mod device;
mod opcode;

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

use crate::day01::{day1_part1, day1_part2};
mod day01;
fn day01() {
    assert_eq!(day1_part1(), 454);
    assert_eq!(day1_part2(), 566);
}

use crate::day02::{day2_part1, day2_part2};
mod day02;
fn day02() {
    assert_eq!(day2_part1(), 7_936);
    assert_eq!(day2_part2(), "lnfqdscwjyteorambzuchrgpx");
}

use crate::day03::{day3_part1, day3_part2};
mod day03;
fn day03() {
    assert_eq!(day3_part1(), 11_3576);
    assert_eq!(day3_part2(), 825);
}

use crate::day04::{day4_part1, day4_part2};
mod day04;
fn day04() {
    assert_eq!(65_489, day4_part1());
    assert_eq!(3_852, day4_part2());
}

use crate::day05::{day5_part1, day5_part2};
mod day05;
fn day05() {
    assert_eq!(9_462, day5_part1());
    assert_eq!(4_952, day5_part2());
}

use crate::day06::{day6_part1, day6_part2};
mod day06;
fn day06() {
    assert_eq!(4_589, day6_part1());
    assert_eq!(40_252, day6_part2());
}

use crate::day07::{day7_part1, day7_part2};
mod day07;
fn day07() {
    assert_eq!("JNOIKSYABEQRUVWXGTZFDMHLPC", day7_part1());
    assert_eq!(1_099, day7_part2());
}

use crate::day08::{day8_part1, day8_part2};
mod day08;
fn day08() {
    assert_eq!(42_146, day8_part1());
    assert_eq!(26_753, day8_part2());
}

use crate::day09::{day9_part1, day9_part2};
mod day09;
fn day09() {
    assert_eq!(374_690, day9_part1());
    assert_eq!(3_009_951_158, day9_part2());
}

use crate::day10::day10_part1;
mod day10;
fn day10() {
    assert_eq!(10511, day10_part1());
}

use crate::day11::{day11_part1, day11_part2};
mod day11;
fn day11() {
    assert_eq!((28, 235, 87), day11_part1());
    assert_eq!((119, 234, 272, 18), day11_part2());
}

use crate::day12::{day12_part1, day12_part2};
mod day12;
fn day12() {
    assert_eq!(2_063, day12_part1());
    assert_eq!(1_600_000_000_328, day12_part2());
}

use crate::day13::{day13_part1, day13_part2};
mod day13;
fn day13() {
    assert_eq!((102, 114), day13_part1());
    assert_eq!((146, 87), day13_part2());
}

use crate::day14::{day14_part1, day14_part2};
mod day14;
fn day14() {
    assert_eq!("1411383621", day14_part1());
    assert_eq!(20_177_474, day14_part2());
}

use crate::day15::{day15_part1, day15_part2};
mod day15;
fn day15() {
    assert_eq!(207_059, day15_part1());
    assert_eq!(49_120, day15_part2());
}

use crate::day16::{day16_part1, day16_part2};
mod day16;
fn day16() {
    assert_eq!(605, day16_part1());
    assert_eq!(653, day16_part2());
}

use crate::day17::{day17_part1, day17_part2};
mod day17;
fn day17() {
    assert_eq!(31_949, day17_part1());
    assert_eq!(26_384, day17_part2());
}

use crate::day18::{day18_part1, day18_part2};
mod day18;
fn day18() {
    assert_eq!(605_154, day18_part1());
    assert_eq!(200_364, day18_part2());
}

use crate::day19::{day19_part1, day19_part2};
mod day19;
fn day19() {
    assert_eq!(1872, day19_part1());
    assert_eq!(
        18_992_592, // 1 + 2 + 5 + 10 + 1_055_143 + 2_110_286 + 5_275_715 + 10_551_430,
        day19_part2()
    );
}

use crate::day20::{day20_part1, day20_part2};
mod day20;
fn day20() {
    assert_eq!(4_360, day20_part1());
    assert_eq!(8_509, day20_part2());
}

use crate::day21::{day21_part1, day21_part2};
mod day21;
fn day21() {
    assert_eq!(103_548, day21_part1());
    assert_eq!(14_256_686, day21_part2());
}

use crate::day22::{day22_part1, day22_part2};
mod day22;
fn day22() {
    assert_eq!(10_115, day22_part1());
    assert_eq!(990, day22_part2());
}

use crate::day23::{day23_part1, day23_part2};
mod day23;
fn day23() {
    assert_eq!(417, day23_part1());
    assert_eq!(112_997_634, day23_part2());
}

use crate::day24::{day24_part1, day24_part2};
mod day24;
fn day24() {
    assert_eq!(
        3186 + 1252 + 2241 + 2590 + 1650 + 7766 + 1790 + 264 + 2257, // 22996
        day24_part1()
    );
    assert_eq!(935 + 857 + 2535 /* 4327 */, day24_part2());
}

use crate::day25::day25_part1;
mod day25;
fn day25() {
    assert_eq!(399, day25_part1());
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
    day21();
    day22();
    day23();
    day24();
    day25();
}
