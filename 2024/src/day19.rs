use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../2024/input/day19.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

type Color = char;
type Towel = Vec<Color>;
type Towels = HashSet<Towel>;
type Design = Vec<Color>;
type Count = usize;

fn solve_part1(input: &str) -> usize {
    let (towels, designs) = parse(input);
    designs
        .iter()
        .filter(|design| is_possible(design, &towels))
        .count()
}

fn solve_part2(input: &str) -> usize {
    let (towels, designs) = parse(input);
    let mut cache: HashMap<&[Color], Count> = HashMap::new();
    designs
        .iter()
        .map(|design| count_possibilities(&mut cache, design, &towels))
        .sum()
}

fn is_possible(design: &[Color], towels: &Towels) -> bool {
    towels
        .iter()
        .filter(|towel| design.starts_with(towel))
        .any(|first_towel| {
            let remaining_design = &design[first_towel.len()..];
            first_towel.len() == design.len() || is_possible(remaining_design, towels)
        })
}

fn count_possibilities<'a>(
    cache: &mut HashMap<&'a [Color], usize>,
    design: &'a [Color],
    towels: &'a Towels,
) -> usize {
    if cache.contains_key(design) {
        return cache[design];
    } else if design.is_empty() {
        return 1;
    }
    let possibility_count = towels
        .iter()
        .filter(|towel| design.starts_with(towel))
        .map(|first_towel| {
            let remaining_design = &design[first_towel.len()..];
            count_possibilities(cache, remaining_design, towels)
        })
        .sum();
    cache.insert(design, possibility_count);
    possibility_count
}

fn parse(input: &str) -> (HashSet<Towel>, Vec<Design>) {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels
        .split(", ")
        .map(|towel| towel.chars().collect())
        .collect();
    let designs = designs.lines().map(|line| line.chars().collect()).collect();
    (towels, designs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part1_example() {
        assert_eq!(6, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(209, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(16, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(777_669_668_613_191, solve_part2(INPUT));
    }
}
