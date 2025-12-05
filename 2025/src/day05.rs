use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../2025/input/day05.txt");

type Id = usize;
pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let (id_ranges, available_ids) = parse(input);
    available_ids
        .iter()
        .filter(|&&id| id_ranges.iter().any(|range| range.contains(&id)))
        .count()
}

fn solve_part2(input: &str) -> usize {
    let (mut id_ranges, _) = parse(input);
    id_ranges.sort_unstable_by_key(|range| *range.start());

    let mut total = 0;
    let mut curr = id_ranges.remove(0);
    for next in id_ranges.drain(..) {
        if curr.contains(next.start()) {
            if curr.end() < next.end() {
                curr = *curr.start()..=*next.end();
            }
        } else {
            total += curr.end() - curr.start() + 1;
            curr = next;
        }
    }
    total += curr.end() - curr.start() + 1;
    total
}

fn parse(input: &str) -> (Vec<RangeInclusive<Id>>, Vec<Id>) {
    let (top, bottom) = input.trim().split_once("\n\n").unwrap();
    let ranges = top
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("-").unwrap();
            let l = l.parse().unwrap();
            let r = r.parse().unwrap();
            l..=r
        })
        .collect();
    let available = bottom
        .lines()
        .flat_map(|line| line.parse())
        .collect::<Vec<_>>();

    (ranges, available)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part1_example() {
        assert_eq!(3, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(623, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(14, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(353507173555373, solve_part2(INPUT));
    }
}
