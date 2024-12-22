const INPUT: &str = include_str!("../input/day02.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(to_report)
        .filter(|report| is_safe(report))
        .count()
}

fn is_safe(report: &[i32]) -> bool {
    report.windows(2).all(|w| w[1] > w[0] && w[1] <= w[0] + 3)
        || report.windows(2).all(|w| w[1] < w[0] && w[1] >= w[0] - 3)
}

fn solve_part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(to_report)
        .filter(|report| is_safe_by_removing_a_level(report))
        .count()
}

fn is_safe_by_removing_a_level(report: &[i32]) -> bool {
    (0..report.len()).any(|idx_to_skip| {
        is_safe(
            &report
                .iter()
                .enumerate()
                .filter(|(i, _)| i != &idx_to_skip)
                .map(|(_, n)| *n)
                .collect::<Vec<_>>(),
        )
    })
}

fn to_report(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1_example() {
        assert_eq!(2, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(564, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(4, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(604, solve_part2(INPUT));
    }
}
