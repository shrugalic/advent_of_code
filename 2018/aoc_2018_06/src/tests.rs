use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

const EXAMPLE1: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

#[test]
fn example1() {
    assert_eq!(17, size_of_largest_finite_area(read_str_to_lines(EXAMPLE1)));
}

#[test]
fn part1() {
    assert_eq!(
        4589,
        size_of_largest_finite_area(read_file_to_lines("input.txt"))
    );
}

#[test]
fn part2_example1() {
    assert_eq!(
        16,
        size_of_area_with_max_total_distance_to_all_coords(read_str_to_lines(EXAMPLE1), 32)
    );
}

#[test]
fn part2() {
    assert_eq!(
        40252,
        size_of_area_with_max_total_distance_to_all_coords(read_file_to_lines("input.txt"), 10_000)
    );
}
