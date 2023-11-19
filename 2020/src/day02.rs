use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

pub(crate) fn is_valid_for_part_1(
    range: &RangeInclusive<usize>,
    letter: &char,
    pw: &dyn AsRef<str>,
) -> bool {
    let count = pw.as_ref().chars().filter(|c| c == letter).count();
    range.contains(&count)
}

pub(crate) fn is_valid_for_part_2(
    range: &RangeInclusive<usize>,
    letter: &char,
    pw: &dyn AsRef<str>,
) -> bool {
    let pw: Vec<char> = pw.as_ref().chars().collect();
    (pw[*range.start() - 1] == *letter) ^ (pw[*range.end() - 1] == *letter)
}

pub(crate) fn day02_read_file(filename: &str) -> Vec<(RangeInclusive<usize>, char, String)> {
    let mut results: Vec<(RangeInclusive<usize>, char, String)> = vec![];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line
            .split(|c: char| c.is_ascii_whitespace() || c == '-' || c == ' ' || c == ':')
            .collect();
        let range: RangeInclusive<usize> = parts[0].parse().unwrap()..=parts[1].parse().unwrap();
        let letter: char = parts[2].chars().next().unwrap();
        let password = parts[4].chars().collect();
        // println!("{} - {} {}: {}", range.start, range.end, letter, password);
        results.push((range, letter, password));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert!(is_valid_for_part_1(&(1..=3), &'a', &"abcde"));
    }
    #[test]
    fn part1_test2() {
        assert!(!is_valid_for_part_1(&(1..=3), &'b', &"cdefg"));
    }
    #[test]
    fn part1_test3() {
        assert!(is_valid_for_part_1(&(2..=9), &'c', &"ccccccccc"));
    }
    #[test]
    fn part2_test1() {
        assert!(is_valid_for_part_2(&(1..=3), &'a', &"abcde"));
    }
    #[test]
    fn part2_test2() {
        assert!(!is_valid_for_part_2(&(1..=3), &'b', &"cdefg"));
    }
    #[test]
    fn part2_test3() {
        assert!(!is_valid_for_part_2(&(2..=9), &'c', &"ccccccccc"));
    }

    #[test]
    fn part1() {
        let tuples = day02_read_file("input/day02.txt");
        let count = tuples
            .iter()
            .filter(|(range, letter, password)| is_valid_for_part_1(range, letter, password))
            .count();
        assert_eq!(454, count);
    }

    #[test]
    fn part2() {
        let tuples = day02_read_file("input/day02.txt");
        let count = tuples
            .iter()
            .filter(|(range, letter, password)| is_valid_for_part_2(range, letter, password))
            .count();
        assert_eq!(649, count);
    }
}
