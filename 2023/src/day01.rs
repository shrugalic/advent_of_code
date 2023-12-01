const INPUT: &str = include_str!("../input/day01.txt");

pub(crate) fn part1() -> u32 {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(number_from_first_and_last_digit)
        .sum()
}

fn number_from_first_and_last_digit(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn solve_part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(number_from_first_and_last_possibly_spelled_out_digit)
        .sum::<usize>()
}

fn number_from_first_and_last_possibly_spelled_out_digit(line: &str) -> usize {
    let string_value_pairs = [
        ("0", 0usize),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut left: usize = usize::MAX;
    let mut right: usize = usize::MIN;
    let mut first_digit = 0;
    let mut last_digit = 0;
    for (s, v) in string_value_pairs {
        if let Some(i) = line.find(s) {
            if i <= left {
                left = i;
                first_digit = v;
            }
        }
        if let Some(i) = line.rfind(s) {
            if i >= right {
                right = i;
                last_digit = v;
            }
        }
    }
    first_digit * 10 + last_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1_example() {
        assert_eq!(142, solve_part1(EXAMPLE1));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(281, solve_part2(EXAMPLE2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(54968, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(54094, solve_part2(INPUT));
    }
}
