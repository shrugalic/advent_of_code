use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day05.txt");

pub(crate) fn part1() -> Number {
    minimum_location_reachable_from_seeds(INPUT)
}

pub(crate) fn part2() -> Number {
    minimum_location_reachable_from_seed_ranges(INPUT)
}

fn minimum_location_reachable_from_seeds(input: &str) -> Number {
    let (seeds, almanac) = parse(input);
    almanac.map_seeds_to_locations(seeds).min().unwrap()
}

fn minimum_location_reachable_from_seed_ranges(input: &str) -> Number {
    let (seeds, almanac) = parse(input);
    let seed_ranges = convert_seeds_to_seed_ranges(seeds);
    almanac
        .map_ranges_to_location_ranges(seed_ranges)
        .map(|seed_range| *seed_range.start())
        .min()
        .unwrap()
}

fn parse(input: &str) -> (Seeds, Almanac) {
    let (seeds, mappers) = input.trim().split_once("\n\n").expect("proper input");
    let seeds = seeds
        .split_ascii_whitespace()
        .skip(1) // skip "seeds: " prefix
        .filter_map(|n| n.parse().ok())
        .collect();
    (seeds, Almanac::from(mappers))
}

fn convert_seeds_to_seed_ranges(seeds: Seeds) -> SeedRanges {
    seeds
        .chunks_exact(2)
        .map(|seed| seed[0]..=seed[0] + seed[1] - 1)
        .collect()
}

type Number = usize;
type Seed = Number;
type Seeds = Vec<Seed>;
type Offset = isize;
type Range = RangeInclusive<Number>;
type SeedRanges = Vec<Range>;

#[derive(Debug)]
struct Almanac {
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    range: Range,
    offset: Offset,
}

impl From<&str> for Almanac {
    fn from(mappers: &str) -> Self {
        let mappers = mappers.split("\n\n").map(Map::from).collect();
        Almanac { maps: mappers }
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut mappings: Vec<_> = input.lines().skip(1).map(Mapping::from).collect();
        mappings.sort_unstable_by_key(|m| *m.range.start());
        Map { mappings }
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
            range: source_range_start..=source_range_end - 1,
            offset: destination_range_start as isize - source_range_start as isize,
        }
    }
}

impl Almanac {
    fn map_seeds_to_locations(self, seeds: Seeds) -> impl Iterator<Item = Number> {
        seeds
            .into_iter()
            .map(move |seed| self.map_seed_to_location(seed))
    }
    fn map_ranges_to_location_ranges(self, ranges: SeedRanges) -> impl Iterator<Item = Range> {
        ranges
            .into_iter()
            .flat_map(move |range| self.map_range(range))
    }
    fn map_seed_to_location(&self, seed: Number) -> Number {
        self.maps
            .iter()
            .fold(seed, |value, mapper| mapper.map_seed(value))
    }
    fn map_range(&self, range: Range) -> impl Iterator<Item = Range> {
        let mut input_ranges = vec![range];
        for map in &self.maps {
            input_ranges = map.map_ranges(map.split_seed_ranges(input_ranges));
        }
        input_ranges.into_iter()
    }
}

impl Map {
    fn map_seed(&self, value: Number) -> Number {
        self.mappings
            .iter()
            .find(|mapping| mapping.contains(&value))
            .map_or(value, |mapping| mapping.map(value))
    }
    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        self.split_seed_ranges(ranges)
            .into_iter()
            .map(|range| self.map_range(range))
            .collect()
    }
    fn split_seed_ranges(&self, mut seed_ranges: SeedRanges) -> SeedRanges {
        for mapping in &self.mappings {
            seed_ranges = seed_ranges
                .into_iter()
                .flat_map(|seed_range| mapping.split_seed_ranges(seed_range))
                .collect();
        }
        seed_ranges
    }
    fn map_range(&self, range: Range) -> Range {
        self.map_seed(*range.start())..=self.map_seed(*range.end())
    }
}

impl Mapping {
    fn contains(&self, value: &Number) -> bool {
        self.range.contains(value)
    }
    fn map(&self, value: Number) -> Number {
        (value as Offset + self.offset) as Number
    }
    fn split_seed_ranges(&self, range: Range) -> SeedRanges {
        self.range.split_on_boundaries(range)
    }
}

trait SplitOnBoundary {
    fn split_on_boundaries(&self, range: Range) -> Vec<Range>;
}
impl SplitOnBoundary for Range {
    fn split_on_boundaries(&self, other: Range) -> Vec<Range> {
        if other.start() < self.start() && self.end() < other.end() {
            // other completely encloses this -> split it into 3 parts
            vec![
                *other.start()..=*self.start() - 1,
                self.clone(),
                *self.end() + 1..=*other.end(),
            ]
        } else if self.start() <= other.start() && other.end() <= self.end() {
            // other is completely inside -> return as is
            vec![other]
        } else if other.start() < self.start() && other.contains(self.start()) {
            // other overlaps on the left -> split it into 2 parts
            if self.start() == &0 {
                dbg!(&self);
                dbg!(&other);
            }
            vec![
                *other.start()..=*self.start() - 1,
                *self.start()..=*other.end(),
            ]
        } else if self.end() < other.end() && other.contains(self.end()) {
            // other overlaps on the right -> split it into 2 parts
            vec![*other.start()..=*self.end(), *self.end() + 1..=*other.end()]
        } else {
            // other is completely outside  -> return as is
            vec![other]
        }
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
    fn test_split_on_boundary() {
        let range = 4..=7;
        assert_eq!(range.split_on_boundaries(0..=3), vec![0..=3]);
        assert_eq!(range.split_on_boundaries(8..=9), vec![8..=9]);
        assert_eq!(range.split_on_boundaries(5..=6), vec![5..=6]);
        assert_eq!(range.split_on_boundaries(4..=7), vec![4..=7]);
        assert_eq!(range.split_on_boundaries(2..=6), vec![2..=3, 4..=6]);
        assert_eq!(range.split_on_boundaries(6..=9), vec![6..=7, 8..=9]);
        assert_eq!(range.split_on_boundaries(4..=8), vec![4..=7, 8..=8]);
        assert_eq!(range.split_on_boundaries(2..=7), vec![2..=3, 4..=7]);
        assert_eq!(range.split_on_boundaries(2..=9), vec![2..=3, 4..=7, 8..=9]);
        assert_eq!(range.split_on_boundaries(3..=8), vec![3..=3, 4..=7, 8..=8]);
    }

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
