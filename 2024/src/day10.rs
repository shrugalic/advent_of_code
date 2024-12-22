use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;
use crate::vec_tile_grid::VecTileGrid;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../2024/input/day10.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let map = parse(input);
    let starting_points = map.low_points();
    let mut score = 0;
    for start in starting_points {
        score += map.count_endpoints_reachable_from(start);
    }
    score
}

fn solve_part2(input: &str) -> usize {
    let map = parse(input);
    let starting_points = map.low_points();
    let mut rating = 0;
    for start in starting_points {
        rating += map.count_unique_paths_starting_at(start);
    }
    rating
}

fn parse(input: &str) -> Map {
    Map {
        grid: VecTileGrid::from(input),
    }
}

type Path = Vec<Vec2D>;
type Elevation = u8;

#[derive(Debug)]
struct Map {
    grid: VecTileGrid<char>,
}

impl Map {
    fn low_points(&self) -> Vec<Vec2D> {
        self.grid.positions(|c| c == &'0')
    }

    fn count_endpoints_reachable_from(&self, start: Vec2D) -> usize {
        let mut current_positions = vec![start];
        let mut endpoints = HashSet::new();
        while let Some(pos) = current_positions.pop() {
            let elevation = self.elevation_at(&pos).unwrap();
            if elevation == 9 {
                endpoints.insert(pos);
                continue;
            }
            let neighbors = self.allowed_neighbors(&pos, elevation);
            current_positions.extend(neighbors);
        }
        endpoints.len()
    }

    fn count_unique_paths_starting_at(&self, start: Vec2D) -> usize {
        let mut paths_in_progress: Vec<Path> = vec![vec![start]];
        let mut unique_paths = HashSet::new();
        while let Some(path) = paths_in_progress.pop() {
            let pos = path.last().unwrap();
            let elevation = self.elevation_at(pos).unwrap();
            if elevation == 9 {
                unique_paths.insert(path);
                continue;
            }
            for neighbor in self.allowed_neighbors(pos, elevation) {
                let mut next_path = path.clone();
                next_path.push(neighbor);
                paths_in_progress.push(next_path);
            }
        }
        unique_paths.len()
    }

    fn elevation_at(&self, pos: &Vec2D) -> Option<Elevation> {
        self.grid
            .char_at(pos)
            .and_then(|c| c.to_digit(10))
            .map(|e| e as Elevation)
    }

    fn allowed_neighbors(&self, curr_pos: &Vec2D, curr_elev: Elevation) -> Vec<Vec2D> {
        curr_pos
            .crosswise_neighbors()
            .filter(|next_pos| {
                self.elevation_at(next_pos)
                    .is_some_and(|elev| elev == curr_elev + 1)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part1_example() {
        assert_eq!(36, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(593, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(81, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1192, solve_part2(INPUT));
    }
}
