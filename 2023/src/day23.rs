use petgraph::algo;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::{iter, vec};

use petgraph::graph::DiGraph;
use petgraph::prelude::*;

use Tile::*;

const INPUT: &str = include_str!("../input/day23.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    // Grid::from(input).longest_route(Part::One)
    let grid = Grid::from(input);

    // let graph = grid.convert_to_optimized_graph(false);
    let graph = grid.convert_to_simple_graph(false);
    // graph.longest_petgraph_path()
    graph.longest_path()
}

fn solve_part2(input: &str) -> usize {
    let grid = Grid::from(input);
    // let graph = grid.convert_to_optimized_graph(false);
    let graph = grid.convert_to_simple_graph(true);
    // graph.longest_petgraph_path()
    graph.longest_path()
}

type Coord = u8;

enum Tile {
    Path,
    Forest,
    Slope(Direction),
}
struct Grid {
    grid: Vec<Vec<Tile>>,
    height: Coord,
    width: Coord,
}

#[derive(Default, Debug, Eq, PartialEq, Clone)]
struct Traveler {
    distance: usize,
    pos: Position,
    prev_pos: Option<Position>,
    prev_node: Position,
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Position {
    x: Coord,
    y: Coord,
}

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    #[default]
    Down,
    Right,
    Left,
}

type Cost = u8;
type Node = Position;
type Connections = BTreeMap<Node, Cost>;
type ConnectionsByNode = BTreeMap<Node, Connections>;

#[derive(Debug)]
struct Graph {
    connections_by_node: ConnectionsByNode,
    start: Node,
    exit: Node,
}

impl Graph {
    #[allow(dead_code)]
    fn longest_petgraph_path(&self) -> usize {
        let mut graph = DiGraph::<&Position, Cost>::new();
        let index_by_pos: HashMap<&Position, NodeIndex> = self
            .connections_by_node
            .keys()
            .chain(iter::once(&self.exit))
            .map(|node| (node, graph.add_node(node)))
            .collect();
        let pos_by_index: HashMap<NodeIndex, &Position> =
            index_by_pos.iter().map(|(node, i)| (*i, *node)).collect();

        for (a, conns) in &self.connections_by_node {
            let a = index_by_pos[&a];
            for (b, cost) in conns {
                let b = index_by_pos[&b];
                graph.add_edge(a, b, *cost);
            }
        }

        let ways = algo::all_simple_paths::<Vec<_>, _>(
            &graph,
            index_by_pos[&self.start],
            index_by_pos[&self.exit],
            0,
            None,
        );
        let mut totals = vec![];
        for way in ways {
            // println!("---");
            let mut total = 0_usize;
            for i in way.windows(2) {
                let a = pos_by_index[&i[0]];
                let b = pos_by_index[&i[1]];
                let c = self.connections_by_node[a][b];

                // println!("{a} to {b}: {c}");
                total += c as usize;
            }
            // println!("total = {total}");
            totals.push(total);
        }

        *totals.iter().max().unwrap()
    }
    fn longest_path(&self) -> usize {
        let mut path_lens: Vec<_> = vec![];
        let mut queue = vec![NodeTraveler::new(&self.start, 0)];
        while let Some(traveler) = queue.pop() {
            if traveler.current_node() == &self.exit {
                path_lens.push(traveler.distance);
                continue;
            }
            for (neighbor, cost) in self
                .connections_by_node
                .get(traveler.current_node())
                .unwrap()
            {
                if !traveler.visited.contains(&neighbor) {
                    queue.push(traveler.moved_to(neighbor, *cost));
                }
            }
        }
        *path_lens.iter().max().unwrap()
    }
}

