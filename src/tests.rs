use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

const EXAMPLE1: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

#[test]
fn claim_from_string() {
    assert_eq!(
        Claim::from("#123 @ 3,2: 5x4"),
        Claim {
            id: 123,
            from_left: 3,
            from_top: 2,
            width: 5,
            height: 4,
        }
    );
}

#[test]
fn example_1() {
    assert_eq!(overlapping_claim_count(&read_str_to_lines(EXAMPLE1)), 4);
}

#[test]
fn part_1() {
    assert_eq!(
        overlapping_claim_count(&read_file_to_lines("input.txt")),
        113576
    );
}

#[test]
fn example_1_part_2() {
    assert_eq!(id_of_non_overlapping_claim(&read_str_to_lines(EXAMPLE1)), 3);
}

#[test]
fn part_2() {
    assert_eq!(
        id_of_non_overlapping_claim(&read_file_to_lines("input.txt")),
        825
    );
}
