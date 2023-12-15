use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use Direction::*;
use Tile::*;

const INPUT: &str = include_str!("../input/day10.txt");

pub(crate) fn part1() -> usize {
    count_steps_to_farthest_point(INPUT)
}

pub(crate) fn part2() -> usize {
    count_regions_unreachable_from_outside(INPUT)
}

fn count_steps_to_farthest_point(input: &str) -> usize {
    let grid = Grid::from(input);
    let start = grid.get_starting_position();
    let directions_at_start = grid.get_directions_possible_at(&start);

    let mut direction = directions_at_start[0]; // either of the two directions is fine
    let mut curr_pos = start;
    let mut step_count = 0;
    loop {
        curr_pos = curr_pos.step_in(&direction);
        step_count += 1;
        if curr_pos == start {
            return step_count / 2;
        }
        let next_tile = grid.tile_at(&curr_pos).expect("a tile at this position");
        direction = next_tile.exit_direction_with_entry(&direction);
    }
}

fn count_regions_unreachable_from_outside(input: &str) -> usize {
    let mut grid = Grid::from(input);
    // println!("Original:\n{grid}\n");

    let start = grid.get_starting_position();
    let directions_at_start = grid.get_directions_possible_at(&start);
    grid.replace_start_with_actual_tile(&start, &directions_at_start);
    // println!("Start replaced with actual tile:\n{grid}\n");

    let direction = directions_at_start[0]; // either of the two directions is fine
    grid.replace_unconnected_pipes_with_ground(start, direction);
    // println!("Removed unconnected pipes:\n{grid}\n");

    grid.mark_obvious_outside_area_starting_from_borders();
    // println!("Removed outside areas:\n{grid}\n");

    grid.mark_areas_on_the_outside_of_the_loop_as_outside();
    // println!("inside filled:\n{grid}\n");

    grid.grid
        .iter()
        .map(|line| line.iter().filter(|tile| tile == &&Ground).count())
        .sum()
}

impl Pos {
    fn new(x: Coord, y: Coord) -> Self {
        Pos { x, y }
    }
    fn step_in(&self, dir: &Direction) -> Pos {
        let (dx, dy) = dir.offset();
        self.offset_by(dx, dy)
    }
    fn offset_by(&self, dx: Coord, dy: Coord) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
    fn left_of_path_in_regards_to(&self, direction: &Direction) -> Pos {
        match direction {
            N => self.step_in(&W),
            E => self.step_in(&N),
            S => self.step_in(&E),
            W => self.step_in(&S),
        }
    }
}

impl Direction {
    fn offset(&self) -> (i16, i16) {
        match self {
            N => (0, -1),
            S => (0, 1),
            E => (1, 0),
            W => (-1, 0),
        }
    }
}

impl Tile {
    fn allows_entry_in(&self, dir: &Direction) -> bool {
        match self {
            Start => true,
            Ground => false,
            pipe => pipe.redirections().contains_key(dir),
        }
    }
    fn exit_direction_with_entry(&self, dir: &Direction) -> Direction {
        *self
            .redirections()
            .get(dir)
            .expect("Tile does not allow entry in this direction")
    }
    fn to_char(&self) -> char {
        match self {
            NS => '|',
            EW => '-',
            NE => 'L',
            NW => 'J',
            SW => '7',
            SE => 'F',
            Ground => '.',
            Start => 'S',
            Outside => ' ',
        }
    }
    fn redirections(&self) -> HashMap<Direction, Direction> {
        match self {
            NS => [(N, N), (S, S)], // |
            EW => [(E, E), (W, W)], // -
            NE => [(S, E), (W, N)], // L
            NW => [(S, W), (E, N)], // J
            SW => [(E, S), (N, W)], // 7
            SE => [(N, E), (W, S)], // F
            Ground | Start | Outside => panic!("Unsupported tile {:?}", self),
        }
        .into_iter()
        .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq)]
enum Tile {
    NS,     // |
    EW,     // -
    NE,     // L
    NW,     // J
    SW,     // 7
    SE,     // F
    Ground, // .
    Start,  // S
    Outside,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    considered: HashSet<Pos>,
}

type Coord = i16;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Pos {
    x: Coord,
    y: Coord,
}

