use crate::parse;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day22.txt");

const PART1_BURSTS: usize = 10_000;
const PART2_BURSTS: usize = 10_000_000;
pub(crate) fn day22_part1() -> usize {
    infections_after_bursts_part1(parse(INPUT), PART1_BURSTS)
}

pub(crate) fn day22_part2() -> usize {
    infections_after_bursts_part2(parse(INPUT), PART2_BURSTS)
}

fn infections_after_bursts_part1(input: Vec<&str>, burst_count: usize) -> usize {
    infections_after_bursts(input, burst_count, Part::One)
}

fn infections_after_bursts_part2(input: Vec<&str>, burst_count: usize) -> usize {
    infections_after_bursts(input, burst_count, Part::Two)
}

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

fn infections_after_bursts(input: Vec<&str>, burst_count: usize, part: Part) -> usize {
    let mut grid = parse_input(&input);
    let mut curr_pos = Loc::new((input[0].len() as isize) / 2, (input.len() / 2) as isize);
    let mut infections_caused = 0;
    let mut dir = Dir::Up;
    for _ in 0..burst_count {
        let is_infected = grid.entry(curr_pos).or_insert(NodeState::Clean);
        match is_infected {
            NodeState::Clean => {
                if part == Part::One {
                    infections_caused += 1;
                }
                dir.turn_left();
            }
            NodeState::Weakened => {
                if part == Part::Two {
                    infections_caused += 1;
                }
            }
            NodeState::Infected => {
                dir.turn_right();
            }
            NodeState::Flagged => {
                dir.turn_around();
            }
        }
        match part {
            Part::One => is_infected.toggle(),
            Part::Two => is_infected.flag(),
        }
        curr_pos.step(&dir);
    }
    infections_caused
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Loc {
    x: isize,
    y: isize,
}
impl Loc {
    fn new(x: isize, y: isize) -> Self {
        Loc { x, y }
    }
    fn step(&mut self, dir: &Dir) {
        match dir {
            Dir::Up => self.y -= 1,
            Dir::Right => self.x += 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
        }
    }
}

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_left(&mut self) {
        *self = match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        };
    }
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }
    fn turn_around(&mut self) {
        *self = match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        };
    }
}

enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}
impl From<char> for NodeState {
    fn from(c: char) -> Self {
        if c == '#' {
            NodeState::Infected
        } else {
            NodeState::Clean
        }
    }
}
impl NodeState {
    fn toggle(&mut self) {
        *self = match self {
            NodeState::Clean => NodeState::Infected,
            NodeState::Infected => NodeState::Clean,
            _ => unreachable!(),
        }
    }
    fn flag(&mut self) {
        *self = match self {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        }
    }
}

fn parse_input(input: &[&str]) -> HashMap<Loc, NodeState> {
    let mut grid: HashMap<Loc, NodeState> = HashMap::new();
    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert(Loc::new(x as isize, y as isize), NodeState::from(c));
        });
    });
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE_MAP: &str = "\
..#
#..
...";

    #[test]
    fn part1_example() {
        assert_eq!(
            5587,
            infections_after_bursts_part1(parse(EXAMPLE_MAP), PART1_BURSTS)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(5565, day22_part1());
    }

    #[test]
    fn part2_example_short() {
        assert_eq!(
            26,
            infections_after_bursts_part2(parse(EXAMPLE_MAP), 100)
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            2511944,
            infections_after_bursts_part2(parse(EXAMPLE_MAP), PART2_BURSTS)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(2511978, day22_part2());
    }
}
