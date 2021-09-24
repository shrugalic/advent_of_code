use line_reader::read_file_to_lines;
use std::collections::HashSet;

pub(crate) fn day01_part1() -> usize {
    distance_from_origin(&read_file_to_lines("input/day01.txt")[0])
}

pub(crate) fn day01_part2() -> usize {
    distance_to_first_location_visited_twice(&read_file_to_lines("input/day01.txt")[0])
}

fn distance_from_origin(input: &str) -> usize {
    let (mut x, mut y) = (0isize, 0isize);
    let mut dir = Dir::N;
    for Step { turn, distance } in input.split(", ").map(Step::from).collect::<Vec<_>>() {
        dir.turn(&turn);
        match dir {
            Dir::N => y -= distance,
            Dir::S => y += distance,
            Dir::E => x += distance,
            Dir::W => x -= distance,
        }
    }

    (x.abs() + y.abs()) as usize
}

fn distance_to_first_location_visited_twice(input: &str) -> usize {
    let (mut x, mut y) = (0isize, 0isize);
    let mut dir = Dir::N;
    let mut visited = HashSet::new();
    visited.insert((x, y));
    'outer: for Step { turn, distance } in input.split(", ").map(Step::from).collect::<Vec<_>>() {
        dir.turn(&turn);
        for _ in 0..distance {
            match dir {
                Dir::N => y -= 1,
                Dir::S => y += 1,
                Dir::E => x += 1,
                Dir::W => x -= 1,
            }
            if !visited.insert((x, y)) {
                break 'outer;
            }
        }
    }

    (x.abs() + y.abs()) as usize
}

enum Turn {
    L,
    R,
}

struct Step {
    turn: Turn,
    distance: isize,
}
impl From<&str> for Step {
    fn from(s: &str) -> Self {
        let distance = s[1..].parse().unwrap();
        let turn = if s.starts_with('R') { Turn::R } else { Turn::L };
        Step { turn, distance }
    }
}

enum Dir {
    N,
    E,
    S,
    W,
}
impl Dir {
    fn turn(&mut self, turn: &Turn) {
        *self = match turn {
            Turn::L => match self {
                Dir::N => Dir::W,
                Dir::E => Dir::N,
                Dir::S => Dir::E,
                Dir::W => Dir::S,
            },
            Turn::R => match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(5, distance_from_origin("R2, L3"));
        assert_eq!(2, distance_from_origin("R2, R2, R2"));
        assert_eq!(12, distance_from_origin("R5, L5, R5, R3"));
    }

    #[test]
    fn part1() {
        assert_eq!(230, day01_part1());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(
            4,
            distance_to_first_location_visited_twice("R8, R4, R4, R8")
        );
    }

    #[test]
    fn part2() {
        assert_eq!(154, day01_part2());
    }
}
