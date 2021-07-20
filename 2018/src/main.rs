use crate::day08::{input_metadata_sum, input_value};
use crate::day09::high_score;
use line_reader::read_file_to_lines;

mod day08;
mod day09;

fn main() {
    day08_part1();
}

pub fn day08_part1() {
    assert_eq!(
        42146,
        input_metadata_sum(&read_file_to_lines("input/day08.txt")[0])
    );
}

pub fn day08_part2() {
    assert_eq!(
        26753,
        input_value(&read_file_to_lines("input/day08.txt")[0])
    );
}

pub fn day09_part1() {
    assert_eq!(374690, high_score(477, 70851));
}
