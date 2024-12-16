use crate::day16::Tile::Wall;
use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;
use crate::vec_tile_grid::VecTileGrid;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use Tile::{End, Start};

const INPUT: &str = include_str!("../../2024/input/day16.txt");
const STEP_COST: Cost = 1;
const TURN_COST: Cost = 1000;

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let (lowest_cost, _) = Map::from(input).find_shortest_paths();
    lowest_cost
}

fn solve_part2(input: &str) -> usize {
    let (_, count_of_tiles_on_any_best_path) = Map::from(input).find_shortest_paths();
    count_of_tiles_on_any_best_path
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Start,
    End,
    Empty,
}

#[derive(Debug)]
struct Map {
    grid: VecTileGrid<Tile>,
    start_pos: Vec2D,
    end_pos: Vec2D,
}

impl Map {
    fn tile_is_empty_at(&self, pos: &Vec2D) -> bool {
        self.grid.char_at(pos).is_some_and(|tile| tile != &Wall)
    }
    fn find_shortest_paths(&self) -> (Cost, usize) {
        let mut best_cost: Option<Cost> = None;
        let mut best_cost_by_pos_and_vel: HashMap<(Position, Velocity), Cost> = HashMap::new();
        let mut positions_on_best_paths: HashSet<Position> = HashSet::new();

        let mut queue: BinaryHeap<Reindeer> = [Reindeer::new(self.start_pos)].into_iter().collect();
        while let Some(curr) = queue.pop() {
            if curr.pos == self.end_pos {
                best_cost = Some(curr.cost);
                positions_on_best_paths.extend(curr.path);
                continue;
            }
            let best_cost_at_pos = best_cost_by_pos_and_vel
                .entry((curr.pos, curr.vel))
                .or_insert(curr.cost);
            if curr.cost > *best_cost_at_pos {
                continue;
            }
            queue.extend(
                [
                    curr.turned_left_and_stepped_forward(),
                    curr.turned_right_and_stepped_forward(),
                    curr.step_forward(),
                ]
                .into_iter()
                .filter(|next| {
                    self.tile_is_empty_at(&next.pos)
                        && best_cost.is_none_or(|best| next.cost <= best)
                }),
            );
        }
        (best_cost.unwrap(), positions_on_best_paths.len())
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let grid = VecTileGrid::from(input);
        let start = *grid.positions(|t| matches!(t, Start)).first().unwrap();
        let end = *grid.positions(|t| matches!(t, End)).first().unwrap();
        Map {
            grid,
            start_pos: start,
            end_pos: end,
        }
    }
}

type Cost = usize;
type Position = Vec2D;
type Velocity = Vec2D;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Reindeer {
    cost: Cost,
    pos: Position,
    vel: Velocity,
    path: Vec<Position>,
}

/// BinaryHeap returns the greatest element, but since the question asks
/// for the lowest cost, let's order Reindeer by lowest cost instead
impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}
impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Reindeer {
    fn new(start: Vec2D) -> Self {
        Reindeer {
            cost: 0,
            vel: Vec2D::EAST,
            pos: start,
            path: vec![start],
        }
    }

    fn step_forward(mut self) -> Self {
        self.cost += STEP_COST;
        self.pos += self.vel;
        self.path.push(self.pos);
        self
    }

    fn turned_left_and_stepped_forward(&self) -> Self {
        let mut next = self.clone();
        next.cost += TURN_COST;
        next.vel.turn_ccw();
        next.step_forward()
    }

    fn turned_right_and_stepped_forward(&self) -> Self {
        let mut next = self.clone();
        next.cost += TURN_COST;
        next.vel.turn_cw();
        next.step_forward()
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            'S' => Start,
            'E' => End,
            '.' => Tile::Empty,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_part1_example1() {
        assert_eq!(7_036, solve_part1(EXAMPLE1));
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(11_048, solve_part1(EXAMPLE2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(99_448, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(45, solve_part2(EXAMPLE1));
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(64, solve_part2(EXAMPLE2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(498, solve_part2(INPUT));
    }
}
