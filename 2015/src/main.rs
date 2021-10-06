mod day01;
mod permutation;
use crate::day01::{day01_part1, day01_part2};
fn day01() {
    assert_eq!(280, day01_part1());
    assert_eq!(1797, day01_part2());
}

mod day02;
use crate::day02::{day02_part1, day02_part2};
fn day02() {
    assert_eq!(1588178, day02_part1());
    assert_eq!(3783758, day02_part2());
}

mod day03;
use crate::day03::{day03_part1, day03_part2};
fn day03() {
    assert_eq!(2565, day03_part1());
    assert_eq!(2639, day03_part2());
}

mod day04;
use crate::day04::{day04_part1, day04_part2};
fn day04() {
    assert_eq!(282749, day04_part1());
    assert_eq!(9962624, day04_part2());
}

mod day05;
use crate::day05::{day05_part1, day05_part2};
fn day05() {
    assert_eq!(238, day05_part1());
    assert_eq!(69, day05_part2());
}

mod day06;
use crate::day06::{day06_part1, day06_part2};
fn day06() {
    assert_eq!(400410, day06_part1());
    assert_eq!(15343601, day06_part2());
}

mod day07;
use crate::day07::{day07_part1, day07_part2};
fn day07() {
    assert_eq!(46065, day07_part1());
    assert_eq!(14134, day07_part2());
}

mod day08;
use crate::day08::{day08_part1, day08_part2};
fn day08() {
    assert_eq!(1371, day08_part1());
    assert_eq!(2117, day08_part2());
}

mod day09;
use crate::day09::{day09_part1, day09_part2};
fn day09() {
    assert_eq!(141, day09_part1());
    assert_eq!(736, day09_part2());
}

mod day10;
use crate::day10::{day10_part1, day10_part2};
fn day10() {
    assert_eq!(492982, day10_part1());
    assert_eq!(6989950, day10_part2());
}

mod day11;
use crate::day11::{day11_part1, day11_part2};
fn day11() {
    assert_eq!("vzbxxyzz", day11_part1());
    assert_eq!("vzcaabcc", day11_part2());
}

mod day12;
use crate::day12::{day12_part1, day12_part2};
fn day12() {
    assert_eq!(119433, day12_part1());
    assert_eq!(68466, day12_part2());
}

mod day13;
use crate::day13::{day13_part1, day13_part2};
fn day13() {
    assert_eq!(733, day13_part1());
    assert_eq!(725, day13_part2());
}

mod day14;
use crate::day14::{day14_part1, day14_part2};
fn day14() {
    assert_eq!(2655, day14_part1());
    assert_eq!(1059, day14_part2());
}

mod day15;
use crate::day15::{day15_part1, day15_part2};
fn day15() {
    assert_eq!(13882464, day15_part1());
    assert_eq!(11171160, day15_part2());
}

mod day16;
use crate::day16::{day16_part1, day16_part2};
fn day16() {
    assert_eq!(373, day16_part1());
    assert_eq!(260, day16_part2());
}

mod day17;
use crate::day17::{day17_part1, day17_part2};
fn day17() {
    assert_eq!(654, day17_part1());
    assert_eq!(57, day17_part2());
}

mod day18;
use crate::day18::{day18_part1, day18_part2};
fn day18() {
    assert_eq!(768, day18_part1());
    assert_eq!(781, day18_part2());
}

mod day19;
use crate::day19::{day19_part1, day19_part2};
fn day19() {
    assert_eq!(535, day19_part1());
    assert_eq!(212, day19_part2());
}

mod day20;
use crate::day20::{day20_part1, day20_part2};
fn day20() {
    assert_eq!(665_280, day20_part1());
    assert_eq!(705_600, day20_part2());
}

mod day21;
use crate::day21::{day21_part1, day21_part2};
fn day21() {
    assert_eq!(91, day21_part1());
    assert_eq!(158, day21_part2());
}

mod day22;
use crate::day22::{day22_part1, day22_part2};
fn day22() {
    assert_eq!(953, day22_part1());
    assert_eq!(1289, day22_part2());
}

mod day23;
use crate::day23::{day23_part1, day23_part2};
fn day23() {
    assert_eq!(255, day23_part1());
    assert_eq!(334, day23_part2());
}

mod day24;
use crate::day24::{day24_part1, day24_part2};
fn day24() {
    assert_eq!(11_846_773_891, day24_part1());
    assert_eq!(80_393_059, day24_part2());
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
}
