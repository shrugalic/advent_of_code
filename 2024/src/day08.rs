use crate::hash_char_grid::{CharGrid, GridContainsPosition, HashCharGrid};
use crate::vec_2d::Vec2D;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../2024/input/day08.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let grid = parse(input);
    let mut antinode_locations = HashSet::new();
    for antennas in grid.antennas_by_frequency.values() {
        for (i, pos1) in antennas.iter().enumerate() {
            for pos2 in antennas.iter().skip(i + 1) {
                let delta = *pos2 - *pos1;
                let antinode1 = *pos1 - delta;
                if grid.contains(&antinode1) {
                    antinode_locations.insert(antinode1);
                }
                let antinode2 = *pos2 + delta;
                if grid.contains(&antinode2) {
                    antinode_locations.insert(antinode2);
                }
            }
        }
    }
    antinode_locations.len()
}

fn solve_part2(input: &str) -> usize {
    let grid = parse(input);
    let mut antinode_locations = HashSet::new();
    for antennas in grid.antennas_by_frequency.values() {
        for (i, pos1) in antennas.iter().enumerate() {
            for pos2 in antennas.iter().skip(i + 1) {
                let delta = *pos2 - *pos1;
                let mut antinode = *pos2 - delta;
                while grid.contains(&antinode) {
                    antinode_locations.insert(antinode);
                    antinode -= delta;
                }
                antinode = *pos1 + delta;
                while grid.contains(&antinode) {
                    antinode_locations.insert(antinode);
                    antinode += delta;
                }
            }
        }
    }
    antinode_locations.len()
}

fn parse(input: &str) -> Grid {
    let grid = HashCharGrid::from(input);
    let mut antennas_by_frequency = HashMap::new();
    for (&pos, &c) in grid.chars.iter() {
        match c {
            '.' => {}
            c => {
                antennas_by_frequency
                    .entry(Frequency(c))
                    .or_insert_with(HashSet::new)
                    .insert(pos);
            }
        }
    }
    Grid {
        grid,
        antennas_by_frequency,
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Frequency(char);

#[derive(Debug)]
struct Grid {
    grid: HashCharGrid,
    antennas_by_frequency: HashMap<Frequency, HashSet<Vec2D>>,
}
impl CharGrid for Grid {
    fn width(&self) -> usize {
        self.grid.width()
    }
    fn height(&self) -> usize {
        self.grid.height()
    }
    fn char_at(&self, pos: &Vec2D) -> Option<&char> {
        self.grid.char_at(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    const EXAMPLE2: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

    #[test]
    fn test_part1_example() {
        assert_eq!(14, solve_part1(EXAMPLE1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(354, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(34, solve_part2(EXAMPLE1));
        assert_eq!(9, solve_part2(EXAMPLE2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1263, solve_part2(INPUT));
    }
}
