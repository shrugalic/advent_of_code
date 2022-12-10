use crate::parse;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day05.txt");

pub(crate) fn day05_part1() -> usize {
    count_nice_strings(parse(INPUT), &is_nice_part1)
}

pub(crate) fn day05_part2() -> usize {
    count_nice_strings(parse(INPUT), &is_nice_part2)
}

fn count_nice_strings(strings: Vec<&str>, is_nice: &dyn Fn(&str) -> bool) -> usize {
    strings.iter().filter(|&s| is_nice(s)).count()
}

const VOWELS: &str = "aeiou";
const FORBIDDEN: [[char; 2]; 4] = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];
fn is_nice_part1(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    chars.iter().filter(|c| VOWELS.contains(**c)).count() >= 3
        && chars.windows(2).any(|a| a[0] == a[1])
        && chars
            .windows(2)
            .all(|a| !FORBIDDEN.iter().any(|s| s[0] == a[0] && s[1] == a[1]))
}

fn is_nice_part2(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    let mut idx_by_pair = HashMap::new();
    chars.windows(2).enumerate().for_each(|(idx, w)| {
        idx_by_pair.entry(w).or_insert_with(Vec::new).push(idx);
    });
    chars.windows(2).enumerate().any(|(idx, w)| {
        idx_by_pair
            .get(w)
            .unwrap()
            .iter()
            .any(|&o_idx| o_idx != idx && o_idx != idx + 1 && idx != o_idx + 1)
    }) && chars.windows(3).any(|w| w[0] == w[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert!(is_nice_part1("aei_xx"));
        assert!(is_nice_part1("xazegov_yy"));
        assert!(is_nice_part1("aeiouaeiouaeiou_zz"));
        assert!(!is_nice_part1("aei_xxy"));
        assert!(!is_nice_part1("xazegov_yy_ab"));
        assert!(!is_nice_part1("aeiouaeiouaeiou_zz_cd"));
    }
    #[test]
    fn part1() {
        assert_eq!(238, day05_part1());
    }

    #[test]
    fn part2_examples() {
        assert!(is_nice_part2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_part2("xxyxx"));
        assert!(!is_nice_part2("uurcxstgmygtbstg"));
        assert!(!is_nice_part2("ieodomkazucvgmuy"));
    }

    #[test]
    fn part2() {
        assert_eq!(69, day05_part2());
    }
}
