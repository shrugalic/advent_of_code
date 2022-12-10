use crate::parse;

const INPUT: &str = include_str!("../input/day01.txt");

pub(crate) fn day01_part1() -> isize {
    result_floor(parse(INPUT)[0])
}
pub(crate) fn day01_part2() -> usize {
    position_of_char_that_results_in_basement_floor(parse(INPUT)[0])
}

fn result_floor(input: &str) -> isize {
    input.chars().map(char_to_diff).sum()
}

fn char_to_diff(c: char) -> isize {
    match c {
        '(' => 1,
        ')' => -1,
        _ => unreachable!(),
    }
}

fn position_of_char_that_results_in_basement_floor(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += char_to_diff(c);
        if floor == -1 {
            return i + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(0, result_floor("(())"));
        assert_eq!(0, result_floor("()()"));
        assert_eq!(3, result_floor("((("));
        assert_eq!(3, result_floor("(()(()("));
        assert_eq!(3, result_floor("))((((("));
        assert_eq!(3, result_floor("))((((("));
        assert_eq!(-1, result_floor("())"));
        assert_eq!(-1, result_floor("))("));
        assert_eq!(-3, result_floor(")))"));
        assert_eq!(-3, result_floor(")())())"));
    }

    #[test]
    fn part1() {
        assert_eq!(280, day01_part1());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(1, position_of_char_that_results_in_basement_floor(")"));
        assert_eq!(5, position_of_char_that_results_in_basement_floor("()())"));
    }

    #[test]
    fn part2() {
        assert_eq!(1797, day01_part2());
    }
}
