use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day01.txt");

pub(crate) fn part1() -> u32 {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> u32 {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> u32 {
    let (mut left, mut right) = split_into_two_lists(input);
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| if l < r { r - l } else { l - r })
        .sum()
}

fn split_into_two_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let pairs = input
        .trim()
        .lines()
        .map(line_to_number_pair)
        .collect::<Vec<_>>();
    (
        pairs.iter().map(|pair| pair.0).collect(),
        pairs.iter().map(|pair| pair.1).collect(),
    )
}

fn line_to_number_pair(line: &str) -> (u32, u32) {
    let parts: Vec<_> = line
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    (parts[0], parts[1])
}

fn solve_part2(input: &str) -> u32 {
    let (left, right) = split_into_two_lists(input);
    let mut counts: HashMap<_, _> = HashMap::new();
    for i in &right {
        counts.entry(i).and_modify(|cnt| *cnt += 1).or_insert(1);
    }
    left.iter()
        .map(|i| i * *counts.get(&i).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1_example() {
        assert_eq!(11, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(31, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1_579_939, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(20_351_745, solve_part2(INPUT));
    }
}
