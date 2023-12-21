use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter;
use std::ops::Add;

use GridType::*;

const INPUT: &str = include_str!("../input/day21.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT, 64)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT, 26_501_365)
}

fn solve_part1(input: &'static str, max_steps: usize) -> usize {
    let rock_map = RockMap::from(input);
    let mut grid = Grid::single_grid(&rock_map);
    for step in 1..=max_steps {
        grid.step(step);
    }
    grid.visit_count(max_steps)
}

#[derive(Debug, Default)]
struct Stats {
    start: usize,
    initial_frontier: HashSet<Position>,
    end: usize,
    visit_count: [usize; 2],
}

fn solve_part2(input: &'static str, max_steps: usize) -> usize {
    let rock_map = RockMap::from(input);
    let frontier = HashSet::from_iter(iter::once(rock_map.start));

    let mut stats_by_pos: HashMap<Position, Stats> = HashMap::new();
    let origin = Position::default();
    let stats = Stats {
        initial_frontier: frontier.clone(),
        ..Default::default()
    };
    stats_by_pos.insert(origin, stats);

    let mut grids_by_pos: HashMap<Position, Grid> = HashMap::new();
    let mut grid = Grid::infinite_grid(&rock_map);
    grid.frontier[0] = frontier;
    grids_by_pos.insert(origin, grid);

    for step in 1..=max_steps {
        // println!("-------------- step {step} --------------");
        let mut all_transfers: HashMap<Position, HashSet<Position>> = HashMap::new();

        // Step all grids
        for (offset, grid) in grids_by_pos.iter_mut() {
            if !grid.is_done() {
                let transfers: HashMap<Position, HashSet<Position>> = grid.step(step);
                if grid.is_done() {
                    let stats = stats_by_pos.get_mut(offset).unwrap();
                    stats.end = step;
                    stats.visit_count[0] = grid.visit_count(0);
                    stats.visit_count[1] = grid.visit_count(1);
                }

                for (transfer_offset, actives) in transfers {
                    let offset = *offset + transfer_offset;
                    all_transfers.entry(offset).or_default().extend(actives);
                }
            }
        }

        // Transfer actives to neighboring grids
        for (offset, frontier) in all_transfers {
            // println!(
            //     "{step}: {} possible transfers to {offset}: {frontier:?}",
            //     frontier.len()
            // );
            if !grids_by_pos.contains_key(&offset) {
                // println!(
                //     "{step}: New grid at @ offset {offset} with frontier len {}",
                //     frontier.len()
                // );
                let stats = Stats {
                    start: step,
                    initial_frontier: frontier.clone(),
                    ..Default::default()
                };
                stats_by_pos.insert(offset, stats);
            }
            let neighbor_grid = grids_by_pos
                .entry(offset)
                .or_insert(Grid::infinite_grid(&rock_map));

            let index = step % 2;
            for pos in &frontier {
                if neighbor_grid.frontier[index].contains(pos) {
                    // println!("- not inserting frontier {pos}");
                    continue;
                }
                if neighbor_grid.visited[index].contains(pos) {
                    // println!("- not inserting visited {pos}");
                    continue;
                }
                if neighbor_grid.is_done() {
                    neighbor_grid.print_details(step, &HashSet::new());
                    println!("{}", Grid::format_positions(&frontier));
                    panic!("want to enter {pos} into frontiers!");
                }
                neighbor_grid.frontier[index].insert(*pos);
                // println!("+ inserted {pos}");
            }
        }
    }

    let mut total_visit_count = 0;
    println!("------------- summing up {max_steps} ----------------");
    for (offset, stats) in stats_by_pos {
        if stats.end == 0 {
            println!(
                "{offset} unfinished: counts {} and {}, frontier {}",
                stats.visit_count[0],
                stats.visit_count[1],
                Grid::format_positions(&stats.initial_frontier)
            );
        } else {
            println!(
                "{offset} finished in {} with counts {} and {}, frontier {}",
                stats.end as isize - stats.start as isize,
                stats.visit_count[0],
                stats.visit_count[1],
                Grid::format_positions(&stats.initial_frontier)
            );
        }
    }
    for (_offset, grid) in grids_by_pos {
        // println!("- grid at {_offset}: {grid}: actives {:?}", grid.visited);
        total_visit_count += grid.visit_count(max_steps);
    }
    total_visit_count
}

#[derive(PartialEq, Debug)]
enum GridType {
    Single,
    Infinite,
}

type Coord = i16;
#[derive(Debug)]
struct RockMap {
    rocks: Vec<Vec<bool>>, // `true` if there's a rock at index [y][x]
    start: Position,
    boundary: Position,
}

