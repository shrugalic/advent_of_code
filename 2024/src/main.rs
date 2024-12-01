use std::fmt::Display;

mod day01;

fn main() {
    print_result(1, day01::part1(), day01::part2());
}

fn print_result(day: i32, part1: impl Display, part2: impl Display) {
    println!("2024 day {day}:\n- part 1: {part1}\n- part 2: {part2}");
}