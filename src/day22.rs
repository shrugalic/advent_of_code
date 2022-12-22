use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use Facing::*;
use Instruction::*;
use Tile::*;

const INPUT: &str = include_str!("../input/day22.txt");

pub(crate) fn day22_part1() -> usize {
    let map = Map::from(INPUT);
    map.follow_path_on_plane()
}

pub(crate) fn day22_part2() -> usize {
    let map = Map::from(INPUT);
    map.follow_path_on_cube()
}

impl Map {
    fn follow_path_on_plane(&self) -> usize {
        let x_ranges_by_y = self.x_ranges_by_y();
        let y_ranges_by_x = self.y_ranges_by_x();

        // Start in left-most open tile in first row
        let mut y = 1;
        let mut x = *self
            .board
            .iter()
            .filter(|&((_, iy), tile)| iy == &y && matches!(tile, &Open))
            .map(|((x, _y), _tile)| x)
            .min()
            .unwrap();
        let mut facing = Right;
        for instruction in &self.path_description {
            // println!("At ({x}, {y}): {instruction}");
            match instruction {
                GoForward(step_count) => {
                    match facing {
                        Right => {
                            let x_range = x_ranges_by_y[y].clone().unwrap();
                            // println!("x_range at {y} = {x_range:?}");
                            if let Some(next_x) = x_range
                                .cycle()
                                .skip_while(|ix| ix != &x)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|x| self.is_open(*x, y))
                                .last()
                            {
                                x = next_x;
                            }
                        }
                        Down => {
                            let y_range = y_ranges_by_x[x].clone().unwrap();
                            // println!("y_range at {x} = {y_range:?}");
                            if let Some(next_y) = y_range
                                .cycle()
                                .skip_while(|iy| iy != &y)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|y| self.is_open(x, *y))
                                .last()
                            {
                                y = next_y;
                            }
                        }
                        Left => {
                            let x_range = x_ranges_by_y[y].clone().unwrap();
                            // println!("x_range at {y} = {x_range:?}");
                            if let Some(next_x) = x_range
                                .rev()
                                .cycle()
                                .skip_while(|ix| ix != &x)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|x| self.is_open(*x, y))
                                .last()
                            {
                                x = next_x;
                            }
                        }
                        Up => {
                            let y_range = y_ranges_by_x[x].clone().unwrap();
                            // println!("y_range at {x} = {y_range:?}");
                            if let Some(next_y) = y_range
                                .rev()
                                .cycle()
                                .skip_while(|iy| iy != &y)
                                .skip(1) // to look at the next one
                                .take(*step_count)
                                .take_while(|y| self.is_open(x, *y))
                                .last()
                            {
                                y = next_y;
                            }
                        }
                    }
                }
                TurnLeft | TurnRight => facing.turn_according_to(instruction),
            }
        }
        1000 * y + 4 * x + facing as usize
    }

    fn follow_path_on_cube(&self) -> usize {
        todo!()
    }
    fn y_ranges_by_x(&self) -> Vec<Option<RangeInclusive<Coord>>> {
        let y_ranges_by_x: Vec<Option<RangeInclusive<Coord>>> = (0..=self.width)
            .into_iter()
            .map(|x| {
                (1..=self.height)
                    .into_iter()
                    .filter(move |y| self.board.contains_key(&(x, *y)))
                    .map(|y| y..=y)
                    .reduce(|a, b| *a.start().min(b.start())..=*a.end().max(b.end()))
            })
            .collect();
        y_ranges_by_x
    }
    fn x_ranges_by_y(&self) -> Vec<Option<RangeInclusive<Coord>>> {
        let x_ranges_by_y: Vec<Option<RangeInclusive<Coord>>> = (0..=self.height)
            .into_iter()
            .map(|y| {
                (1..=self.width)
                    .into_iter()
                    .filter(move |x| self.board.contains_key(&(*x, y)))
                    .map(|x| x..=x)
                    .reduce(|a, b| *a.start().min(b.start())..=*a.end().max(b.end()))
            })
            .collect();
        x_ranges_by_y
    }
    fn is_open(&self, x: Coord, y: Coord) -> bool {
        matches!(self.board.get(&(x, y)), Some(Open))
    }
}

type Coord = usize;
#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Open,
            '#' => Wall,
            _ => unreachable!("Invalid tile '{c}'"),
        }
    }
}
impl Tile {
    fn to_char(&self) -> char {
        match self {
            Open => '.',
            Wall => '#',
        }
    }
}
#[derive(Copy, Clone)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Facing {
    fn turn_according_to(&mut self, instruction: &Instruction) {
        *self = match instruction {
            GoForward(_) => *self,
            TurnLeft => match self {
                Right => Up,
                Down => Right,
                Left => Down,
                Up => Left,
            },
            TurnRight => match self {
                Right => Down,
                Down => Left,
                Left => Up,
                Up => Right,
            },
        };
    }
}
#[derive(Debug)]
enum Instruction {
    GoForward(usize),
    TurnLeft,
    TurnRight,
}
impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s.trim() {
            "L" => TurnLeft,
            "R" => TurnRight,
            s => GoForward(
                s.parse()
                    .unwrap_or_else(|_| panic!("Got invalid number '{s}'")),
            ),
        }
    }
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GoForward(step_count) => format!("Going forward (max {step_count})"),
                TurnLeft => "turning left".to_string(),
                TurnRight => "turning right".to_string(),
            }
        )
    }
}
type Pos = (Coord, Coord);
#[derive(Debug)]
struct Map {
    board: BTreeMap<Pos, Tile>,
    width: Coord,
    height: Coord,
    path_description: Vec<Instruction>,
}
impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let (map, path) = s.split_once("\n\n").unwrap();
        let board: BTreeMap<(Coord, Coord), Tile> = map
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, tile)| ['.', '#'].contains(tile))
                    .map(move |(x, tile)| ((x + 1, y + 1), Tile::from(tile)))
            })
            .collect();

        let mut path_description = vec![];
        let mut num_i = 0;
        while num_i < path.len() {
            let mut turn_i = num_i + 1;
            while turn_i < path.len() && !path[num_i..=turn_i].ends_with(['L', 'R']) {
                turn_i += 1;
            }
            path_description.push(Instruction::from(&path[num_i..turn_i]));
            if turn_i < path.len() {
                path_description.push(Instruction::from(&path[turn_i..=turn_i]));
            }
            num_i = turn_i + 1;
        }

        let width = *board.keys().map(|(x, _y)| x).max().unwrap();
        let height = *board.keys().map(|(_x, y)| y).max().unwrap();
        Map {
            board,
            width,
            height,
            path_description,
        }
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (1..=self.height)
                .map(|y| {
                    (1..=self.width)
                        .into_iter()
                        .map(|x| self.board.get(&(x, y)).map(Tile::to_char).unwrap_or(' '))
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_example() {
        let map = Map::from(EXAMPLE);
        assert_eq!(6_032, map.follow_path_on_plane());
    }

    #[test]
    fn part1() {
        assert_eq!(31_568, day22_part1());
    }

    #[test]
    fn part2_example() {
        let map = Map::from(EXAMPLE);
        assert_eq!(5_031, map.follow_path_on_cube());
    }

    #[test]
    fn part2() {
        assert_eq!(1, day22_part2());
    }
}
