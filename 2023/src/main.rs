use std::fmt::Display;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

fn main() {
    print_result(1, day01::part1(), day01::part2());
    print_result(2, day02::part1(), day02::part2());
    print_result(3, day03::part1(), day03::part2());
    print_result(4, day04::part1(), day04::part2());
    print_result(5, day05::part1(), day05::part2());
    print_result(6, day06::part1(), day06::part2());
    print_result(7, day07::part1(), day07::part2());
    print_result(8, day08::part1(), day08::part2());
    print_result(9, day09::part1(), day09::part2());
    print_result(10, day10::part1(), day10::part2());
    print_result(11, day11::part1(), day11::part2());
    print_result(12, day12::part1(), day12::part2());
    print_result(13, day13::part1(), day13::part2());
    print_result(14, day14::part1(), day14::part2());
    print_result(15, day15::part1(), day15::part2());
    print_result(16, day16::part1(), day16::part2());
    print_result(17, day17::part1(), day17::part2());
    print_result(18, day18::part1(), day18::part2());
    print_result(19, day19::part1(), day19::part2());
    print_result(20, day20::part1(), day20::part2());
    print_result(21, day21::part1(), day21::part2());
}

fn print_result(day: i32, part1: impl Display, part2: impl Display) {
    println!("2023 day {day}:\n- part 1: {part1}\n- part 2: {part2}");
}
