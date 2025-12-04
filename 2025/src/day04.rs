use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;
use crate::vec_tile_grid::VecTileGrid;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../../2025/input/day04.txt");

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Paper,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '@' => Tile::Paper,
            _ => unreachable!("Invalid tile: {}", value),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".",
                Tile::Paper => "@",
            }
        )
    }
}

struct Wall {
    grid: VecTileGrid<Tile>,
}

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    Wall::from(input).accessible_paper_positions().len()
}

fn solve_part2(input: &str) -> usize {
    let mut wall = Wall::from(input);
    let mut total_papers_removed = 0;
    loop {
        let positions = wall.accessible_paper_positions();
        if positions.is_empty() {
            return total_papers_removed;
        } else {
            total_papers_removed += positions.len();
            wall.remove_accessible_papers(positions);
        }
    }
}

impl Wall {
    fn accessible_paper_positions(&self) -> Vec<Vec2D> {
        self.grid
            .positions(|tile| tile == &Tile::Paper)
            .iter()
            .filter(|paper_position| {
                let neighboring_paper_count = paper_position
                    .all_neighbors()
                    .filter(|pos| {
                        self.grid
                            .tile_at(pos)
                            .map(|tile| tile == &Tile::Paper)
                            .unwrap_or(false)
                    })
                    .count();
                // A paper is accessible if it has fewer than 4 neighboring papers
                neighboring_paper_count < 4
            })
            .cloned()
            .collect::<Vec<_>>()
    }

    fn remove_accessible_papers(&mut self, positions: Vec<Vec2D>) {
        positions
            .iter()
            .for_each(|pos| *self.grid.mut_tile_at(pos).unwrap() = Tile::Empty);
    }
}

impl From<&str> for Wall {
    fn from(input: &str) -> Self {
        Wall {
            grid: VecTileGrid::from(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1_example() {
        assert_eq!(13, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1419, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(43, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8739, solve_part2(INPUT));
    }
}
