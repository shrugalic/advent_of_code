use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day03.txt");

pub(crate) fn day03_part1() -> usize {
    let backpacks = parse(INPUT);
    sum_of_common_items_in_left_and_right_half(backpacks)
}

pub(crate) fn day03_part2() -> usize {
    let backpacks = parse(INPUT);
    sum_of_common_items_of_each_group_of_3(backpacks)
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|s| s.bytes().collect::<Vec<_>>())
        .collect()
}

fn sum_of_common_items_in_left_and_right_half(backpacks: Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for rucksack in backpacks {
        let mid = rucksack.len() / 2;
        let left: HashSet<_> = HashSet::from_iter(&rucksack[0..mid]);
        let right: HashSet<_> = HashSet::from_iter(&rucksack[mid..]);
        let common_char = left.intersection(&right).next().unwrap();
        sum += common_char.to_priority() as usize;
    }
    sum
}

fn sum_of_common_items_of_each_group_of_3(backpacks: Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for rucksack in backpacks.as_slice().windows(3).step_by(3) {
        let first: HashSet<_> = HashSet::from_iter(&rucksack[0]);
        let second: HashSet<_> = HashSet::from_iter(&rucksack[1]);
        let common_chars: HashSet<_> = first.intersection(&second).cloned().collect();
        let third: HashSet<_> = HashSet::from_iter(&rucksack[2]);
        let common_char = common_chars.intersection(&third).next().unwrap();
        sum += common_char.to_priority() as usize;
    }
    sum
}

trait ToPriority {
    fn to_priority(&self) -> u8;
}
impl ToPriority for u8 {
    fn to_priority(&self) -> u8 {
        match *self {
            b'a'..=b'z' => 1 + *self - b'a',
            b'A'..=b'Z' => 27 + *self - b'A',
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_to_priority() {
        assert_eq!(1, b'a'.to_priority());
        assert_eq!(27, b'A'.to_priority());
    }

    #[test]
    fn example1() {
        let backpacks = parse(EXAMPLE);
        assert_eq!(157, sum_of_common_items_in_left_and_right_half(backpacks));
    }

    #[test]
    fn example2() {
        let backpacks = parse(EXAMPLE);
        assert_eq!(70, sum_of_common_items_of_each_group_of_3(backpacks));
    }

    #[test]
    fn part1() {
        assert_eq!(7_763, day03_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(2_569, day03_part2());
    }
}