#[derive(Clone)]
struct NodeTraveler<'n> {
    visited: Vec<&'n Node>,
    distance: usize,
}
impl<'n> NodeTraveler<'n> {
    fn new(node: &'n Node, distance: usize) -> Self {
        let visited = vec![node];
        NodeTraveler { visited, distance }
    }
    fn current_node(&self) -> &Node {
        self.visited.last().unwrap()
    }
    fn moved_to(&self, neighbor: &'n Node, cost: Cost) -> Self {
        let mut next = self.clone();
        next.visited.push(neighbor);
        next.distance += cost as usize;
        next
    }
}

impl Grid {
    fn convert_to_simple_graph(&self, is_part_2: bool) -> Graph {
        let mut connections_by_node = ConnectionsByNode::new();
        let start = Position::new(1, 0);
        let exit = Position::new(self.width - 2, self.height - 1);
        let mut nodes: HashSet<Position> = HashSet::new();
        nodes.insert(start);
        nodes.insert(exit);

        let mut queue = Vec::from([Traveler::new(start)]);
        while let Some(curr_traveler) = queue.pop() {
            // println!("At pos {}", curr_traveler.pos);
            if curr_traveler.pos == exit {
                let prev_node = curr_traveler.prev_node;
                // println!(
                //     "Found way from {prev_node} to exit {exit} with cost {}",
                //     curr_traveler.distance
                // );
                let curr_distance = curr_traveler.distance as Cost;
                connections_by_node
                    .entry(prev_node)
                    .or_default()
                    .entry(exit)
                    .and_modify(|prev_distance| {
                        if *prev_distance <= curr_distance {
                            *prev_distance = curr_distance;
                        } else {
                            println!(
                                "would have overwritten {prev_distance} with {curr_distance}!"
                            );
                        }
                    })
                    .or_insert(curr_distance);
                continue;
            }
            let next_positions = curr_traveler.next_valid_positions(self);
            let mut next_travelers = if is_part_2 {
                curr_traveler.next_travelers2(next_positions, self)
            } else {
                curr_traveler.next_travelers(next_positions, self)
            };

            let curr_node = curr_traveler.pos;
            let curr_distance = curr_traveler.distance as Cost;

            // Update distances from previous node
            if curr_traveler.pos != curr_traveler.prev_node {
                let prev_node = curr_traveler.prev_node;
                // println!(
                //     "Found way from {prev_node} to {curr_node} with cost {}",
                //     curr_traveler.distance
                // );
                connections_by_node
                    .entry(prev_node)
                    .or_default()
                    .entry(curr_node)
                    .and_modify(|prev_distance| {
                        if *prev_distance <= curr_distance {
                            *prev_distance = curr_distance;
                        } else {
                            println!(
                                "would have overwritten {prev_distance} with {curr_distance}!"
                            );
                        }
                    })
                    .or_insert(curr_distance);
                if is_part_2 {
                    // Enter reverse connection
                    connections_by_node
                        .entry(curr_node)
                        .or_default()
                        .entry(prev_node)
                        .and_modify(|prev_distance| {
                            if *prev_distance <= curr_distance {
                                *prev_distance = curr_distance;
                            } else {
                                println!(
                                    "would have overwritten {prev_distance} with {curr_distance}!"
                                );
                            }
                        })
                        .or_insert(curr_distance);
                }
                // If we came back to an already visited node, definitely
                // don't queue travelers for it again
                if !nodes.insert(curr_node) {
                    continue;
                }
            }

            next_travelers.iter_mut().for_each(|next_traveler| {
                // Save node to travelers for later retrieval
                next_traveler.prev_node = curr_node;
                // Reset distance at node
                next_traveler.distance -= curr_traveler.distance;
            });

            queue.extend(next_travelers);
        }
        Graph {
            connections_by_node,
            start,
            exit,
        }
    }
    #[allow(dead_code)]
    fn convert_to_optimized_graph(&self, is_part_2: bool) -> Graph {
        let mut connections_by_node = ConnectionsByNode::new();
        let start = Position::new(1, 0);
        let exit = Position::new(self.width - 2, self.height - 1);
        let mut nodes: HashSet<Position> = HashSet::new();
        nodes.insert(start);
        nodes.insert(exit);

        let mut queue = Vec::from([Traveler::new(start)]);
        while let Some(curr_traveler) = queue.pop() {
            // println!("At pos {}", curr_traveler.pos);
            if curr_traveler.pos == exit {
                let prev_node = curr_traveler.prev_node;
                // println!(
                //     "Found way from {prev_node} to exit {exit} with cost {}",
                //     curr_traveler.distance
                // );
                let curr_distance = curr_traveler.distance as Cost;
                connections_by_node
                    .entry(prev_node)
                    .or_default()
                    .entry(exit)
                    .and_modify(|prev_distance| {
                        if *prev_distance <= curr_distance {
                            *prev_distance = curr_distance;
                        } else {
                            println!(
                                "would have overwritten {prev_distance} with {curr_distance}!"
                            );
                        }
                    })
                    .or_insert(curr_distance);
                continue;
            }
            let next_positions = curr_traveler.next_valid_positions(self);
            let mut next_travelers = if is_part_2 {
                curr_traveler.next_travelers2(next_positions, self)
            } else {
                curr_traveler.next_travelers(next_positions, self)
            };
            if next_travelers.len() > 1 {
                // Path splits, so curr_traveler is at a node
                let curr_node = curr_traveler.pos;
                let curr_distance = curr_traveler.distance as Cost;

                // Update distances from previous node
                let prev_node = curr_traveler.prev_node;
                // println!(
                //     "Found way from {prev_node} to {curr_node} with cost {}",
                //     curr_traveler.distance
                // );
                connections_by_node
                    .entry(prev_node)
                    .or_default()
                    .entry(curr_node)
                    .and_modify(|prev_distance| {
                        if *prev_distance <= curr_distance {
                            *prev_distance = curr_distance;
                        } else {
                            println!(
                                "would have overwritten {prev_distance} with {curr_distance}!"
                            );
                        }
                    })
                    .or_insert(curr_distance);
                if is_part_2 {
                    // Enter reverse connection
                    connections_by_node
                        .entry(curr_node)
                        .or_default()
                        .entry(prev_node)
                        .and_modify(|prev_distance| {
                            if *prev_distance <= curr_distance {
                                *prev_distance = curr_distance;
                            } else {
                                println!(
                                    "would have overwritten {prev_distance} with {curr_distance}!"
                                );
                            }
                        })
                        .or_insert(curr_distance);
                }
                // If we came back to an already visited node, definitely
                // don't queue travelers for it again
                if !nodes.insert(curr_node) {
                    continue;
                }

                next_travelers.iter_mut().for_each(|next_traveler| {
                    // Save node to travelers for later retrieval
                    next_traveler.prev_node = curr_node;
                    // Reset distance at node
                    next_traveler.distance -= curr_traveler.distance;
                });
            }
            queue.extend(next_travelers);
        }
        Graph {
            connections_by_node,
            start,
            exit,
        }
    }
    #[allow(dead_code)]
    fn longest_route(&self) -> usize {
        let start = Position::new(1, 0);
        let exit = Position::new(self.width - 2, self.height - 1);
        let mut queue = Vec::new();
        queue.push(Traveler::new(start));

        let mut path_lengths = vec![];
        while let Some(curr_traveler) = queue.pop() {
            if curr_traveler.pos == exit {
                path_lengths.push(curr_traveler.distance);
                continue;
            }
            let next_positions = curr_traveler.next_valid_positions(self);
            for crucible in curr_traveler.next_travelers(next_positions, self) {
                queue.push(crucible);
            }
        }
        dbg!(&path_lengths);
        *path_lengths.iter().max().unwrap()
    }
    fn tile_at(&self, pos: &Position) -> Option<&Tile> {
        self.grid
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
    }
    fn contains(&self, pos: &Position) -> bool {
        (0..self.width).contains(&pos.x) && (0..self.width).contains(&pos.y)
    }
}

