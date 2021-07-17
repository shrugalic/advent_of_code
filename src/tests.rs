use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

const EXAMPLE1: &str = "dabAcCaCBAcCcaDA";

#[test]
fn test_are_same_char_different_case() {
    assert!(are_same_char_different_case('a', 'A'));
    assert!(are_same_char_different_case('A', 'a'));
    assert!(!are_same_char_different_case('a', 'b'));
}

#[test]
fn example1_part1() {
    assert_eq!(
        10,
        remaining_units_after_reaction(&read_str_to_lines(EXAMPLE1)[0])
    );
}

#[test]
fn part1() {
    assert_eq!(
        9462,
        remaining_units_after_reaction(&read_file_to_lines("input.txt")[0])
    );
}

#[test]
fn example1_part2() {
    assert_eq!(
        4,
        length_of_shortest_possible_polymer(&read_str_to_lines(EXAMPLE1)[0])
    );
}

#[test]
fn part2() {
    assert_eq!(
        4952,
        length_of_shortest_possible_polymer(&read_file_to_lines("input.txt")[0])
    );
}
