use rayon::prelude::*;
use std::ops::Range;

const INPUT: &str = include_str!("../input/day05.txt");

pub(crate) fn part1() -> Number {
    minimum_location_reachable_from_seeds(INPUT)
}

pub(crate) fn part2() -> Number {
    minimum_location_reachable_from_seed_ranges(INPUT)
}

fn minimum_location_reachable_from_seeds(input: &str) -> Number {
    Almanac::from(input).minimum_location_reachable_from_seeds()
}

fn minimum_location_reachable_from_seed_ranges(input: &str) -> Number {
    Almanac::from(input)
        .with_resolved_seed_ranges()
        .minimum_location_reachable_from_seeds()
}

type Number = usize;
type Offset = isize;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Number>,
    mappers: Vec<Mapper>,
}

#[derive(Debug)]
struct Mapper {
    mappings: Vec<Mapping>,
}

#[derive(Debug, Default)]
struct Mapping {
    source_range: Range<Number>,
    offset: Offset,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let (seeds, mappers) = input.trim().split_once("\n\n").expect("proper input");
        let seeds = seeds
            .split_ascii_whitespace()
            .skip(1) // skip "seeds: " prefix
            .filter_map(|n| n.parse().ok())
            .collect();
        let mappers = mappers.split("\n\n").map(Mapper::from).collect();
        Almanac { seeds, mappers }
    }
}

impl From<&str> for Mapper {
    fn from(input: &str) -> Self {
        let mappings: Vec<_> = input.lines().skip(1).map(Mapping::from).collect();
        Mapper { mappings }
    }
}

impl From<&str> for Mapping {
    fn from(line: &str) -> Self {
        let parts: Vec<_> = line
            .split_ascii_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let destination_range_start = parts[0];
        let source_range_start = parts[1];
        let range_length = parts[2];
        let source_range_end = source_range_start + range_length;
        Mapping {
            source_range: source_range_start..source_range_end,
            offset: destination_range_start as isize - source_range_start as isize,
        }
    }
}

impl Almanac {
    fn map_seeds_to_locations(self) -> Vec<Number> {
        self.seeds.par_iter().map(|seed| self.map(seed)).collect()
    }
    fn map(&self, seed: &Number) -> Number {
        self.mappers
            .iter()
            .fold(*seed, |value, mapper| mapper.map(value))
    }
    fn with_resolved_seed_ranges(mut self) -> Self {
        self.seeds = self
            .seeds
            .chunks_exact(2)
            .flat_map(|pair| (pair[0]..pair[0] + pair[1]).into_iter().collect::<Vec<_>>())
            .collect();
        self
    }
    fn minimum_location_reachable_from_seeds(self) -> Number {
        self.map_seeds_to_locations().into_iter().min().unwrap()
    }
}

impl Mapper {
    fn map(&self, value: Number) -> Number {
        self.mappings
            .iter()
            .find(|mapping| mapping.contains(&value))
            .map_or(value, |mapping| mapping.map(value))
    }
}

impl Mapping {
    fn contains(&self, value: &Number) -> bool {
        self.source_range.contains(&value)
    }
    fn map(&self, value: Number) -> Number {
        (value as Offset + self.offset) as Number
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part1_example() {
        assert_eq!(35, minimum_location_reachable_from_seeds(EXAMPLE));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(46, minimum_location_reachable_from_seed_ranges(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(424_490_994, minimum_location_reachable_from_seeds(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            15_290_096,
            minimum_location_reachable_from_seed_ranges(INPUT)
        );
    }
}