impl Traveler {
    fn new(pos: Position) -> Self {
        Traveler {
            pos,
            prev_node: pos,
            ..Default::default()
        }
    }

    fn next_valid_positions(&self, grid: &Grid) -> Vec<Position> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ]
        .into_iter()
        .map(|dir| self.pos.moved_into(&dir))
        .filter(|next_pos| grid.contains(next_pos))
        .filter(|next_pos| Some(*next_pos) != self.prev_pos)
        .collect()
    }

    fn next_travelers(&self, positions: Vec<Position>, grid: &Grid) -> Vec<Traveler> {
        positions
            .into_iter()
            .filter_map(|next_pos| {
                let dir = match (
                    next_pos.x as i16 - self.pos.x as i16,
                    next_pos.y as i16 - self.pos.y as i16,
                ) {
                    (-1, 0) => Direction::Left,
                    (1, 0) => Direction::Right,
                    (0, -1) => Direction::Up,
                    (0, 1) => Direction::Down,
                    _ => unreachable!(),
                };
                match grid.tile_at(&next_pos).unwrap() {
                    Forest => None,
                    Slope(slope) if slope != &dir => None,
                    Path => Some(self.moved_to(next_pos)),
                    Slope(slope_direction) => {
                        let next = self.moved_to(next_pos);
                        let after_next_pos = next_pos.moved_into(slope_direction);
                        if grid.tile_at(&after_next_pos).is_none() {
                            unreachable!("slopes should lead to valid tiles {after_next_pos}");
                        }
                        Some(next.moved_to(after_next_pos))
                    }
                }
            })
            .collect()
    }
    fn next_travelers2(&self, positions: Vec<Position>, grid: &Grid) -> Vec<Traveler> {
        positions
            .into_iter()
            .filter_map(|next_pos| match grid.tile_at(&next_pos) {
                None | Some(Forest) => None,
                Some(Path) | Some(Slope(_)) => Some(self.moved_to(next_pos)),
            })
            .collect()
    }
    fn moved_to(&self, pos: Position) -> Traveler {
        let mut next = self.clone();
        next.prev_pos = Some(next.pos);
        next.pos = pos;
        next.distance += 1;
        next
    }
}

