use crate::day08::{input_metadata_sum, input_value};
use crate::day09::high_score;
use crate::day10::message;
use crate::day11::{largest_total_power_3x3_square, largest_total_power_variable_size_square};
use crate::day12::{number_of_plants_after_20_gens, number_of_plants_after_generations};
use line_reader::read_file_to_lines;

mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    day08();
    day09();
    day10();
    day11();
    day12();
}

fn day08() {
    assert_eq!(
        42146,
        input_metadata_sum(&read_file_to_lines("input/day08.txt")[0])
    );
    assert_eq!(
        26753,
        input_value(&read_file_to_lines("input/day08.txt")[0])
    );
}

fn day09() {
    assert_eq!(374690, high_score(477, 70851));
    assert_eq!(3_009_951_158, high_score(477, 7_085_100));
}

fn day10() {
    let input = &read_file_to_lines("input/day10.txt");
    assert_eq!(10511, message(input).1);
}

fn day11() {
    assert_eq!((28, 235, 87), largest_total_power_3x3_square(8199));
    assert_eq!(
        (119, 234, 272, 18),
        largest_total_power_variable_size_square(8199)
    );
}

fn day12() {
    assert_eq!(
        325,
        number_of_plants_after_20_gens(&read_file_to_lines("input/day12.txt"))
    );
    assert_eq!(
        1_600_000_000_328,
        number_of_plants_after_generations(&read_file_to_lines("input/day12.txt"), 50_000_000_000)
    );
}
