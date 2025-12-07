use crate::vec_2d::Vec2D;
use crate::vec_tile_grid::VecTileGrid;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../2025/input/day07.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let grid = parse(input);
    let mut curr_beams = HashSet::from([grid.lines[0].iter().position(|c| c == &'S').unwrap()]);
    let mut split_count = 0;
    for line in grid.lines.into_iter().skip(1) {
        let mut next_beams = HashSet::new();
        for x in curr_beams {
            match line[x] {
                '^' => {
                    next_beams.insert(x - 1);
                    next_beams.insert(x + 1);
                    split_count += 1;
                }
                '.' => {
                    next_beams.insert(x);
                }
                _ => unreachable!(),
            }
        }
        curr_beams = next_beams;
    }
    split_count
}

fn solve_part2(input: &str) -> usize {
    let grid = parse(input);
    count_timelines_at(
        Vec2D::new(grid.lines[0].iter().position(|c| c == &'S').unwrap(), 1),
        &grid,
        &mut HashMap::new(),
    )
}

fn count_timelines_at(
    pos: Vec2D,
    grid: &VecTileGrid<char>,
    count_by_pos: &mut HashMap<Vec2D, usize>,
) -> usize {
    if pos.y as usize == grid.lines.len() {
        return 1;
    } else if let Some(cached_count) = count_by_pos.get(&pos) {
        return *cached_count;
    }

    let time_line_count = match &grid.lines[pos.y as usize][pos.x as usize] {
        '^' => {
            count_timelines_at(pos.left_neighbor(), grid, count_by_pos)
                + count_timelines_at(pos.right_neighbor(), grid, count_by_pos)
        }
        '.' => count_timelines_at(pos.below_neighbor(), grid, count_by_pos),
        _ => unreachable!(),
    };
    count_by_pos.insert(pos, time_line_count);
    time_line_count
}

fn parse(input: &str) -> VecTileGrid<char> {
    VecTileGrid::from(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_part1_example() {
        assert_eq!(21, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1615, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(40, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(43560947406326, solve_part2(INPUT));
    }
}
