use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day12.txt");

pub(crate) fn day12_part1() -> usize {
    let map = ElevationGrid::from(INPUT);
    map.step_count_of_shortest_path_from_start_to_end()
}

pub(crate) fn day12_part2() -> usize {
    let map = ElevationGrid::from(INPUT);
    map.step_count_of_shortest_path_from_end_to_any_lowest_point()
}

struct ElevationGrid {
    grid: Vec<Vec<char>>,
}
impl ElevationGrid {
    fn step_count_of_shortest_path_from_start_to_end(mut self) -> usize {
        self.step_count_of_shortest_path('S', &['E'], |diff| diff <= 1)
    }
    fn step_count_of_shortest_path_from_end_to_any_lowest_point(mut self) -> usize {
        self.step_count_of_shortest_path('E', &['a', 'S'], |diff| diff >= -1)
    }
    fn step_count_of_shortest_path(
        &mut self,
        start: char,
        targets: &[char],
        is_valid_candidate: fn(diff: i8) -> bool,
    ) -> usize {
        let mut candidates = BinaryHeap::new();
        let start_pos = *self.find_pos_of(&[start]).first().unwrap();
        candidates.push(Reverse(State::new(start_pos)));
        let mut visited = HashSet::new();
        visited.insert(start_pos);
        while let Some(Reverse(curr)) = candidates.pop() {
            if targets.contains(self.char_at(&curr.pos)) {
                return curr.step_count;
            }
            let curr_elevation = self.elevation_at(&curr.pos);
            for next_pos in self.neighbors(&curr.pos) {
                let next_elevation = self.elevation_at(&next_pos);
                let elevation_diff = next_elevation as i8 - curr_elevation as i8;
                if is_valid_candidate(elevation_diff) && visited.insert(next_pos) {
                    candidates.push(Reverse(curr.moved_to(next_pos)))
                }
            }
        }
        unreachable!()
    }
    fn neighbors(&self, pos: &Pos) -> Vec<(usize, usize)> {
        [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .into_iter()
            .map(|(dx, dy)| (pos.0 as isize + dx, pos.1 as isize + dy))
            .filter(|(x, y)| self.contains(x, y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }
    fn contains(&self, x: &isize, y: &isize) -> bool {
        (0..self.grid[0].len() as isize).contains(x) && (0..self.grid.len() as isize).contains(y)
    }
    fn find_pos_of(&self, wanted: &[char]) -> Vec<Pos> {
        let mut matches = vec![];
        for y in 0..self.grid.len() {
            for (x, c) in self.grid[y].iter().enumerate() {
                if wanted.contains(c) {
                    matches.push((x, y));
                }
            }
        }
        matches
    }
    fn elevation_at(&self, pos: &Pos) -> Elevation {
        self.char_at(pos).to_elevation()
    }
    fn char_at(&self, pos: &Pos) -> &char {
        &self.grid[pos.1][pos.0]
    }
}
impl From<&str> for ElevationGrid {
    fn from(input: &str) -> Self {
        ElevationGrid {
            grid: input
                .trim()
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }
}
impl Display for ElevationGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

type Elevation = u8;
type Pos = (usize, usize);

trait ToElevation {
    fn to_elevation(&self) -> u8;
}
impl ToElevation for char {
    fn to_elevation(&self) -> u8 {
        (match self {
            'S' => 'a',
            'E' => 'z',
            c => *c,
        }) as Elevation
            - b'a'
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    step_count: usize,
    pos: Pos,
}
impl State {
    fn new(pos: Pos) -> Self {
        State { step_count: 0, pos }
    }
    fn moved_to(&self, pos: Pos) -> Self {
        State {
            step_count: self.step_count + 1,
            pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_example() {
        let map = ElevationGrid::from(EXAMPLE);
        assert_eq!(31, map.step_count_of_shortest_path_from_start_to_end());
    }

    #[test]
    fn part1() {
        assert_eq!(380, day12_part1());
    }

    #[test]
    fn part2_example() {
        let map = ElevationGrid::from(EXAMPLE);
        assert_eq!(
            29,
            map.step_count_of_shortest_path_from_end_to_any_lowest_point()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(375, day12_part2());
    }
}
