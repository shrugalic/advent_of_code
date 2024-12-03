use std::fmt::Display;

mod day01;
mod day02;
mod day03;

fn main() {
    print_result(1, day01::part1(), day01::part2());
    print_result(1, day02::part1(), day02::part2());
    print_result(1, day03::part1(), day03::part2());
}

fn print_result(day: i32, part1: impl Display, part2: impl Display) {
    println!("2024 day {day}:\n- part 1: {part1}\n- part 2: {part2}");
}