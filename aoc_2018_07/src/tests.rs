use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

#[test]
fn test_to_index() {
    assert_eq!(0, 'A'.to_index());
}

#[test]
fn test_to_step() {
    assert_eq!('A', 0u8.to_step());
}

const EXAMPLE1: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

#[test]
fn example_1() {
    assert_eq!("CABDFE", order_of_steps(&read_str_to_lines(EXAMPLE1)));
}
#[test]
fn part_1() {
    assert_eq!(
        "JNOIKSYABEQRUVWXGTZFDMHLPC",
        order_of_steps(&read_file_to_lines("input.txt"))
    );
}
