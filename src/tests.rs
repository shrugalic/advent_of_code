use crate::cumulate_frequency_adjustments;
use line_reader::{read_file_to_lines, read_str_to_lines};

const EXAMPLE_1: &str = "1
-2
+3
+1";

#[test]
fn example_1() {
    assert_eq!(
        cumulate_frequency_adjustments(&read_str_to_lines(EXAMPLE_1)),
        3
    );
}

#[test]
fn part_1() {
    assert_eq!(
        cumulate_frequency_adjustments(&read_file_to_lines("input.txt")),
        454
    );
}
