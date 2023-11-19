use std::collections::hash_map::{DefaultHasher, Entry, Entry::Vacant};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::parse;

const INPUT: &str = include_str!("../input/day18.txt");

pub(crate) fn day18_part1() -> usize {
    let mut grid = LumberCollectionArea::from(parse(INPUT));
    grid.run(10);
    let (trees, lumberyards) = grid.tree_and_lumberyard_count();
    let result = trees * lumberyards;
    result
}

pub(crate) fn day18_part2() -> usize {
    let mut grid = LumberCollectionArea::from(parse(INPUT));
    grid.run(1_000_000_000);
    let (trees, lumberyards) = grid.tree_and_lumberyard_count();
    let result2 = trees * lumberyards;
    result2
}
type RuntimeMinutes = usize;
type Coord = usize;
type X = Coord;
type Y = Coord;

struct Loc {
    x: X,
    y: Y,
}

impl Loc {
    fn neighbors(&self) -> Vec<Loc> {
        let x = self.x;
        let y = self.y;
        let mut neighbors = vec![
            Loc { x: x + 1, y },
            Loc { x, y: y + 1 },
            Loc { x: x + 1, y: y + 1 },
        ];
        if x > 0 {
            neighbors.push(Loc { x: x - 1, y: y + 1 });
            neighbors.push(Loc { x: x - 1, y });
        }
        if y > 0 {
            neighbors.push(Loc { x, y: y - 1 });
            neighbors.push(Loc { x: x + 1, y: y - 1 });
        }
        if x > 0 && y > 0 {
            neighbors.push(Loc { x: x - 1, y: y - 1 });
        }
        neighbors
    }
}

#[derive(Clone, PartialEq, Hash)]
pub(crate) enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}

impl From<char> for Acre {
    fn from(c: char) -> Self {
        match c {
            '.' => Acre::OpenGround,
            '|' => Acre::Trees,
            '#' => Acre::Lumberyard,
            _ => panic!("Illegal char {}", c),
        }
    }
}

impl From<&Acre> for char {
    fn from(l: &Acre) -> Self {
        match l {
            Acre::OpenGround => '.',
            Acre::Trees => '|',
            Acre::Lumberyard => '#',
        }
    }
}

#[derive(Hash)]
pub(crate) struct LumberCollectionArea<T: Hash> {
    grid: Vec<Vec<T>>, // rows of T, indexed by [y][x]
}

impl From<Vec<&str>> for LumberCollectionArea<Acre> {
    fn from(input: Vec<&str>) -> Self {
        let grid = input
            .iter()
            .map(|line| line.chars().map(Acre::from).collect())
            .collect();
        LumberCollectionArea { grid }
    }
}

impl ToString for LumberCollectionArea<Acre> {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(char::from).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl LumberCollectionArea<Acre> {
    pub(crate) fn run(&mut self, total: RuntimeMinutes) {
        let mut elapsed_by_hash = HashMap::new();
        let mut elapsed = 0;
        while elapsed < total {
            self.run_1_minute();
            elapsed += 1;

            // Calculate a hash to detect cyclical pattern repetitions
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let hash = hasher.finish();

            match elapsed_by_hash.entry(hash) {
                Vacant(entry) => {
                    entry.insert(elapsed);
                }
                Entry::Occupied(entry) => {
                    // After a while, the pattern starts repeating itself
                    let previous = entry.get();
                    let period = elapsed - previous;
                    let cycles = (total - elapsed) / period;
                    // Skip as many full cycles as possible
                    elapsed += period * cycles;
                }
            }
        }
    }
    fn run_1_minute(&mut self) {
        let mut grid = self.grid.clone();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let neighbors: Vec<&Acre> = Loc { x, y }
                    .neighbors()
                    .iter()
                    .filter_map(|loc| self.grid.get(loc.y).and_then(|row| row.get(loc.x)))
                    .collect();
                let count = |wanted: &Acre| neighbors.iter().filter(|&&a| a == wanted).count();
                let trees = count(&Acre::Trees);
                let lumberyards = count(&Acre::Lumberyard);
                match (&self.grid[y][x], trees, lumberyards) {
                    (Acre::OpenGround, 3..=8, _) => grid[y][x] = Acre::Trees,
                    (Acre::Trees, _, 3..=8) => grid[y][x] = Acre::Lumberyard,
                    (Acre::Lumberyard, 0, _) | (Acre::Lumberyard, _, 0) => {
                        grid[y][x] = Acre::OpenGround
                    }
                    (_, _, _) => {}
                }
            }
        }
        self.grid = grid;
    }

    fn count(&self, wanted: &Acre) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&acre| acre == wanted).count())
            .sum()
    }

    fn tree_and_lumberyard_count(&self) -> (usize, usize) {
        let trees = self.count(&Acre::Trees);
        let lumberyards = self.count(&Acre::Lumberyard);
        (trees, lumberyards)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    use super::*;

    #[test]
    fn example_to_string() {
        let grid = LumberCollectionArea::from(parse(EXAMPLE[0]));
        assert_eq!(grid.to_string(), EXAMPLE[0]);
    }

    #[test]
    fn check_example_every_minute_for_10_minutes() {
        let mut grid = LumberCollectionArea::from(parse(EXAMPLE[0]));
        for expected in EXAMPLE.iter().skip(1) {
            grid.run_1_minute();
            assert_eq!(grid.to_string(), *expected);
        }
    }

    #[test]
    fn example_value_after_10_minutes() {
        let mut grid = LumberCollectionArea::from(parse(EXAMPLE[0]));
        grid.run(10);
        assert_eq!((37, 31), grid.tree_and_lumberyard_count());
    }

    #[test]
    fn part1() {
        assert_eq!(605_154, day18_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(200_364, day18_part2());
    }

    const EXAMPLE: [&str; 11] = [
        "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.",
        "\
.......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|.",
        "\
.......#..
......|#..
.|.|||....
..##|||..#
..###|||#|
...#|||||.
|||||||||.
||||||||||
||||||||||
.|||||||||",
        "\
.......#..
....|||#..
.|.||||...
..###|||.#
...##|||#|
.||##|||||
||||||||||
||||||||||
||||||||||
||||||||||",
        "\
.....|.#..
...||||#..
.|.#||||..
..###||||#
...###||#|
|||##|||||
||||||||||
||||||||||
||||||||||
||||||||||",
        "\
....|||#..
...||||#..
.|.##||||.
..####|||#
.|.###||#|
|||###||||
||||||||||
||||||||||
||||||||||
||||||||||",
        "\
...||||#..
...||||#..
.|.###|||.
..#.##|||#
|||#.##|#|
|||###||||
||||#|||||
||||||||||
||||||||||
||||||||||",
        "\
...||||#..
..||#|##..
.|.####||.
||#..##||#
||##.##|#|
|||####|||
|||###||||
||||||||||
||||||||||
||||||||||",
        "\
..||||##..
..|#####..
|||#####|.
||#...##|#
||##..###|
||##.###||
|||####|||
||||#|||||
||||||||||
||||||||||",
        "\
..||###...
.||#####..
||##...##.
||#....###
|##....##|
||##..###|
||######||
|||###||||
||||||||||
||||||||||",
        "\
.||##.....
||###.....
||##......
|##.....##
|##.....##
|##....##|
||##.####|
||#####|||
||||#|||||
||||||||||",
    ];
}
