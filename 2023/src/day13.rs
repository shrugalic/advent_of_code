use std::cmp::min;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

use Tile::*;

const INPUT: &str = include_str!("../input/day13.txt");

pub(crate) fn part1() -> usize {
    sum_of_scores(INPUT)
}

pub(crate) fn part2() -> usize {
    sum_of_scores_with_smudges_replaced(INPUT)
}

fn sum_of_scores(input: &str) -> usize {
    parse(input).map(|grid| grid.score()).sum()
}

fn sum_of_scores_with_smudges_replaced(input: &str) -> usize {
    parse(input).map(Grid::score_with_smudge_fixed).sum()
}

fn parse(input: &str) -> impl Iterator<Item = Grid> + '_ {
    input.trim().split("\n\n").map(Grid::from)
}

impl Grid {
    fn score(&self) -> usize {
        let rows = self.rows_of_reflection();
        let columns = self.columns_of_reflection();
        Grid::score_rows_and_columns(rows, columns)
    }
    fn rows_of_reflection(&self) -> BTreeSet<usize> {
        self.indices_where(Grid::rows_are_reflected, self.height())
    }
    fn columns_of_reflection(&self) -> BTreeSet<usize> {
        self.indices_where(Grid::columns_are_reflected, self.width())
    }
    fn score_rows_and_columns(rows: BTreeSet<usize>, columns: BTreeSet<usize>) -> usize {
        match (rows.len(), columns.len()) {
            (1, 0) => 100 * *rows.first().unwrap(),
            (0, 1) => *columns.first().unwrap(),
            (c, r) => panic!("Unexpected count of columns ({c}) and rows ({r})"),
        }
    }
    fn indices_where(
        &self,
        are_reflections: fn((&Grid, (usize, usize))) -> bool,
        total_count: usize,
    ) -> BTreeSet<usize> {
        (1..total_count)
            .into_iter()
            .filter_map(|next| {
                let prev_count = next;
                let next_count = total_count - prev_count;
                let max_offset = min(prev_count, next_count);
                let curr = next - 1;
                let is_reflection = (0..max_offset)
                    .into_iter()
                    .map(|offset| (self, (curr - offset, next + offset)))
                    .all(are_reflections);
                is_reflection.then_some(prev_count)
            })
            .collect()
    }
    fn rows_are_reflected((grid, (y1, y2)): (&Grid, (usize, usize))) -> bool {
        grid.grid[y1] == grid.grid[y2]
    }
    fn columns_are_reflected((grid, (x1, x2)): (&Grid, (usize, usize))) -> bool {
        (0..grid.height()).all(|y| grid.grid[y][x1] == grid.grid[y][x2])
    }
    fn score_with_smudge_fixed(self) -> usize {
        let original_rows = self.rows_of_reflection();
        let original_cols = self.columns_of_reflection();

        let (new_rows, new_cols) = self.rows_and_columns_of_reflection_with_smudge_fixed();

        let different_rows: BTreeSet<_> = new_rows.difference(&original_rows).cloned().collect();
        let different_cols: BTreeSet<_> = new_cols.difference(&original_cols).cloned().collect();
        Grid::score_rows_and_columns(different_rows, different_cols)
    }
    fn rows_and_columns_of_reflection_with_smudge_fixed(
        mut self,
    ) -> (BTreeSet<usize>, BTreeSet<usize>) {
        let mut rows_of_reflection = BTreeSet::new();
        let mut cols_of_reflection = BTreeSet::new();
        for x in 0..self.width() {
            for y in 0..self.height() {
                self.toggle_tile_at(x, y);
                rows_of_reflection.append(&mut self.rows_of_reflection());
                cols_of_reflection.append(&mut self.columns_of_reflection());
                self.toggle_tile_at(x, y);
            }
        }
        (rows_of_reflection, cols_of_reflection)
    }
    fn toggle_tile_at(&mut self, x: usize, y: usize) {
        self.grid[y][x].toggle();
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl Tile {
    fn toggle(&mut self) {
        *self = match self {
            Ash => Rock,
            Rock => Ash,
        };
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Grid { grid }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match tile {
                            Ash => '.',
                            Rock => '#',
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Ash,
            '#' => Rock,
            _ => panic!("{}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_part1_example() {
        assert_eq!(405, sum_of_scores(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(34_821, sum_of_scores(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(400, sum_of_scores_with_smudges_replaced(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(36_919, sum_of_scores_with_smudges_replaced(INPUT));
    }

    #[test]
    fn difficult_example() {
        let input = "\
..###.#
####..#
##..##.
##..##.
...#..#
...####
..#.##.
##.####
##.####
####..#
...#..#
##.#..#
..#####
##.####
..#.##.
..#####
....##.";
        let grid = Grid::from(input);
        assert_eq!(1, grid.score());
        assert_eq!(5, grid.score_with_smudge_fixed());
    }
}
