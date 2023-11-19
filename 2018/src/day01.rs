use crate::parse;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day01.txt");

pub(crate) fn day1_part1() -> isize {
    cumulate_frequency_adjustments(&parse(INPUT))
}

pub(crate) fn day1_part2() -> isize {
    find_first_repeated_frequency(&parse(INPUT))
}

fn cumulate_frequency_adjustments(input: &[&str]) -> isize {
    input.iter().map(|s| s.parse::<isize>().unwrap()).sum()
}

pub(crate) fn find_first_repeated_frequency(input: &[&str]) -> isize {
    let adjustments: Vec<isize> = input.iter().map(|s| s.parse().unwrap()).collect();
    let mut seen: HashSet<isize> = HashSet::new();
    let mut freq = 0;
    loop {
        for adj in &adjustments {
            freq += adj;
            if seen.contains(&freq) {
                return freq;
            } else {
                seen.insert(freq);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "1
-2
+3
+1";

    #[test]
    fn example_1() {
        assert_eq!(cumulate_frequency_adjustments(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part_1() {
        assert_eq!(454, day1_part1());
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(find_first_repeated_frequency(&parse(EXAMPLE)), 2);
    }

    #[test]
    fn part_2() {
        assert_eq!(566, day1_part2());
    }
}
