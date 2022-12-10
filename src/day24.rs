use crate::parse;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("../input/day24.txt");

pub(crate) fn day24_part1() -> usize {
    let mut maze = Maze::from(parse(INPUT));
    maze.len_of_shortest_path_to_reach_all_points_of_interest()
}

pub(crate) fn day24_part2() -> usize {
    let mut maze = Maze::from(parse(INPUT));
    maze.len_of_shortest_round_trip_to_reach_all_points_of_interest()
}

type StepCount = usize;
type PointOfInterestID = u8;
// HashSet<PointOfInterestIDs> can't be used, because it doesn't implement Hash
type VisitedPOIs = usize;

#[derive(PartialEq)]
enum Tile {
    Wall,
    OpenSpace,
    PointOfInterest(PointOfInterestID),
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::OpenSpace,
            n => Tile::PointOfInterest(n.to_digit(10).unwrap() as u8),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

struct Maze {
    grid: Vec<Vec<Tile>>,
}
impl From<Vec<&str>> for Maze {
    fn from(v: Vec<&str>) -> Self {
        Maze {
            grid: v
                .into_iter()
                .map(|line| line.chars().map(Tile::from).collect())
                .collect(),
        }
    }
}
impl Maze {
    fn len_of_shortest_path_to_reach_all_points_of_interest(&mut self) -> usize {
        self.len_of_shortest_path(false)
    }
    fn len_of_shortest_round_trip_to_reach_all_points_of_interest(&mut self) -> usize {
        self.len_of_shortest_path(true)
    }
    fn len_of_shortest_path(&mut self, return_to_start: bool) -> usize {
        let points_of_interest: Vec<_> = self.point_of_interest_positions();
        let point_of_interest_count = points_of_interest.len();
        let start_pos = Maze::starting_position(points_of_interest);

        // Keeps track of the lowest step count for each position, depending on the visited POIs
        let mut visited_pois_by_step_count_per_pos: Vec<Vec<HashMap<VisitedPOIs, StepCount>>> =
            vec![vec![HashMap::new(); self.grid[0].len()]; self.grid.len()];

        let mut queue = BinaryHeap::new();
        queue.push(State {
            pos: start_pos,
            step_count: 0,
            visited: HashSet::new(),
        });
        while let Some(mut state) = queue.pop() {
            // Visit POI if any
            if let Tile::PointOfInterest(num) = self.grid[state.pos.y][state.pos.x] {
                if state.visited.insert(num)
                    && state.visited.len() == point_of_interest_count
                    && !return_to_start
                {
                    return state.step_count;
                }
            }
            if state.pos == start_pos && state.visited.len() == point_of_interest_count {
                return state.step_count;
            }
            // Abort if this position was already visited with the same POIs and fewer steps
            let step_count = visited_pois_by_step_count_per_pos[state.pos.y][state.pos.x]
                .entry(state.visited_pois())
                .or_insert(usize::MAX);
            if &state.step_count < step_count {
                *step_count = state.step_count;
            } else {
                continue;
            }
            self.neighbors_of(&state.pos)
                .into_iter()
                .filter(|pos| self.grid[pos.y][pos.x] != Tile::Wall)
                .for_each(|pos| {
                    queue.push(State::new(pos, state.step_count + 1, state.visited.clone()));
                });
        }
        unreachable!()
    }
    fn point_of_interest_positions(&self) -> Vec<(Pos, PointOfInterestID)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if let Tile::PointOfInterest(num) = tile {
                        Some((Pos { x, y }, *num))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    fn starting_position(points_of_interest: Vec<(Pos, PointOfInterestID)>) -> Pos {
        points_of_interest
            .iter()
            .find(|(_, num)| num == &0)
            .map(|(pos, _)| *pos)
            .unwrap()
    }
    fn neighbors_of(&self, pos: &Pos) -> Vec<Pos> {
        let mut neighbors = vec![];
        if pos.x > 0 {
            neighbors.push(Pos::new(pos.x - 1, pos.y));
        }
        if pos.y > 0 {
            neighbors.push(Pos::new(pos.x, pos.y - 1));
        }
        if pos.x + 1 < self.grid[0].len() {
            neighbors.push(Pos::new(pos.x + 1, pos.y));
        }
        if pos.y + 1 < self.grid.len() {
            neighbors.push(Pos::new(pos.x, pos.y + 1));
        }
        neighbors
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    pos: Pos,
    step_count: StepCount,
    visited: HashSet<PointOfInterestID>,
}
impl State {
    fn new(pos: Pos, step_count: usize, visited: HashSet<PointOfInterestID>) -> Self {
        State {
            pos,
            step_count,
            visited,
        }
    }
    fn visited_pois(&self) -> VisitedPOIs {
        let mut pois: Vec<_> = self.visited.iter().collect();
        // Reverse sort so the 0 ends up at the end, and doesn't disappear
        pois.sort_unstable_by(|a, b| b.cmp(a));
        pois.iter().fold(0usize, |a, b| 10usize * a + **b as usize)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prioritize fewer steps over more visited POIs
        match self.step_count.cmp(&other.step_count) {
            // More visited points of interest is better
            Ordering::Equal => self.visited.len().cmp(&other.visited.len()),
            // Fewer steps is better
            step_count => step_count.reverse(),
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn part1_example() {
        let mut maze = Maze::from(parse(EXAMPLE));
        assert_eq!(
            14,
            maze.len_of_shortest_path_to_reach_all_points_of_interest()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(474, day24_part1());
    }

    #[test]
    fn part2_example() {
        let mut maze = Maze::from(parse(EXAMPLE));
        assert_eq!(
            20,
            maze.len_of_shortest_round_trip_to_reach_all_points_of_interest()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(696, day24_part2());
    }
}
