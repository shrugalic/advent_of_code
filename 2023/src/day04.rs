use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day04.txt");

pub(crate) fn part1() -> usize {
    sum_of_points(INPUT)
}

pub(crate) fn part2() -> usize {
    total_card_count(INPUT)
}

fn sum_of_points(input: &str) -> usize {
    parse_cards(input).map(Card::point_value).sum()
}

fn total_card_count(input: &str) -> usize {
    let match_counts: Vec<_> = parse_cards(input).map(Card::match_count).collect();
    let mut card_counts = vec![1; match_counts.len()];
    for (index, match_count) in match_counts.into_iter().enumerate() {
        let card_count = card_counts[index];
        for offset in 1..=match_count {
            if index + offset < card_counts.len() {
                card_counts[index + offset] += card_count;
            }
        }
    }
    card_counts.into_iter().sum()
}

fn parse_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.trim().lines().map(Card::from)
}

type Number = usize;
#[derive(Debug, PartialEq, Default)]
struct Card {
    winning: HashSet<Number>,
    own: HashSet<Number>,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let (left, right) = line.split_once(" | ").expect("A ' | ' separator");
        let (_, winning) = left.split_once(": ").expect("A ': ' separator");
        let parse_numbers = |s: &str| -> HashSet<usize> {
            s.split_ascii_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        };
        let winning = parse_numbers(winning);
        let own = parse_numbers(right);
        Card { winning, own }
    }
}

impl Card {
    fn point_value(self) -> usize {
        let count = self.match_count() as u32;
        if count == 0 {
            0
        } else {
            2usize.pow(count - 1)
        }
    }
    fn match_count(self) -> usize {
        self.winning.intersection(&self.own).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_card_from() {
        let game = Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            game,
            Card {
                winning: vec![41, 48, 83, 86, 17].into_iter().collect(),
                own: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect()
            }
        )
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(13, sum_of_points(EXAMPLE));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(30, total_card_count(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(25_174, sum_of_points(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6_420_979, total_card_count(INPUT));
    }
}
