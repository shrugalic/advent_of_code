use crate::day08::input_metadata_sum;
use line_reader::read_file_to_lines;

mod day08;

fn main() {
    day08_part1();
}

fn day08_part1() {
    assert_eq!(
        42146,
        input_metadata_sum(&read_file_to_lines("input/day08.txt")[0])
    );
}
