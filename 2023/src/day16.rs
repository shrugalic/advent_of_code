use std::collections::{BTreeSet, HashSet};
use std::fmt::{Display, Formatter};

use Direction::*;
use Tile::*;

use crate::day16::Tile::Empty;

const INPUT: &str = include_str!("../input/day16.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.energized_tile_count_when_starting_with(Beam::new(0, 0, Right))
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from(input);
    let top_down_beams = (0..grid.width).map(|x| Beam::new(x, 0, Down));
    let bottom_up_beams = (0..grid.width).map(|x| Beam::new(x, grid.height - 1, Up));
    let left_to_right_beams = (0..grid.height).map(|y| Beam::new(0, y, Right));
    let right_to_left_beams = (0..grid.height).map(|y| Beam::new(grid.width - 1, y, Left));
    top_down_beams
        .chain(bottom_up_beams)
        .chain(left_to_right_beams)
        .chain(right_to_left_beams)
        .map(|beam| grid.energized_tile_count_when_starting_with(beam))
        .max()
        .unwrap()
}

impl Grid {
    fn energized_tile_count_when_starting_with(&self, start: Beam) -> usize {
        let mut encountered = HashSet::new();
        let mut queue = vec![start];
        while let Some(beam) = queue.pop() {
            for new_beam in self.beams_after_tile_interacted_with(beam) {
                if self.grid_contains(&new_beam.pos) && !encountered.contains(&new_beam) {
                    queue.push(new_beam);
                }
            }
            encountered.insert(beam);
        }
        encountered
            .into_iter()
            .map(|beam| beam.pos)
            .collect::<BTreeSet<_>>()
            .len()
    }
    fn beams_after_tile_interacted_with(&self, beam: Beam) -> Vec<Beam> {
        let tile = &self.grid[beam.pos.y as usize][beam.pos.x as usize];
        match (tile, beam.dir) {
            (Empty, _) | (VerticalSplitter, Up | Down) | (HorizontalSplitter, Left | Right) => {
                vec![beam]
            }
            (DiagonalDownMirror, Up) | (DiagonalUpMirror, Down) => vec![beam.turned(Left)],
            (DiagonalDownMirror, Down) | (DiagonalUpMirror, Up) => vec![beam.turned(Right)],
            (DiagonalDownMirror, Left) | (DiagonalUpMirror, Right) => vec![beam.turned(Up)],
            (DiagonalDownMirror, Right) | (DiagonalUpMirror, Left) => vec![beam.turned(Down)],
            (VerticalSplitter, Left | Right) => vec![beam.turned(Up), beam.turned(Down)],
            (HorizontalSplitter, Up | Down) => vec![beam.turned(Left), beam.turned(Right)],
        }
        .into_iter()
        .map(Beam::moved_one_step)
        .collect()
    }
    fn grid_contains(&self, pos: &Position) -> bool {
        (0..self.width).contains(&pos.x) && (0..self.height).contains(&pos.y)
    }
}

impl Beam {
    fn new(x: Coord, y: Coord, dir: Direction) -> Self {
        let pos = Position { x, y };
        Beam { pos, dir }
    }
    fn turned(mut self, dir: Direction) -> Self {
        self.dir = dir;
        self
    }
    fn moved_one_step(mut self) -> Self {
        match &self.dir {
            Up => self.pos.y -= 1,
            Down => self.pos.y += 1,
            Left => self.pos.x -= 1,
            Right => self.pos.x += 1,
        }
        self
    }
}

type Coord = i8;

struct Grid {
    grid: Vec<Vec<Tile>>,
    height: Coord,
    width: Coord,
}

enum Tile {
    Empty,              // .
    DiagonalDownMirror, // \
    DiagonalUpMirror,   // /
    VerticalSplitter,   // |
    HorizontalSplitter, // -
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Beam {
    pos: Position,
    dir: Direction,
}

#[derive(Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Position {
    x: Coord,
    y: Coord,
}

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    #[default]
    Right,
    Left,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<_> = input
            .trim()
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
            .collect();
        let height = grid.len() as Coord;
        let width = grid[0].len() as Coord;
        Grid {
            grid,
            height,
            width,
        }
    }
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '\\' => DiagonalDownMirror,
            '/' => DiagonalUpMirror,
            '|' => VerticalSplitter,
            '-' => HorizontalSplitter,
            _ => unreachable!("Illegal char {value}"),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(Tile::to_string).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Empty => '.',
                DiagonalDownMirror => '\\',
                DiagonalUpMirror => '/',
                VerticalSplitter => '|',
                HorizontalSplitter => '-',
            }
        )
    }
}
impl Display for Beam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.dir, self.pos)
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:3}, {:3})", self.x, self.y)
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Up => '^',
                Down => 'v',
                Right => '>',
                Left => '<',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_part1_example() {
        assert_eq!(46, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1_example_parse_and_to_string() {
        let raw_grid = Grid::from(EXAMPLE);
        assert_eq!(EXAMPLE.trim(), format!("{raw_grid}"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(6_795, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(51, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(7_154, solve_part2(INPUT));
    }
}
