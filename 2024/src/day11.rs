use std::collections::HashMap;

const INPUT: &str = include_str!("../../2024/input/day11.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    Stones::from(input).blink(25).count()
}

fn solve_part2(input: &str) -> usize {
    Stones::from(input).blink(75).count()
}

type StoneNum = u64;
type Count = usize;

struct Stones {
    stones: HashMap<StoneNum, Count>,
}

impl From<&str> for Stones {
    fn from(input: &str) -> Self {
        let stones = input
            .trim()
            .split_whitespace()
            .map(|n| (n.parse().unwrap(), 1))
            .collect();
        Stones { stones }
    }
}

impl Stones {
    fn count(&self) -> Count {
        self.stones.values().map(|count| count).sum()
    }

    fn blink(mut self, count: u8) -> Self {
        for _ in 0..count {
            let mut next_stones = HashMap::new();
            for (stone, count) in self.stones.drain() {
                let next = Stones::blink_single(stone);
                *next_stones.entry(next[0]).or_insert(0) += count;
                if next.len() == 2 {
                    *next_stones.entry(next[1]).or_insert(0) += count;
                }
            }
            self.stones = next_stones;
        }
        self
    }

    fn blink_single(stone: StoneNum) -> Vec<StoneNum> {
        match stone {
            0 => vec![1],
            n if n.to_string().len() % 2 == 0 => {
                let divisor = (10 as StoneNum).pow((n.to_string().len() / 2) as u32);
                vec![n / divisor, n % divisor]
            }
            n => vec![n * 2024],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "0 1 10 99 999";
    const EXAMPLE2: &str = "125 17";

    #[test]
    fn test_part1_example1() {
        assert_eq!(7, Stones::from(EXAMPLE1).blink(1).count());
    }
    #[test]
    fn test_part1_example2() {
        assert_eq!(55_312, Stones::from(EXAMPLE2).blink(25).count());
    }

    #[test]
    fn test_part1() {
        assert_eq!(194_557, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(231_532_558_973_909, solve_part2(INPUT));
    }
}
