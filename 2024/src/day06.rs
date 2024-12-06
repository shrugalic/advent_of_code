use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

const INPUT: &str = include_str!("../../2024/input/day06.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let grid = parse(input);
    let (path, _) = grid.patrol_until_off_grid_or_stuck_in_a_loop();
    path.len()
}

fn solve_part2(input: &str) -> usize {
    let mut grid = parse(input);
    let (path, _) = grid.patrol_until_off_grid_or_stuck_in_a_loop();
    let mut loop_counter = 0;
    for extra_obstacle in path {
        grid.obstacles.insert(extra_obstacle);

        if let (_, _stuck_in_a_loop @ true) = grid.patrol_until_off_grid_or_stuck_in_a_loop() {
            loop_counter += 1;
        }

        grid.obstacles.remove(&extra_obstacle);
    }
    loop_counter
}

fn parse(input: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;
    let mut obstacles = HashSet::new();
    let mut start_pos = Position::new(0, 0);
    for (y, line) in input.trim().lines().enumerate() {
        if width == 0 {
            width = line.len();
        }
        height = height.max(y);
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert(Position::new(x, y));
                }
                '^' => start_pos = Position::new(x, y),
                '.' => {}
                _ => unreachable!("Illegal character: {}", c),
            }
        }
        if height > 0 {
            height += 1;
        }
    }
    Grid {
        width,
        height,
        start_pos,
        obstacles,
    }
}
struct Grid {
    width: usize,
    height: usize,
    start_pos: Position,
    obstacles: HashSet<Position>,
}
impl Grid {
    fn patrol_until_off_grid_or_stuck_in_a_loop(&self) -> (HashSet<Position>, bool) {
        let mut visited = HashSet::new();
        let mut guard = Guard {
            pos: self.start_pos,
            dir: Direction::Up,
        };
        while self.contains(guard.pos) && !visited.contains(&guard) {
            visited.insert(guard);
            guard.take_a_step(self);
        }
        let looped = visited.contains(&guard);
        let path = visited.into_iter().map(|guard| guard.pos).collect();
        (path, looped)
    }
    fn contains(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.width as isize && pos.y >= 0 && pos.y < self.height as isize
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.height)
                .map(|y| {
                    (0..self.width)
                        .map(|x| {
                            let pos = Position::new(x, y);
                            if self.obstacles.contains(&pos) {
                                '#'
                            } else if self.start_pos == pos {
                                '^'
                            } else {
                                '.'
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Guard {
    pos: Position,
    dir: Direction,
}
impl Guard {
    fn take_a_step(&mut self, grid: &Grid) {
        let mut next = self.pos + self.dir.offset();
        while grid.contains(next) && grid.obstacles.contains(&next) {
            self.turn_clockwise();
            next = self.pos + self.dir.offset();
        }
        self.pos += self.dir.offset()
    }
    fn turn_clockwise(&mut self) {
        self.dir = self.dir.turned_clockwise();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn offset(&self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
    fn turned_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1_example() {
        assert_eq!(41, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(4_374, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(6, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1_705, solve_part2(INPUT));
    }
}
