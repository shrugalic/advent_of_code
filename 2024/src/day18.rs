use crate::vec_2d::Vec2D;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../../2024/input/day18.txt");
const GRID_SIZE: usize = 71;
const BLOCK_COUNT: Time = 1024;

pub(crate) fn part1() -> Time {
    solve_part1(INPUT, GRID_SIZE, BLOCK_COUNT)
}

pub(crate) fn part2() -> String {
    solve_part2(INPUT, GRID_SIZE, BLOCK_COUNT)
}

type Time = u16;
fn solve_part1(input: &str, grid_size: usize, block_count: Time) -> Time {
    let blocked_positions = parse_blocked_positions_from(input);
    let time_blocked_grid = time_blocked_grid_from(&blocked_positions, grid_size);

    count_steps_to_exit(&time_blocked_grid, block_count).unwrap()
}

fn solve_part2(input: &str, grid_size: usize, mut start_time: Time) -> String {
    let blocked_positions = parse_blocked_positions_from(input);
    let time_blocked_grid = time_blocked_grid_from(&blocked_positions, grid_size);

    while let Some(_step_count) = count_steps_to_exit(&time_blocked_grid, start_time) {
        start_time += 1;
    }

    let last_successful_start_time = (start_time - 1) as usize;
    let blocking_pos = &blocked_positions[last_successful_start_time];
    format!("{},{}", blocking_pos.x, blocking_pos.y)
}

/// Let one more position be blocked at every time step, starting at 1.
/// The time at a position indicates at what time this position will be blocked.
fn time_blocked_grid_from(blocked_positions: &[Vec2D], grid_size: usize) -> Vec<Vec<Time>> {
    let mut grid: Vec<Vec<Time>> = vec![vec![Time::MAX as Time; grid_size]; grid_size];
    let mut time = 1;
    for Vec2D { x, y } in blocked_positions {
        grid[*y as usize][*x as usize] = time;
        time += 1;
    }
    grid
}

fn count_steps_to_exit(time_blocked_grid: &[Vec<Time>], reference_time: Time) -> Option<Time> {
    let grid_size = time_blocked_grid.len();
    let exit = Vec2D::new(grid_size - 1, grid_size - 1);
    // The value indicates at what time this position was first visited
    let mut time_visited_grid = vec![vec![Time::MAX; grid_size]; grid_size];

    let range = 0..grid_size as isize;
    let is_within_grid = |pos: &Vec2D| range.contains(&pos.x) && range.contains(&pos.y);
    let is_blocked =
        |pos: &Vec2D| reference_time >= time_blocked_grid[pos.y as usize][pos.x as usize];

    let mut queue = BinaryHeap::new();
    queue.push(TimePos {
        time: reference_time,
        pos: Vec2D::new(0, 0),
    });

    while let Some(TimePos { time, pos }) = queue.pop() {
        if pos == exit {
            let step_count = time - reference_time;
            return Some(step_count);
        } else if time >= time_visited_grid[pos.y as usize][pos.x as usize] {
            continue; // Position already visited earlier
        }

        time_visited_grid[pos.y as usize][pos.x as usize] = time;
        queue.extend(
            pos.crosswise_neighbors()
                .map(|next_pos| TimePos {
                    time: time + 1,
                    pos: next_pos,
                })
                .filter(|next| {
                    is_within_grid(&next.pos)
                        && !is_blocked(&next.pos)
                        && next.time < time_visited_grid[next.pos.y as usize][next.pos.x as usize]
                }),
        );
    }
    None
}

#[derive(Debug, Eq, PartialEq)]
struct TimePos {
    time: Time,
    pos: Vec2D,
}

/// BinaryHeap returns the greatest element, but since the question asks
/// for the lowest time, let's order by lowest time instead
impl Ord for TimePos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}
impl PartialOrd for TimePos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_blocked_positions_from(input: &str) -> Vec<Vec2D> {
    input.trim().lines().map(Vec2D::from).collect()
}

impl From<&str> for Vec2D {
    fn from(line: &str) -> Self {
        let (l, r) = line.split_once(',').unwrap();
        Vec2D::new(l.parse().unwrap(), r.parse().unwrap())
    }
}

#[expect(unused)]
fn time_grid_to_string(
    grid: &[Vec<Time>],
    filter: fn(Time) -> bool,
    true_char: char,
    false_char: char,
) -> String {
    grid.iter()
        .map(|line| {
            line.iter()
                .map(|t| if filter(*t) { true_char } else { false_char })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_GRID_SIZE: usize = 7;
    const EXAMPLE_BLOCK_COUNT: Time = 12;
    const EXAMPLE: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part1_example() {
        assert_eq!(
            22,
            solve_part1(EXAMPLE, EXAMPLE_GRID_SIZE, EXAMPLE_BLOCK_COUNT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(310, solve_part1(INPUT, GRID_SIZE, BLOCK_COUNT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            "6,1",
            solve_part2(EXAMPLE, EXAMPLE_GRID_SIZE, EXAMPLE_BLOCK_COUNT)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!("16,46", solve_part2(INPUT, GRID_SIZE, BLOCK_COUNT));
    }
}