impl Position {
    fn new(x: Coord, y: Coord) -> Self {
        Position { x, y }
    }
    fn moved_into(&self, dir: &Direction) -> Self {
        match dir {
            // Note: Using 0u8.wrapping_sub(1) returns u8::MAX, which is just as outside
            // the grid as -1 would be (unless the grid size were exactly u8::MAX).
            // -1 would also have been outside the range, but would panic with a u8,
            // and our grid of 141 is slightly too large for an i8.
            Direction::Up => Position::new(self.x, self.y.wrapping_sub(1)),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x.wrapping_sub(1), self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<_> = input
            .trim()
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
            .collect();
        let height = grid.len() as Coord;
        let width = grid[0].len() as Coord;
        Grid {
            grid,
            height,
            width,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Forest,
            '.' => Path,
            '^' => Slope(Direction::Up),
            '>' => Slope(Direction::Right),
            'v' => Slope(Direction::Down),
            '<' => Slope(Direction::Left),
            _ => unreachable!("Illegal value {value}"),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(Tile::to_string).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for Traveler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.distance, self.pos)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Path => ".".to_string(),
                Forest => "#".to_string(),
                Slope(dir) => dir.to_string(),
            }
        )
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "^",
                Direction::Down => "v",
                Direction::Right => ">",
                Direction::Left => "<",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn test_part1_example() {
        assert_eq!(94, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1_example_parse_and_to_string() {
        let grid = Grid::from(EXAMPLE);
        assert_eq!(EXAMPLE.trim(), format!("{grid}"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(2_250, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(154, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6_470, solve_part2(INPUT));
    }
}
