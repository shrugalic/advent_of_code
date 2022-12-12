use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day12.txt");

pub(crate) fn day12_part1() -> usize {
    let mut map = HeightMap::from(INPUT);
    map.step_count_of_shortest_path_from_start()
}

pub(crate) fn day12_part2() -> usize {
    let map = HeightMap::from(INPUT);
    map.step_count_of_shortest_path_starting_at_any_low_point()
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

struct HeightMap {
    grid: Vec<Vec<char>>,
}
impl HeightMap {
    fn step_count_of_shortest_path_from_start(&mut self) -> usize {
        self.step_count_of_shortest_path(self.start()).unwrap()
    }
    fn step_count_of_shortest_path_starting_at_any_low_point(mut self: HeightMap) -> usize {
        self.lowest_points()
            .into_iter()
            .filter_map(|start| self.step_count_of_shortest_path(start))
            .min()
            .unwrap()
    }
    fn step_count_of_shortest_path(&mut self, start: Pos) -> Option<usize> {
        let mut candidates = BinaryHeap::new();
        let end = self.end();
        candidates.push(Reverse(State::new(start)));
        let mut visited = HashSet::new();
        visited.insert(self.start());
        while let Some(Reverse(curr)) = candidates.pop() {
            if curr.pos.eq(&end) {
                return Some(curr.step_count);
            }
            let curr_elevation = self.elevation(&curr.pos);
            for next_pos in self.neighbors(&curr.pos) {
                if self.elevation(&next_pos) > curr_elevation + 1 {
                    continue;
                }
                if visited.insert(next_pos) {
                    candidates.push(Reverse(curr.moved_to(next_pos)))
                }
            }
        }
        None
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
    fn start(&self) -> Pos {
        *self.find_pos_of(&['a']).first().unwrap()
    }
    fn lowest_points(&self) -> Vec<Pos> {
        self.find_pos_of(&['S', 'a'])
    }
    fn end(&self) -> Pos {
        *self.find_pos_of(&['E']).first().unwrap()
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
    fn elevation(&self, pos: &Pos) -> Elevation {
        self.grid[pos.1][pos.0].to_elevation()
    }
}
impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        HeightMap {
            grid: input
                .trim()
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }
}
impl Display for HeightMap {
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
        let mut map = HeightMap::from(EXAMPLE);
        assert_eq!(31, map.step_count_of_shortest_path_from_start());
    }

    #[test]
    fn part1() {
        assert_eq!(380, day12_part1());
    }

    #[test]
    fn part2_example() {
        let map = HeightMap::from(EXAMPLE);
        assert_eq!(
            29,
            map.step_count_of_shortest_path_starting_at_any_low_point()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(375, day12_part2());
    }
}
