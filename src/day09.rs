use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day09.txt");

pub(crate) fn day09_part1() -> usize {
    let commands = parse(INPUT);
    number_of_unique_positions_visited_by_tail(commands, 2)
}

pub(crate) fn day09_part2() -> usize {
    let commands = parse(INPUT);
    number_of_unique_positions_visited_by_tail(commands, 10)
}

fn parse(input: &str) -> Vec<Command> {
    input.trim().lines().map(Command::from).collect()
}

fn number_of_unique_positions_visited_by_tail(commands: Vec<Command>, rope_length: usize) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut rope = vec![Pos::default(); rope_length];
    visited.insert(*rope.last().unwrap());

    for Command { steps, direction } in commands {
        for _ in 0..steps {
            let head = &mut rope[0];
            head.move_in(&direction);

            for i in 1..rope_length {
                let head = rope[i - 1];
                let tail = &mut rope[i];
                if !head.is_neighbor_of(tail) {
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

trait MoveInDirection {
    fn move_in(&mut self, direction: &Direction);
}
impl MoveInDirection for Pos {
    fn move_in(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

trait IsNeighbor {
    fn is_neighbor_of(&self, other: &Self) -> bool;
}
impl IsNeighbor for Pos {
    fn is_neighbor_of(&self, other: &Self) -> bool {
        self.neighbors().contains(other)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, 1),
            (0, 0),
            (0, -1),
            (1, 1),
            (1, 0),
            (1, -1),
        ]
        .into_iter()
        .map(|(x, y)| Pos {
            x: self.x + x,
            y: self.y + y,
        })
        .collect()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!("Unknown direction {}", c),
        }
    }
}

struct Command {
    steps: u8,
    direction: Direction,
}
impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once(' ').expect("a space");
        let direction = Direction::from(left);
        let step_count = right.parse().expect("a valid u8");
        Command {
            steps: step_count,
            direction: direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LARGER_EXAMPLE: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_example() {
        let commands = parse(EXAMPLE);
        assert_eq!(13, number_of_unique_positions_visited_by_tail(commands, 2));
    }

    #[test]
    fn part1() {
        assert_eq!(6_311, day09_part1());
    }

    #[test]
    fn part2_example1() {
        let commands = parse(EXAMPLE);
        assert_eq!(1, number_of_unique_positions_visited_by_tail(commands, 10));
    }

    #[test]
    fn part2_example2() {
        let commands = parse(LARGER_EXAMPLE);
        assert_eq!(36, number_of_unique_positions_visited_by_tail(commands, 10));
    }

    #[test]
    fn part2() {
        assert_eq!(2_482, day09_part2());
    }
}
