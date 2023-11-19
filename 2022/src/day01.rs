use std::cmp::Reverse;

const INPUT: &str = include_str!("../input/day01.txt");

pub(crate) fn day01_part1() -> usize {
    let supplies = parse_supplies(INPUT);
    largest_total_supply(&supplies)
}

pub(crate) fn day01_part2() -> usize {
    let supplies = parse_supplies(INPUT);
    sum_of_largest_three_total_supplies(&supplies)
}

fn parse_supplies(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split("\n\n")
        .filter_map(|s| s.lines().map(|s| s.parse().ok()).collect())
        .collect()
}

fn calculate_totals(supplies: &[Vec<usize>]) -> Vec<usize> {
    supplies.iter().map(|v| v.iter().sum()).collect()
}

fn largest_total_supply(supplies: &[Vec<usize>]) -> usize {
    *calculate_totals(supplies).iter().max().unwrap()
}

fn sum_of_largest_three_total_supplies(supplies: &[Vec<usize>]) -> usize {
    let mut totals = calculate_totals(supplies);
    totals.sort_unstable_by_key(|&k| Reverse(k));
    totals.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_parse_supplies() {
        let supplies = parse_supplies(EXAMPLE);
        assert_eq!(
            vec![
                vec![1_000, 2_000, 3_000],
                vec![4_000],
                vec![5_000, 6_000],
                vec![7_000, 8_000, 9_000],
                vec![10_000],
            ],
            supplies
        );
    }

    #[test]
    fn test_calculate_totals() {
        let supplies = vec![
            vec![1_000, 2_000, 3_000],
            vec![4_000],
            vec![5_000, 6_000],
            vec![7_000, 8_000, 9_000],
            vec![10_000],
        ];
        assert_eq!(
            vec![6_000, 4_000, 11_000, 24_000, 10_000],
            calculate_totals(&supplies)
        );
    }

    #[test]
    fn test_largest_total_supply() {
        let supplies = parse_supplies(EXAMPLE);
        assert_eq!(24_000, largest_total_supply(&supplies));
    }

    #[test]
    fn test_sum_of_largest_three_total_supplies() {
        let supplies = parse_supplies(EXAMPLE);
        assert_eq!(45_000, sum_of_largest_three_total_supplies(&supplies));
    }

    #[test]
    fn test_part1() {
        assert_eq!(72_240, day01_part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(210_957, day01_part2());
    }
}
