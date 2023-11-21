const INPUT: &str = include_str!("../input/day01.txt");

pub(crate) fn part1() -> usize {
    let v: Vec<Vec<usize>> = INPUT
        .trim()
        .split("\n\n")
        .filter_map(|s| s.lines().map(|s| s.parse().ok()).collect())
        .collect();
    v.len()
}

pub(crate) fn part2() -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1000
2000
3000";

    #[test]
    fn test_part1() {
        assert_eq!(0, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2());
    }
}