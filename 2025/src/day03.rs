const INPUT: &str = include_str!("../../2025/input/day03.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    parse(input).map(two_digit_max_joltage).sum()
}

fn solve_part2(input: &str) -> usize {
    parse(input).map(twelve_digit_max_joltage).sum()
}

fn two_digit_max_joltage(joltage: Vec<u8>) -> usize {
    let len = joltage.len();
    let (left_idx, left_digit) = joltage[0..len - 1] // skip the last digit to leave room for a 2nd one
        .iter()
        .enumerate()
        .rev() // max returns the last element if there are multiple maximums, but we need the left-most
        .max_by_key(|(_, d)| **d)
        .unwrap();
    let right_digit = joltage[left_idx + 1..].iter().max().unwrap();
    (*left_digit * 10 + *right_digit) as usize
}

fn twelve_digit_max_joltage(joltage: Vec<u8>) -> usize {
    max_joltage(&joltage, 12)
}

fn max_joltage(joltage: &[u8], needed: usize) -> usize {
    let len = joltage.len();
    let (idx, digit) = joltage[0..len - (needed - 1)] // leave room for remaining digits
        .iter()
        .enumerate()
        .rev() // max returns the last element if there are multiple maximums, but we need the left-most
        .max_by_key(|(_, d)| **d)
        .unwrap();

    let digit = *digit as usize;
    if needed == 1 {
        digit
    } else {
        let remaining = max_joltage(&joltage[idx + 1..], needed - 1);
        let exp = remaining.ilog(10);
        digit * 10usize.pow(exp + 1) + remaining
    }
}

fn parse(input: &str) -> impl Iterator<Item = Vec<u8>> {
    input.trim().lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part1_example() {
        assert_eq!(98 + 89 + 78 + 92 /* = 357 */, solve_part1(EXAMPLE));
    }

    #[rstest]
    #[case("819", 89)]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("9293", 99)]
    fn run_part1_test(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, solve_part1(input));
    }

    #[test]
    fn debug() {
        assert_eq!(89, solve_part1("819"));
    }

    #[test]
    fn test_part1() {
        // 17157 is not right
        assert_eq!(17330, solve_part1(INPUT));
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn run_part2_test(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, solve_part2(input));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            987654321111 + 811111111119 + 434234234278 + 888911112111, // = 3121910778619
            solve_part2(EXAMPLE)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(171518260283767, solve_part2(INPUT));
    }
}