#[derive(Debug)]
struct Grid<'a> {
    rock_map: &'a RockMap,
    grid_type: GridType,
    visited: [HashSet<Position>; 2],  // 0 for even, 1 for odd
    frontier: [HashSet<Position>; 2], // 0 for even, 1 for odd
}

impl<'a> Grid<'a> {
    fn single_grid(rock_map: &'a RockMap) -> Self {
        Grid {
            rock_map,
            grid_type: Single,
            visited: [HashSet::new(), HashSet::new()],
            frontier: [
                HashSet::from_iter(iter::once(rock_map.start)), // even
                HashSet::new(),                                 // odd
            ],
        }
    }
    fn infinite_grid(rock_map: &'a RockMap) -> Self {
        Grid {
            rock_map,
            grid_type: Infinite,
            visited: [HashSet::new(), HashSet::new()],
            frontier: [HashSet::new(), HashSet::new()],
        }
    }
    fn step(&mut self, step: usize) -> HashMap<Position, HashSet<Position>> {
        let previous = step % 2;
        let current = (step + 1) % 2;

        let mut transfers: HashMap<Position, HashSet<Position>> = HashMap::new();
        let new_frontier: HashSet<Position> = self.frontier[current]
            .iter()
            .flat_map(|current| current.neighbors(self.rock_map))
            .filter(|next| !self.frontier[previous].contains(next))
            .filter(|next| {
                if next.is_within(&self.rock_map.boundary) {
                    true
                } else if self.grid_type == Single {
                    false
                } else {
                    // Transfer to neighboring grid
                    let offset = next.grid_offset(&self.rock_map.boundary);
                    transfers
                        .entry(offset)
                        .or_default()
                        .insert(next.normalized_to(&self.rock_map.boundary));
                    false
                    // true
                }
            })
            .collect();
        // self.print_details(step, &new_frontier);

        // if new_frontier.is_empty() && self.frontier[previous].is_empty() {
        //     self.print_details(step, &new_frontier);
        //     self.is_in_steady_state = true;
        //     // panic!("this happens?");
        //     // println!("entered steady state:\n{self}\n");
        //     return transfers;
        // }
        self.visited[previous].extend(self.frontier[previous].drain());
        self.frontier[previous] = new_frontier;

        transfers
    }
    fn is_done(&self) -> bool {
        self.frontier[0].is_empty()
            && self.frontier[1].is_empty()
            && !self.visited[0].is_empty()
            && !self.visited[1].is_empty()
    }
    fn visit_count(&self, step_count: usize) -> usize {
        self.visited[step_count % 2].len() + self.frontier[step_count % 2].len()
    }
    #[allow(dead_code)]
    fn print_details(&self, step: usize, new_frontier: &HashSet<Position>) {
        let previous = step % 2;
        let current = (step + 1) % 2;
        println!(
            "- new frontier {:?} \n- current frontier {} \n- previous frontier {}\n- current visited {} \n- previous visited {}\n",
            Grid::format_positions(new_frontier),
            Grid::format_positions(&self.frontier[current]),
            Grid::format_positions(&self.frontier[previous]),
            Grid::format_positions(&self.visited[current]),
            Grid::format_positions(&self.visited[previous])
        );
    }
    #[allow(dead_code)]
    fn format_positions(set: &HashSet<Position>) -> String {
        let mut vec: Vec<_> = set.iter().collect();
        vec.sort_unstable();
        format!(
            "{{{}}}",
            vec.iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Position {
    x: Coord,
    y: Coord,
}

impl RockMap {
    fn rock_at_pos(&self, pos: &Position) -> bool {
        let normalized = pos.normalized_to(&self.boundary);
        self.rocks[normalized.y as usize][normalized.x as usize]
    }
}

impl Position {
    fn new(x: Coord, y: Coord) -> Self {
        Position { x, y }
    }
    fn offset(value: Coord, limit: Coord) -> Coord {
        if value < 0 {
            (value - limit) / limit
        } else {
            value / limit
        }
    }
    fn grid_offset(self, boundary: &Position) -> Position {
        Position::new(
            Position::offset(self.x, boundary.x),
            Position::offset(self.y, boundary.y),
        )
    }
    fn is_within(&self, boundary: &Position) -> bool {
        (0..boundary.x).contains(&self.x) && (0..boundary.y).contains(&self.y)
    }
    fn neighbors(self, grid: &RockMap) -> impl Iterator<Item = Position> + '_ {
        [
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
        ]
        .into_iter()
        .filter(|pos| !grid.rock_at_pos(pos))
    }
    fn normalized_to(&self, boundary: &Position) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        x %= boundary.x;
        y %= boundary.y;
        if x < 0 {
            x += boundary.x;
        }
        if y < 0 {
            y += boundary.y;
        }
        Position::new(x % boundary.x, y % boundary.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl From<&str> for RockMap {
    fn from(input: &str) -> Self {
        let mut start = Position::new(0, 0);
        let grid: Vec<_> = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .inspect(|(x, c)| {
                        if c == &'S' {
                            start = Position::new(*x as Coord, y as Coord);
                        }
                    })
                    .map(|(_x, c)| c == '#')
                    .collect::<Vec<_>>()
            })
            .collect();
        let boundary = Position::new(grid[0].len() as Coord, grid.len() as Coord);
        RockMap {
            rocks: grid,
            start,
            boundary,
        }
    }
}

impl Display for Grid<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.rock_map.boundary.y)
                .map(|y| (0..self.rock_map.boundary.x)
                    .map(|x| Position::new(x as Coord, y as Coord))
                    .map(|pos| if self.rock_map.rock_at_pos(&pos) {
                        '#'
                    } else if self.frontier[0].contains(&pos) {
                        'c'
                    } else if self.frontier[1].contains(&pos) {
                        'o'
                    } else if self.visited[0].contains(&pos) {
                        'C'
                    } else if self.visited[1].contains(&pos) {
                        'O'
                    } else {
                        '.'
                    })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for RockMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rocks
                .iter()
                .enumerate()
                .map(|(y, row)| row
                    .iter()
                    .enumerate()
                    .map(
                        |(x, b)| if self.start == Position::new(x as Coord, y as Coord) {
                            'S'
                        } else if *b {
                            '#'
                        } else {
                            '.'
                        }
                    )
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn test_part1_example() {
        assert_eq!(2, solve_part1(EXAMPLE, 1));
        assert_eq!(4, solve_part1(EXAMPLE, 2));
        assert_eq!(6, solve_part1(EXAMPLE, 3));
        assert_eq!(9, solve_part1(EXAMPLE, 4));
        assert_eq!(13, solve_part1(EXAMPLE, 5));
        assert_eq!(16, solve_part1(EXAMPLE, 6));
    }

    #[test]
    fn test_part1_example_parse_and_to_string() {
        let rock_map = RockMap::from(EXAMPLE);
        assert_eq!(EXAMPLE.trim(), format!("{rock_map}"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(3671, solve_part1(INPUT, 64));
    }

    #[test]
    fn test_offset() {
        let max = 10;
        assert_eq!(0, Position::offset(0, max));
        assert_eq!(0, Position::offset(5, max));
        assert_eq!(0, Position::offset(9, max));

        assert_eq!(1, Position::offset(10, max));
        assert_eq!(1, Position::offset(15, max));
        assert_eq!(1, Position::offset(19, max));

        assert_eq!(2, Position::offset(20, max));
        assert_eq!(2, Position::offset(25, max));

        assert_eq!(-1, Position::offset(-1, max));
        assert_eq!(-1, Position::offset(-5, max));
        assert_eq!(-1, Position::offset(-9, max));

        assert_eq!(-2, Position::offset(-10, max));
        assert_eq!(-2, Position::offset(-15, max));
    }

    #[test]
    fn test_normalize() {
        let boundary = Position::new(10, 10);
        let normalized = |pos: Position| pos.normalized_to(&boundary);

        assert_eq!(Position::new(0, 0), normalized(Position::new(0, 0)));
        assert_eq!(Position::new(9, 9), normalized(Position::new(9, 9)));

        assert_eq!(Position::new(0, 0), normalized(Position::new(10, 10)));
        assert_eq!(Position::new(9, 9), normalized(Position::new(19, 19)));

        assert_eq!(Position::new(0, 0), normalized(Position::new(-10, -10)));
        assert_eq!(Position::new(9, 9), normalized(Position::new(-1, -1)));
    }

    #[test]
    fn test_part2_example() {
        // my own
        assert_eq!(22, solve_part2(EXAMPLE, 7)); // 8A 13C 1T
        assert_eq!(30, solve_part2(EXAMPLE, 8)); // 10F 7P 9O 4T
        assert_eq!(41, solve_part2(EXAMPLE, 9)); // 18F 9P 13C 1T
        assert_eq!(63, solve_part2(EXAMPLE, 11)); // 21F, 19P 22C

        // AOC
        assert_eq!(16, solve_part2(EXAMPLE, 6));
        assert_eq!(50, solve_part2(EXAMPLE, 10));
        assert_eq!(1594, solve_part2(EXAMPLE, 50));
        assert_eq!(6536, solve_part2(EXAMPLE, 100));
        assert_eq!(167_004, solve_part2(EXAMPLE, 500));
        assert_eq!(668_697, solve_part2(EXAMPLE, 1000));
        assert_eq!(16_733_044, solve_part2(EXAMPLE, 5000));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(1, solve_part2(INPUT, 26_501_365));
    }
}
