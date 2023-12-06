use std::fmt::Display;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    print_result(1, day01::part1(), day01::part2());
    print_result(2, day02::part1(), day02::part2());
    print_result(3, day03::part1(), day03::part2());
    print_result(4, day04::part1(), day04::part2());
    print_result(5, day05::part1(), day05::part2());
    print_result(6, day06::part1(), day06::part2());
}

fn print_result(day: i32, part1: impl Display, part2: impl Display) {
    println!("2023 day {day}:\n- part 1: {part1}\n- part 2: {part2}");
}
