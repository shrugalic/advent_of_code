use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

#[test]
fn count_one_of_each() {
    assert_eq!(
        count_2_and_3_identical_letters(&"bababc".to_string()),
        (1, 1)
    );
}

const EXAMPLE1: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";

#[test]
fn example_1() {
    assert_eq!(product_of_2_and_3_counts(&read_str_to_lines(EXAMPLE1)), 12);
}

#[test]
fn part_1() {
    assert_eq!(
        product_of_2_and_3_counts(&read_file_to_lines("input.txt")),
        7936
    );
}

const EXAMPLE2: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";

#[test]
fn example_2() {
    assert_eq!(
        differing_letters_of_correct_boxes(&read_str_to_lines(EXAMPLE2)),
        "fgij"
    );
}

#[test]
fn part_2() {
    assert_eq!(
        differing_letters_of_correct_boxes(&read_file_to_lines("input.txt")),
        "lnfqdscwjyteorambzuchrgpx"
    );
}
