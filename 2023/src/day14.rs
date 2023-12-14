use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem;

use Direction::*;
use Tile::*;

const INPUT: &str = include_str!("../input/day14.txt");

pub(crate) fn part1() -> usize {
    total_load_after_tilting_north(INPUT)
}

pub(crate) fn part2() -> usize {
    total_load_after_a_billion_tilt_cycles(INPUT)
}

fn total_load_after_tilting_north(input: &str) -> usize {
    Grid::from(input).tilt(North).total_load()
}

fn total_load_after_a_billion_tilt_cycles(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut log = HashMap::new();
    let total = 1_000_000_000;
    log.insert(grid.to_string(), 0);
    for current in 1..=total {
        grid = grid.cycle();
        if let Some(previous) = log.insert(grid.to_string(), current) {
            let remaining = total - current;
            let period = current - previous;
            let remainder = remaining % period;
            for _ in 0..remainder {
                grid = grid.cycle();
            }
            return grid.total_load();
        }
    }
    unreachable!()
}

impl Grid {
    fn cycle(self) -> Self {
        self.tilt(North).tilt(West).tilt(South).tilt(East)
    }
    fn tilt(mut self, direction: Direction) -> Grid {
        match direction {
            North => {
                for x in 0..self.width() {
                    let mut ceiling = 0;
                    for y in 0..self.height() {
                        ceiling = match &self.grid[y][x] {
                            EmptySpace => ceiling,
                            CubeShapedRocks => y + 1,
                            RoundedRocks => {
                                if ceiling < y {
                                    mem::swap(&mut self.grid[ceiling][x], &mut RoundedRocks);
                                    mem::swap(&mut self.grid[y][x], &mut EmptySpace);
                                    ceiling + 1
                                } else {
                                    y + 1
                                }
                            }
                        };
                    }
                }
            }
            South => {
                for x in 0..self.width() {
                    let mut floor = self.height() - 1;
                    for y in (0..self.height()).rev() {
                        floor = match &self.grid[y][x] {
                            EmptySpace => floor,
                            CubeShapedRocks => y.saturating_sub(1),
                            RoundedRocks => {
                                if y < floor {
                                    mem::swap(&mut self.grid[floor][x], &mut RoundedRocks);
                                    mem::swap(&mut self.grid[y][x], &mut EmptySpace);
                                    floor - 1
                                } else {
                                    y.saturating_sub(1)
                                }
                            }
                        };
                    }
                }
            }
            East => {
                for y in 0..self.height() {
                    let mut right = self.width() - 1;
                    for x in (0..self.width()).rev() {
                        right = match &self.grid[y][x] {
                            EmptySpace => right,
                            CubeShapedRocks => x.saturating_sub(1),
                            RoundedRocks => {
                                if x < right {
                                    self.grid[y].swap(x, right);
                                    right - 1
                                } else {
                                    x.saturating_sub(1)
                                }
                            }
                        };
                    }
                }
            }
            West => {
                for y in 0..self.height() {
                    let mut left = 0;
                    for x in 0..self.width() {
                        left = match &self.grid[y][x] {
                            EmptySpace => left,
                            CubeShapedRocks => x + 1,
                            RoundedRocks => {
                                if left < x {
                                    self.grid[y].swap(left, x);
                                    left + 1
                                } else {
                                    x + 1
                                }
                            }
                        };
                    }
                }
            }
        }
        self
    }
    fn total_load(&self) -> usize {
        let height = self.height();
        let weight = |y: usize| -> usize { height - y };
        self.grid
            .iter()
            .enumerate()
            .map(|(y, line)| weight(y) * line.iter().filter(|tile| tile == &&RoundedRocks).count())
            .sum()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }
}

#[derive(PartialEq)]
enum Tile {
    RoundedRocks,
    CubeShapedRocks,
    EmptySpace,
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let grid = value
            .trim()
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Grid { grid }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| { row.iter().map(Tile::to_string).collect::<String>() })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => RoundedRocks,
            '#' => CubeShapedRocks,
            '.' => EmptySpace,
            _ => unreachable!("Illegal char '{c}'"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RoundedRocks => 'O',
                CubeShapedRocks => '#',
                EmptySpace => '.',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_part1_example() {
        assert_eq!(136, total_load_after_tilting_north(EXAMPLE));
    }

    #[test]
    fn test_part1_example_tilt_north() {
        let grid = Grid::from(EXAMPLE).tilt(North);
        assert_eq!(
            grid.to_string(),
            "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(109_098, total_load_after_tilting_north(INPUT));
    }

    #[test]
    fn test_part2_example_1_cycle() {
        let grid = Grid::from(EXAMPLE).cycle();
        assert_eq!(
            grid.to_string(),
            "\
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
        );
    }

    #[test]
    fn test_part2_example_2_cycles() {
        let grid = Grid::from(EXAMPLE).cycle().cycle();
        assert_eq!(
            grid.to_string(),
            "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
        );
    }

    #[test]
    fn test_part2_example_3_cycles() {
        let grid = Grid::from(EXAMPLE).cycle().cycle().cycle();
        assert_eq!(
            grid.to_string(),
            "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
        );
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(64, total_load_after_a_billion_tilt_cycles(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(100_064, total_load_after_a_billion_tilt_cycles(INPUT));
    }
}
