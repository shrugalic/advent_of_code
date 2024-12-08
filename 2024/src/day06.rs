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
        grid.obstacle_row_by_column[extra_obstacle.pos.y as usize][extra_obstacle.pos.x as usize] =
            true;
        grid.obstacle_column_by_row[extra_obstacle.pos.x as usize][extra_obstacle.pos.y as usize] =
            true;

        let stuck_in_a_loop = grid.patrol_until_off_grid_or_stuck_in_a_loop(start);
        if stuck_in_a_loop {
            loop_counter += 1;
        }
        start = extra_obstacle; // start here next time

        grid.obstacle_row_by_column[extra_obstacle.pos.y as usize][extra_obstacle.pos.x as usize] =
            false;
        grid.obstacle_column_by_row[extra_obstacle.pos.x as usize][extra_obstacle.pos.y as usize] =
            false;
    }
    loop_counter
}

fn parse(input: &str) -> Grid {
    let grid = HashCharGrid::from(input);
    let mut obstacle_column_by_row: Vec<Vec<bool>> = vec![vec![false; grid.height()]; grid.width()];
    let mut obstacle_row_by_column: Vec<Vec<bool>> = vec![vec![false; grid.width()]; grid.height()];
    let mut start_pos = Vec2D::new(0, 0);
    for (pos, &c) in grid.chars.iter() {
        match c {
            '#' => {
                let x = pos.x as usize;
                let y = pos.y as usize;
                obstacle_column_by_row[x][y] = true;
                obstacle_row_by_column[y][x] = true;
            }
            '^' => start_pos = *pos,
            '.' => {}
            _ => unreachable!("Illegal character: {}", c),
        }
    }
    Grid {
        grid,
        start_pos,
        obstacle_column_by_row,
        obstacle_row_by_column,
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashCharGrid,
    start_pos: Vec2D,
    obstacle_column_by_row: Vec<Vec<bool>>,
    obstacle_row_by_column: Vec<Vec<bool>>,
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

            let dir = guard.dir;
            let (sub_path, out_of_bounds) = guard.step_forward_as_far_as_possible_or_turn(self);
            for pos in sub_path {
                path.push(Guard { pos, dir });
            }
            if out_of_bounds {
                break;
            }
        }
        path
    }
    fn patrol_until_off_grid_or_stuck_in_a_loop(&self, mut guard: Guard) -> bool {
        let mut visited: Vec<Vec<Vec<bool>>> =
            vec![vec![vec![false; self.height()]; self.width()]; 4];

        while self.contains(&guard.pos)
            && !visited[guard.dir as u8 as usize][guard.pos.x as usize][guard.pos.y as usize]
        {
            visited[guard.dir as u8 as usize][guard.pos.x as usize][guard.pos.y as usize] = true;

            let prev_dir = guard.dir;
            let (sub_path, out_of_bounds) = guard.step_forward_as_far_as_possible_or_turn(self);
            for pos in sub_path {
                visited[prev_dir as u8 as usize][pos.x as usize][pos.y as usize] = true;
            }
            if out_of_bounds {
                return false;
            }
        }
        self.contains(&guard.pos)
            && visited[guard.dir as u8 as usize][guard.pos.x as usize][guard.pos.y as usize]
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
    fn step_forward_as_far_as_possible_or_turn(&mut self, grid: &Grid) -> (Vec<Vec2D>, bool) {
        let mut path: Vec<Vec2D> = Vec::new();
        const MIN_X: isize = 0;
        const MIN_Y: isize = 0;
        let max_y = grid.height() as isize - 1;
        let max_x = grid.width() as isize - 1;
        let x = self.pos.x as usize;
        let y = self.pos.y as usize;
        match self.dir {
            Direction::Up => {
                self.pos.y = grid.obstacle_column_by_row[x][..y]
                    .iter()
                    .rposition(|b| *b)
                    .map(|next_obstacle_y| next_obstacle_y + 1)
                    .unwrap_or(MIN_Y as usize) as isize;
                for ny in (self.pos.y as usize..y).rev() {
                    path.push(Vec2D::new(x, ny));
                }
            }
            Direction::Down => {
                self.pos.y = grid.obstacle_column_by_row[x][y + 1..]
                    .iter()
                    .position(|b| *b)
                    .map(|o_y| o_y + y + 1) // account for prior elements
                    .map(|next_obstacle_y| next_obstacle_y - 1)
                    .unwrap_or(max_y as usize) as isize;
                for ny in (y + 1)..=self.pos.y as usize {
                    path.push(Vec2D::new(x, ny));
                }
            }
            Direction::Left => {
                self.pos.x = grid.obstacle_row_by_column[y][..x]
                    .iter()
                    .rposition(|b| *b)
                    .map(|next_obstacle_x| next_obstacle_x + 1)
                    .unwrap_or(MIN_X as usize) as isize;
                for nx in (self.pos.x as usize..x).rev() {
                    path.push(Vec2D::new(nx, y));
                }
            }
            Direction::Right => {
                self.pos.x = grid.obstacle_row_by_column[y][x + 1..]
                    .iter()
                    .position(|b| *b)
                    .map(|o_x| o_x + x + 1) // account for prior elements
                    .map(|next_obstacle_x| next_obstacle_x - 1)
                    .unwrap_or(max_x as usize) as isize;
                for nx in x + 1..=self.pos.x as usize {
                    path.push(Vec2D::new(nx, y));
                }
            }
        }
        let mut next = self.pos + self.dir.offset();
        while grid.contains(&next) && grid.obstacle_column_by_row[next.x as usize][next.y as usize]
        {
            self.turn_clockwise();
            next = self.pos + self.dir.offset();
        }
        // Positions are not technically out-of-bounds, but right on the border and headed outside
        let out_of_bounds = self.pos.x == MIN_X
            || self.pos.x == max_x
            || self.pos.y == MIN_Y
            || self.pos.y == max_y;
        (path, out_of_bounds)
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
