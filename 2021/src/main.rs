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

fn main() {
    day01();
    day02();
    day03();
    day04();
}
