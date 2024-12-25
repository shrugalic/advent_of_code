const INPUT: &str = include_str!("../../2024/input/day25.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let (height, locks, keys) = parse(input);
    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    key.iter()
                        .zip(lock.iter())
                        .all(|(k, l)| k + l <= height as u8)
                })
                .count()
        })
        .sum()
}

type Lock = Vec<u8>;
type Key = Vec<u8>;
fn parse(input: &str) -> (usize, Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut height = 0;
    for s in input.trim().split("\n\n") {
        let chars: Vec<_> = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        let is_key = chars[0].iter().all(|n| n == &'.');
        height = chars.len() - 2;
        let mut counts = vec![0; chars[0].len()];
        for line in chars.iter().skip(1).take(height) {
            for (i, c) in line.iter().enumerate() {
                counts[i] += if c == &'#' { 1u8 } else { 0u8 }
            }
        }
        if is_key {
            keys.push(counts);
        } else {
            locks.push(counts);
        }
    }
    (height, locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn test_part1_example() {
        assert_eq!(3, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(3_264, solve_part1(INPUT));
    }
}
