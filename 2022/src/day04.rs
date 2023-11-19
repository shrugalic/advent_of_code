use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day04.txt");

pub(crate) fn day04_part1() -> usize {
    let pairs = parse(INPUT);
    fully_enclosed_range_count(pairs)
}

pub(crate) fn day04_part2() -> usize {
    let pairs = parse(INPUT);
    overlapping_range_count(pairs)
}

fn parse(input: &str) -> Vec<(RangeInclusive<u8>, RangeInclusive<u8>)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<u8> = line.split([',', '-']).map(|s| s.parse().unwrap()).collect();
            (parts[0]..=parts[1], parts[2]..=parts[3])
        })
        .collect()
}

fn fully_enclosed_range_count(pairs: Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            a.start() <= b.start() && a.end() >= b.end()
                || a.start() >= b.start() && a.end() <= b.end()
        })
        .count()
}

fn overlapping_range_count(pairs: Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| a.start() <= b.end() && a.end() >= b.start())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_example() {
        let pairs = parse(EXAMPLE);
        assert_eq!(2, fully_enclosed_range_count(pairs));
    }

    #[test]
    fn part2_example() {
        let pairs = parse(EXAMPLE);
        assert_eq!(4, overlapping_range_count(pairs));
    }

    #[test]
    fn part1() {
        assert_eq!(507, day04_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(897, day04_part2());
    }
}