impl Grid {
    fn get_starting_position(&self) -> Pos {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(y, line)| {
                line.iter()
                    .position(|tile| tile == &Start)
                    .map(|x| Pos::new(x as Coord, y as Coord))
            })
            .next()
            .expect("A line with a start tile")
    }
    fn get_directions_possible_at(&self, start: &Pos) -> [Direction; 2] {
        let starting_directions: Vec<Direction> = [N, E, S, W]
            .into_iter()
            .filter(|direction| {
                let next_pos = start.step_in(direction);
                self.tile_at(&next_pos)
                    .is_some_and(|next_tile| next_tile.allows_entry_in(direction))
            })
            .collect();
        [starting_directions[0], starting_directions[1]]
    }
    fn replace_start_with_actual_tile(
        &mut self,
        start: &Pos,
        directions_at_start: &[Direction; 2],
    ) {
        let actual_tile = match (&directions_at_start[0], &directions_at_start[1]) {
            (N, S) | (S, N) => NS,
            (E, W) | (W, E) => EW,
            (N, E) | (E, N) => NE,
            (N, W) | (W, N) => NW,
            (S, W) | (W, S) => SW,
            (S, E) | (E, S) => SE,
            (l, r) => panic!("No connection for {l:?} and {r:?}"),
        };
        self.replace_tile(start, actual_tile);
    }
    fn replace_unconnected_pipes_with_ground(&mut self, start: Pos, mut direction: Direction) {
        let mut pipe_positions = HashSet::new();
        pipe_positions.insert(start);
        let mut curr_pos = start;
        loop {
            curr_pos = curr_pos.step_in(&direction);
            pipe_positions.insert(curr_pos);
            if curr_pos == start {
                break;
            }
            let next_tile = self.tile_at(&curr_pos).expect("valid tile");
            direction = next_tile.exit_direction_with_entry(&direction);
        }
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos::new(x as Coord, y as Coord);
                if !pipe_positions.contains(&pos) {
                    self.replace_tile(&pos, Ground);
                }
            }
        }
    }
    fn mark_obvious_outside_area_starting_from_borders(&mut self) {
        for x in 0..self.width() as Coord {
            self.fill_ground_with_outside_marking(x, 0);
            self.fill_ground_with_outside_marking(x, (self.height() - 1) as Coord);
        }
        for y in 1..self.height() - 1 {
            self.fill_ground_with_outside_marking(0, y as Coord);
            self.fill_ground_with_outside_marking((self.width() - 1) as Coord, y as Coord);
        }
    }
    fn mark_areas_on_the_outside_of_the_loop_as_outside(&mut self) {
        // Find a new starting position next to a known "outside" area
        let mut start = Pos::new(0, (self.height() / 2) as Coord);
        while let Some(Outside) = self.tile_at(&start) {
            start.x += 1;
        }

        // Choose the direction such that the known "outside" area is on the left
        let mut direction = match self.tile_at(&start).unwrap() {
            NS => N,
            NE => N,
            SE => E,
            tile => unreachable!("Tile {:?}", tile),
        };

        // Follow path and mark stuff on the left as outside
        let mut curr_pos = start.step_in(&direction);
        while curr_pos != start {
            let left_of_entrance = curr_pos.left_of_path_in_regards_to(&direction);
            self.fill_ground_with_outside_marking(left_of_entrance.x, left_of_entrance.y);

            let tile = self.tile_at(&curr_pos).expect("valid_tile");
            direction = tile.exit_direction_with_entry(&direction);

            let left_of_exit = curr_pos.left_of_path_in_regards_to(&direction);
            self.fill_ground_with_outside_marking(left_of_exit.x, left_of_exit.y);

            curr_pos = curr_pos.step_in(&direction);
        }
    }
    fn fill_ground_with_outside_marking(&mut self, x: Coord, y: Coord) {
        let mut positions_to_consider = vec![Pos::new(x, y)];
        while let Some(pos) = positions_to_consider.pop() {
            self.considered.insert(pos);
            if let Some(Ground) = self.tile_at(&pos) {
                self.replace_tile(&pos, Outside);
                for neighbor in [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .map(|(dx, dy)| pos.offset_by(dx, dy))
                    .filter(|pos| matches!(self.tile_at(pos), Some(Ground)))
                {
                    if !self.considered.contains(&neighbor) {
                        positions_to_consider.push(neighbor);
                    }
                }
            }
        }
    }
    fn tile_at(&self, pos: &Pos) -> Option<&Tile> {
        self.grid
            .get(pos.y as usize)
            .and_then(|line| line.get(pos.x as usize))
    }
    fn replace_tile(&mut self, pos: &Pos, new_tile: Tile) {
        *self
            .grid
            .get_mut(pos.y as usize)
            .unwrap()
            .get_mut(pos.x as usize)
            .unwrap() = new_tile;
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Grid {
            grid,
            considered: HashSet::new(),
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
                .map(|line| line.iter().map(Tile::to_char).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => Ground,
            'S' => Start,
            _ => panic!("Unknown tile '{c}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1A: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....
";

    const EXAMPLE_1B: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const EXAMPLE_1C: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    const EXAMPLE_1D: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const EXAMPLE_2A: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const EXAMPLE_2B: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

    const EXAMPLE_2C: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE_2D: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_part1_examples() {
        assert_eq!(4, count_steps_to_farthest_point(EXAMPLE_1A));
        assert_eq!(4, count_steps_to_farthest_point(EXAMPLE_1B));
        assert_eq!(8, count_steps_to_farthest_point(EXAMPLE_1C));
        assert_eq!(8, count_steps_to_farthest_point(EXAMPLE_1D));
    }

    #[test]
    fn test_part1() {
        assert_eq!(7_063, count_steps_to_farthest_point(INPUT));
    }

    #[test]
    fn test_part2_open_example() {
        assert_eq!(4, count_regions_unreachable_from_outside(EXAMPLE_2A));
    }
    #[test]
    fn test_part2_closed_examples() {
        assert_eq!(4, count_regions_unreachable_from_outside(EXAMPLE_2B));
        assert_eq!(8, count_regions_unreachable_from_outside(EXAMPLE_2C));
        assert_eq!(10, count_regions_unreachable_from_outside(EXAMPLE_2D));
    }

    #[test]
    fn test_part2() {
        assert_eq!(589, count_regions_unreachable_from_outside(INPUT));
    }
}
