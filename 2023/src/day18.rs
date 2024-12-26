use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, RangeInclusive};

use Direction::*;

const INPUT: &str = include_str!("../input/day18.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &'static str) -> usize {
    let instructions = parse(input).collect();
    count_of_border_tiles_plus_inside_tiles(instructions)
}

fn solve_part2(input: &'static str) -> usize {
    let instructions = parse(input).map(Instruction::convert).collect();
    count_of_border_tiles_plus_inside_tiles(instructions)
}

fn count_of_border_tiles_plus_inside_tiles(instructions: Vec<Instruction>) -> usize {
    let mut ground = Ground::default();
    ground.dig_according_to(instructions);
    // println!("{ground}");
    ground.border_tile_count() + ground.inside_tile_count()
}

fn parse(input: &'static str) -> impl Iterator<Item = Instruction<'static>> {
    input.trim().lines().map(Instruction::from)
}

type Coord = i32;
type Distance = i32;

struct Instruction<'a> {
    direction: Direction,
    distance: Distance,
    color: Color<'a>,
}

#[derive(Default, Debug)]
struct Ground {
    horizontal_lines_by_y: BTreeMap<Coord, Vec<RangeInclusive<Coord>>>,
    vertical_lines_by_x: BTreeMap<Coord, Vec<RangeInclusive<Coord>>>,
}

#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: Coord,
    y: Coord,
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Copy, Clone, Debug)]
struct Color<'a> {
    hex: &'a str,
}

impl Ground {
    fn dig_according_to(&mut self, mut dig_plan: Vec<Instruction>) {
        let mut pos = Position::default();
        let range_from = |a: Coord, b: Coord| a.min(b)..=a.max(b);
        for instruction in dig_plan.drain(..) {
            let offset = Position::from(&instruction.direction);
            let delta = offset.scaled_by(instruction.distance);
            let end = pos + delta;
            let range;
            match instruction.direction {
                Left | Right => {
                    range = range_from(pos.x, end.x);
                    self.horizontal_lines_by_y
                        .entry(pos.y)
                        .or_default()
                        .push(range.clone());
                }
                Up | Down => {
                    range = range_from(pos.y, end.y);
                    self.vertical_lines_by_x
                        .entry(pos.x)
                        .or_default()
                        .push(range.clone());
                }
            }
            pos = end;
        }
    }

