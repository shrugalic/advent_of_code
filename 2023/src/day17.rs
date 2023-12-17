use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use std::vec;

use Direction::*;

const INPUT: &str = include_str!("../input/day17.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    Grid::from(input).minimal_heat_loss_with(&StraightStepCount::min_1_max_3())
}

fn solve_part2(input: &str) -> usize {
    Grid::from(input).minimal_heat_loss_with(&StraightStepCount::min_4_max_10())
}

type Coord = u8;
type HeatLoss = u8;
struct Grid {
    heat_loss_grid: Vec<Vec<HeatLoss>>,
    height: Coord,
    width: Coord,
}

struct StraightStepCount {
    min: u8,
    max: u8,
}

#[derive(Default, Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Crucible {
    total_heat_loss: usize,
    straight_step_count: u8,
    pos: Position,
    dir: Direction,
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
struct Position {
    x: Coord,
    y: Coord,
}

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    #[default]
    Right,
    Left,
}

impl Grid {
    fn minimal_heat_loss_with(&self, constraint: &StraightStepCount) -> usize {
        let exit = Position::new(self.width - 1, self.height - 1);
        let mut min_heat_losses = HashMap::new();

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Crucible::new(Right));
        priority_queue.push(Crucible::new(Down));

        while let Some(crucible) = priority_queue.pop() {
            if crucible.pos == exit {
                return crucible.total_heat_loss;
            }

            for crucible in crucible.next_crucibles(self, constraint) {
                let min_heat_loss = min_heat_losses
                    .entry((crucible.pos, crucible.dir))
                    .or_insert(usize::MAX);
                if crucible.total_heat_loss < *min_heat_loss {
                    *min_heat_loss = crucible.total_heat_loss;
                    priority_queue.push(crucible);
                }
            }
        }
        unreachable!()
    }
    fn apply_heat_loss_to(&self, crucible: &mut Crucible) {
        crucible.total_heat_loss +=
            self.heat_loss_grid[crucible.pos.y as usize][crucible.pos.x as usize] as usize;
    }
}

impl Crucible {
    fn new(dir: Direction) -> Self {
        Crucible {
            dir,
            ..Default::default()
        }
    }
    fn next_crucibles(&self, grid: &Grid, constraint: &StraightStepCount) -> Vec<Crucible> {
        [Up, Down, Right, Left]
            .into_iter()
            .filter(|next| &self.dir != next && self.dir != next.opposite())
            .flat_map(|dir| self.next_crucibles_in_direction(dir, grid, constraint))
            .collect()
    }
    fn next_crucibles_in_direction(
        self,
        dir: Direction,
        grid: &Grid,
        constraint: &StraightStepCount,
    ) -> Vec<Crucible> {
        let mut next_crucible = self;
        next_crucible.dir = dir;
        let mut next_crucibles = vec![];
        for straight_step_count in 1..=constraint.max {
            next_crucible.pos = next_crucible.pos.moved_into(dir);
            if !next_crucible.pos.is_within(grid) {
                return next_crucibles;
            }
            grid.apply_heat_loss_to(&mut next_crucible);
            next_crucible.straight_step_count = straight_step_count;
            if straight_step_count >= constraint.min {
                next_crucibles.push(next_crucible);
            }
        }
        next_crucibles
    }
}

impl Position {
    fn new(x: Coord, y: Coord) -> Self {
        Position { x, y }
    }
    fn is_within(&self, grid: &Grid) -> bool {
        (0..grid.width).contains(&self.x) && (0..grid.height).contains(&self.y)
    }
    fn moved_into(&self, dir: Direction) -> Self {
        match dir {
            // Note: Using 0u8.wrapping_sub(1) returns u8::MAX, which is just as outside
            // the grid as -1 would be (unless the grid size were exactly u8::MAX).
            // -1 would also have been outside the range, but would panic with a u8,
            // and our grid of 141 is slightly too large for an i8.
            Up => Position::new(self.x, self.y.wrapping_sub(1)),
            Down => Position::new(self.x, self.y + 1),
            Left => Position::new(self.x.wrapping_sub(1), self.y),
            Right => Position::new(self.x + 1, self.y),
        }
    }
}

impl StraightStepCount {
    fn min_1_max_3() -> StraightStepCount {
        StraightStepCount { min: 1, max: 3 }
    }
    fn min_4_max_10() -> StraightStepCount {
        StraightStepCount { min: 4, max: 10 }
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_heat_loss.cmp(&self.total_heat_loss)
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid: Vec<_> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("valid digit") as HeatLoss)
                    .collect::<Vec<_>>()
            })
            .collect();
        let height = grid.len() as Coord;
        let width = grid[0].len() as Coord;
        Grid {
            heat_loss_grid: grid,
            height,
            width,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.heat_loss_grid
                .iter()
                .map(|row| row.iter().map(HeatLoss::to_string).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Display for Crucible {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ {}, {} total_heat_loss, {} curr_steps_in_same_direction",
            self.dir, self.pos, self.total_heat_loss, self.straight_step_count
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Up => '^',
                Down => 'v',
                Right => '>',
                Left => '<',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    const EXAMPLE_2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn test_part1_example() {
        assert_eq!(102, solve_part1(EXAMPLE_1));
    }

    #[test]
    fn test_part1_example_parse_and_to_string() {
        let grid = Grid::from(EXAMPLE_1);
        assert_eq!(EXAMPLE_1.trim(), format!("{grid}"));
    }

    #[test]
    fn test_part1_binary_heap_priority() {
        let lowest_heat_loss = Crucible::new(Right);

        let mut increased_heat_loss = lowest_heat_loss;
        increased_heat_loss.total_heat_loss += 1;

        let mut queue = BinaryHeap::new();
        queue.push(lowest_heat_loss);
        queue.push(increased_heat_loss);

        assert_eq!(Some(lowest_heat_loss), queue.pop());
        assert_eq!(Some(increased_heat_loss), queue.pop());
    }

    #[test]
    fn test_part1() {
        assert_eq!(1_001, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example_1() {
        assert_eq!(94, solve_part2(EXAMPLE_1));
    }
    #[test]
    fn test_part2_example_2() {
        assert_eq!(71, solve_part2(EXAMPLE_2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1_197, solve_part2(INPUT));
    }
}
