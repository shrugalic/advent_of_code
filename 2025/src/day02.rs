use std::iter;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../2025/input/day02.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    parse(input)
        .flat_map(|range| {
            range.into_iter().filter(|num| {
                let chars: Vec<char> = num.to_string().chars().collect();
                is_sequence_repeated_twice(&chars)
            })
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .flat_map(|range| {
            range.into_iter().filter(|num| {
                let chars: Vec<char> = num.to_string().chars().collect();
                is_repeated_sequences_of_length(1, &chars)
                    || is_repeated_sequences_of_length(2, &chars)
                    || is_repeated_sequences_of_length(3, &chars)
                    || is_repeated_sequences_of_length(4, &chars)
                    || is_repeated_sequences_of_length(5, &chars)
            })
        })
        .sum()
}

fn is_sequence_repeated_twice(c: &Vec<char>) -> bool {
    (c.len() == 2 && c[0] == c[1])
        || (c.len() == 4 && c[0] == c[2] && c[1] == c[3])
        || (c.len() == 6 && c[0] == c[3] && c[1] == c[4] && c[2] == c[5])
        || (c.len() == 8 && c[0] == c[4] && c[1] == c[5] && c[2] == c[6] && c[3] == c[7])
        || (c.len() == 10
            && c[0] == c[5]
            && c[1] == c[6]
            && c[2] == c[7]
            && c[3] == c[8]
            && c[4] == c[9])
}

fn is_repeated_sequences_of_length(len: usize, chars: &Vec<char>) -> bool {
    let invalid = chars.len() > len
        && chars.len() % len == 0
        && chars
            .windows(len)
            .step_by(len)
            .skip(1)
            .all(|w| (0..len).into_iter().all(|i| w[i] == chars[i]));
    if invalid && false {
        let formatted = chars
            .windows(len)
            .step_by(len)
            .flat_map(|slice| iter::chain(slice.iter(), iter::once(&'_')))
            .take(chars.len() + chars.len() / len - 1)
            .collect::<String>();
        println!("len {}: {}", len, formatted);
    }
    invalid
}

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<usize>> {
    input
        .trim()
        .split([',', '\n']) // '\n' is just for the EXAMPLE
        .filter(|s| !s.is_empty()) // Only EXAMPLE has '\n' following ','
        .map(|s| s.split_once('-').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .map(|(l, r)| l..=r)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

    use rstest::rstest;
    #[rstest]
    #[case("11-22", 33)]
    #[case("95-115", 99)]
    #[case("998-1012", 10_10)]
    #[case("1188511880-1188511890", 11885_11885)]
    #[case("222220-222224", 222_222)]
    #[case("1698522-1698528", 0)]
    #[case("446443-446449", 446_446)]
    #[case("38593856-38593862", 3859_3859)]
    #[case("565653-565659", 0)]
    #[case("824824821-824824827", 0)]
    #[case("2121212118-2121212124", 0)]
    fn run_part1_test(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, solve_part1(input));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(1227775554, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(38437576669, solve_part1(INPUT));
    }

    #[rstest]
    #[case("11-22", 33)]
    #[case("95-115", 99 + 111)]
    #[case("998-1012", 999 + 10_10)]
    #[case("1188511880-1188511890", 11885_11885)]
    #[case("222220-222224", 222222)]
    #[case("1698522-1698528", 0)]
    #[case("446443-446449", 446_446)]
    #[case("38593856-38593862", 3859_3859)]
    #[case("565653-565659", 56_56_56)]
    #[case("824824821-824824827", 824_824_824)]
    #[case("2121212118-2121212124", 21_21_21_21_21)]
    #[case("0-10", 0)]
    fn run_part2_test(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, solve_part2(input));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(4174379265, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        // 49046150799 is too high
        assert_eq!(49046150754, solve_part2(INPUT));
    }
}