    fn border_tile_count(&self) -> usize {
        let line_len = |r: &RangeInclusive<Coord>| (r.end() - r.start()) as usize;
        let sum_of_line_lens = |(_, lines): (&Coord, &Vec<RangeInclusive<Coord>>)| {
            lines.iter().map(line_len).sum::<usize>()
        };
        self.horizontal_lines_by_y
            .iter()
            .map(sum_of_line_lens)
            .sum::<usize>()
            + self
                .vertical_lines_by_x
                .iter()
                .map(sum_of_line_lens)
                .sum::<usize>()
    }
    fn x_coords_of_vertical_lines_at(&self, y: Coord) -> Vec<&Coord> {
        self.vertical_lines_by_x
            .iter()
            .filter_map(|(x, v_lines)| {
                v_lines
                    .iter()
                    .any(|y_range| y_range.contains(&y))
                    .then_some(x)
            })
            .collect()
    }
    fn inside_tile_count(&self) -> usize {
        let y_range = self.y_range();
        let mut total = 0usize;
        for y in y_range {
            let is_h_line = |start, end| {
                self.horizontal_lines_by_y
                    .get(&y)
                    .map(|h_lines| {
                        h_lines
                            .iter()
                            .any(|x_range| x_range.start() == &start && x_range.end() == &end)
                    })
                    .unwrap_or(false)
            };

            let mut is_inside = false;
            let xs_at_y = self.x_coords_of_vertical_lines_at(y);
            for pair in xs_at_y.windows(2) {
                is_inside = !is_inside;
                let x_left = *pair[0];
                let x_right = *pair[1];
                if is_h_line(x_left, x_right) {
                    let from_up = self.is_border(&x_left, &(y - 1));
                    let from_down = self.is_border(&x_left, &(y + 1));
                    let to_up = self.is_border(&x_right, &(y - 1));
                    let to_down = self.is_border(&x_right, &(y + 1));
                    if from_up && to_down || from_down && to_up {
                        is_inside = !is_inside;
                    }
                } else if is_inside {
                    total += (x_right - x_left - 1) as usize
                }
            }
        }
        total
    }
    fn x_range(&self) -> RangeInclusive<Coord> {
        let xs: Vec<_> = self.vertical_lines_by_x.keys().collect();
        Ground::range_from(&xs)
    }
    fn y_range(&self) -> RangeInclusive<Coord> {
        let ys: Vec<_> = self.horizontal_lines_by_y.keys().collect();
        Ground::range_from(&ys)
    }
    fn range_from(coords: &[&Coord]) -> RangeInclusive<Coord> {
        let start = **coords.iter().min().unwrap();
        let end = **coords.iter().max().unwrap();
        start..=end
    }
    fn is_border(&self, x: &Coord, y: &Coord) -> bool {
        self.vertical_lines_by_x
            .get(x)
            .iter()
            .any(|v_lines| v_lines.iter().any(|y_range| y_range.contains(y)))
            || self
                .horizontal_lines_by_y
                .get(y)
                .iter()
                .any(|h_line| h_line.iter().any(|x_range| x_range.contains(x)))
    }
}

impl Position {
    fn new(x: Coord, y: Coord) -> Self {
        Position { x, y }
    }
    fn scaled_by(&self, factor: Distance) -> Self {
        Position::new(self.x * factor, self.y * factor)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<'i> Instruction<'i> {
    fn convert(self) -> Instruction<'i> {
        let distance = &self.color.hex[0..5];
        let direction = match &self.color.hex[5..=5] {
            "0" => Right,
            "1" => Down,
            "2" => Left,
            "3" => Up,
            d => unreachable!("Invalid direction {d}"),
        };
        let distance = Distance::from_str_radix(distance, 16).expect("valid hex");
        Instruction {
            direction,
            distance,
            color: self.color,
        }
    }
}

impl From<&Direction> for Position {
    fn from(direction: &Direction) -> Self {
        match direction {
            Right => Position::new(1, 0),
            Down => Position::new(0, 1),
            Left => Position::new(-1, 0),
            Up => Position::new(0, -1),
        }
    }
}
impl From<&'static str> for Instruction<'_> {
    fn from(line: &'static str) -> Self {
        let mut parts = line.split_ascii_whitespace();
        let direction = Direction::from(parts.next().unwrap());
        let distance: Distance = parts.next().unwrap().parse().unwrap();
        let color = Color::from(parts.next().unwrap());
        Instruction {
            direction,
            distance,
            color,
        }
    }
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Right,
            "D" => Down,
            "L" => Left,
            "U" => Up,
            _ => unreachable!("Illegal direction '{value}'"),
        }
    }
}
impl From<&'static str> for Color<'_> {
    fn from(line: &'static str) -> Self {
        // example: (#70c710)
        // indices: 012345678
        let hex = &line[2..8];
        Color { hex }
    }
}

impl Display for Instruction<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.direction, self.distance, self.color)
    }
}
impl Display for Color<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex)
    }
}
impl Display for Ground {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_range = self.x_range();
        let s = self
            .y_range()
            .map(|y| {
                x_range
                    .clone()
                    .map(|x| if self.is_border(&x, &y) { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", s)
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Up => 'U',
                Down => 'D',
                Right => 'R',
                Left => 'L',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part1_example() {
        assert_eq!(62, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(36_725, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(952_408_144_115, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(97_874_103_749_720, solve_part2(INPUT));
    }
}
