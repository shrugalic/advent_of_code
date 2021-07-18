use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

const EXAMPLE1: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

#[test]
fn part_1_example_1() {
    assert_eq!(240, strategy_one(&read_str_to_lines(EXAMPLE1)));
}

#[test]
fn part_1() {
    assert_eq!(65489, strategy_one(&read_file_to_lines("input.txt")));
}

#[test]
fn part_2_example_1() {
    assert_eq!(4455, strategy_two(&read_str_to_lines(EXAMPLE1)));
}

#[test]
fn part_2() {
    assert_eq!(3852, strategy_two(&read_file_to_lines("input.txt")));
}
