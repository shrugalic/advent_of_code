use line_reader::read_file_to_lines;

pub(crate) fn day1_part1() -> u32 {
    let line = read_file_to_lines("input/day01.txt").remove(0);
    solve_part1_captcha(line)
}

pub(crate) fn day1_part2() -> u32 {
    let line = read_file_to_lines("input/day01.txt").remove(0);
    solve_part2_captcha(line)
}

fn solve_part1_captcha<T: AsRef<str>>(line: T) -> u32 {
    solve_captcha(parse_input(line), 1)
}

fn solve_part2_captcha<T: AsRef<str>>(line: T) -> u32 {
    let numbers = parse_input(line);
    let offset = numbers.len() / 2;
    solve_captcha(numbers, offset)
}

fn solve_captcha(mut numbers: Vec<u32>, offset: usize) -> u32 {
    numbers.extend_from_within(0..offset);

    numbers
        .iter()
        .zip(numbers.iter().skip(offset))
        .filter_map(|(a, b)| {
            // println!("{} <> {}", a, b);
            if a == b {
                Some(a)
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn parse_input<T: AsRef<str>>(line: T) -> Vec<u32> {
    line.as_ref()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(3, solve_part1_captcha("1122"));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(4, solve_part1_captcha("1111"));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(0, solve_part1_captcha("1234"));
    }

    #[test]
    fn part1_example4() {
        assert_eq!(9, solve_part1_captcha("91212129"));
    }

    #[test]
    fn test_day1_part1() {
        assert_eq!(1144, day1_part1());
    }

    #[test]
    fn part2_example1() {
        assert_eq!(6, solve_part2_captcha("1212"));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(0, solve_part2_captcha("1221"));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(4, solve_part2_captcha("123425"));
    }

    #[test]
    fn part2_example4() {
        assert_eq!(12, solve_part2_captcha("123123"));
    }

    #[test]
    fn part2_example5() {
        assert_eq!(4, solve_part2_captcha("12131415"));
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(1194, day1_part2());
    }
}
