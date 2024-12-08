use crate::char_grid::{CharGrid, GridContainsPosition};
use crate::hash_char_grid::HashCharGrid;
use crate::vec_2d::Vec2D;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../2024/input/day06.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let grid = parse(input);
    let path = grid.patrol_until_off_grid();
    path.into_iter()
        .map(|guard| guard.pos)
        .collect::<HashSet<_>>()
        .len()
}

fn solve_part2(input: &str) -> usize {
    let mut grid = parse(input);
    let path = grid.patrol_until_off_grid();
    let mut unique_pos_path: Vec<Guard> = Vec::with_capacity(path.len());
    for guard in path {
        if !unique_pos_path.iter().any(|g| g.pos == guard.pos) {
            unique_pos_path.push(guard);
        }
    }
    let mut loop_counter = 0;
    let mut start = Guard {
        pos: grid.start_pos,
        dir: Direction::Up,
    };
    for extra_obstacle in unique_pos_path {
        grid.obstacles.insert(extra_obstacle.pos);

        let stuck_in_a_loop = grid.patrol_until_off_grid_or_stuck_in_a_loop(start);
        if stuck_in_a_loop {
            loop_counter += 1;
        }
        start = extra_obstacle; // start here next time

        grid.obstacles.remove(&extra_obstacle.pos);
    }
    loop_counter
}

fn parse(input: &str) -> Grid {
    let grid = HashCharGrid::from(input);
    let mut obstacles = HashSet::new();
    let mut start_pos = Vec2D::new(0, 0);
    for (pos, &c) in grid.chars.iter() {
        match c {
            '#' => {
                obstacles.insert(*pos);
            }
            '^' => start_pos = *pos,
            '.' => {}
            _ => unreachable!("Illegal character: {}", c),
        }
    }
    Grid {
        grid,
        start_pos,
        obstacles,
    }
}
struct Grid {
    grid: HashCharGrid,
    start_pos: Vec2D,
    obstacles: HashSet<Vec2D>,
}
impl Grid {
    fn patrol_until_off_grid(&self) -> Vec<Guard> {
        let mut path = Vec::new();
        let mut guard = Guard {
            pos: self.start_pos,
            dir: Direction::Up,
        };
        while self.contains(&guard.pos) && !path.contains(&guard) {
            path.push(guard);
            guard.take_a_step(self);
        }
        path
    }
    fn patrol_until_off_grid_or_stuck_in_a_loop(&self, mut guard: Guard) -> bool {
        let mut visited = HashSet::with_capacity(4725);
        while self.contains(&guard.pos) && !visited.contains(&guard) {
            visited.insert(guard);
            guard.take_a_step(self);
        }
        visited.contains(&guard)
    }
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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Guard {
    pos: Vec2D,
    dir: Direction,
}
impl Guard {
    fn take_a_step(&mut self, grid: &Grid) {
        let mut next = self.pos + self.dir.offset();
        while grid.contains(&next) && grid.obstacles.contains(&next) {
            self.turn_clockwise();
            next = self.pos + self.dir.offset();
        }
        self.pos += self.dir.offset()
    }
    fn turn_clockwise(&mut self) {
        self.dir = self.dir.turned_clockwise();
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
    fn offset(&self) -> Vec2D {
        match self {
            Direction::Up => Vec2D { x: 0, y: -1 },
            Direction::Down => Vec2D { x: 0, y: 1 },
            Direction::Left => Vec2D { x: -1, y: 0 },
            Direction::Right => Vec2D { x: 1, y: 0 },
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
