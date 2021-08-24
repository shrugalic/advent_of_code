use line_reader::read_file_to_lines;

pub(crate) fn day2_part1() -> usize {
    sum_of_differences_of_each_lines_max_and_min_number(read_file_to_lines("input/day02.txt"))
}

pub(crate) fn day2_part2() -> usize {
    sum_of_divisions_of_the_only_two_evenly_divisible_numbers(read_file_to_lines("input/day02.txt"))
}

fn sum_of_differences_of_each_lines_max_and_min_number(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| parse_line(line))
        .map(|numbers| difference_of_max_and_min_number(numbers))
        .sum()
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn difference_of_max_and_min_number(numbers: Vec<usize>) -> usize {
    numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
}

fn sum_of_divisions_of_the_only_two_evenly_divisible_numbers(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| parse_line(line))
        .map(|nums| divide_the_only_two_evenly_divisible_numbers_in(nums))
        .sum()
}

fn divide_the_only_two_evenly_divisible_numbers_in(numbers: Vec<usize>) -> usize {
    for a in &numbers {
        for b in &numbers {
            if a < b && b % a == 0 {
                return b / a;
            } else if a > b && a % b == 0 {
                return a / b;
            } else {
                // skip
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE_1: &str = "\
5 1 9 5
7 5 3
2 4 6 8";

    #[test]
    fn example1() {
        assert_eq!(
            18,
            sum_of_differences_of_each_lines_max_and_min_number(read_str_to_lines(EXAMPLE_1))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(47136, day2_part1());
    }
    const EXAMPLE_2: &str = "\
5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn example2() {
        assert_eq!(
            9,
            sum_of_divisions_of_the_only_two_evenly_divisible_numbers(read_str_to_lines(EXAMPLE_2))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(250, day2_part2());
    }
}
