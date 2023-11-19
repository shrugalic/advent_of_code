use md5::Digest;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const PUZZLE_INPUT: &str = "hhhxzeay";

pub(crate) fn day17_part1() -> String {
    shortest_path(PUZZLE_INPUT)
}

pub(crate) fn day17_part2() -> usize {
    longest_path_len(PUZZLE_INPUT)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn is_possible_at(&self, pos: &Pos) -> bool {
        match self {
            Direction::Up => pos.y > 0,
            Direction::Down => pos.y < 3,
            Direction::Left => pos.x > 0,
            Direction::Right => pos.x < 3,
        }
    }
    fn with_open_doors(x: &[char]) -> Vec<Direction> {
        let mut dirs = vec![];
        if x[0].is_open() {
            dirs.push(Direction::Up);
        }
        if x[1].is_open() {
            dirs.push(Direction::Down);
        }
        if x[2].is_open() {
            dirs.push(Direction::Left);
        }
        if x[3].is_open() {
            dirs.push(Direction::Right);
        }
        dirs
    }
}
impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        }
        .to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    fn is_at_target(&self) -> bool {
        self.x == 3 && self.y == 3
    }
    fn move_into(&self, dir: &Direction) -> Pos {
        let (x, y) = (self.x, self.y);
        match dir {
            Direction::Up => Pos { x, y: y - 1 },
            Direction::Down => Pos { x, y: y + 1 },
            Direction::Left => Pos { x: x - 1, y },
            Direction::Right => Pos { x: x + 1, y },
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    pos: Pos,
    path: Vec<Direction>,
}
impl State {
    fn reachable_neighbors(&self, pass_code: &str) -> Vec<State> {
        let mut states = vec![];
        let header = md5(pass_code, &self.to_string()).to_char_vec();

        for dir in Direction::with_open_doors(&header)
            .into_iter()
            .filter(|dir| dir.is_possible_at(&self.pos))
        {
            let mut next = self.clone();
            next.pos = self.pos.move_into(&dir);
            next.path.push(dir);
            states.push(next);
        }
        states
    }
}
impl ToString for State {
    fn to_string(&self) -> String {
        self.path.iter().map(Direction::to_string).collect()
    }
}
impl Default for State {
    fn default() -> Self {
        State {
            pos: Pos::default(),
            path: Vec::default(),
        }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.len().cmp(&other.path.len()).reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(pass_code: &str) -> String {
    let mut candidates = BinaryHeap::new();
    candidates.push(State::default());
    while let Some(state) = candidates.pop() {
        if state.pos.is_at_target() {
            return state.to_string();
        } else {
            candidates.extend(state.reachable_neighbors(pass_code));
        }
    }
    unreachable!()
}

fn longest_path_len(pass_code: &str) -> usize {
    let mut candidates = vec![State::default()];
    let mut max_len = 0;
    while let Some(state) = candidates.pop() {
        if state.pos.is_at_target() {
            max_len = max_len.max(state.path.len());
        } else {
            candidates.extend(state.reachable_neighbors(pass_code));
        }
    }
    max_len
}

fn md5(pass_code: &str, path: &str) -> Digest {
    md5::compute(format!("{}{}", pass_code, path))
}

trait ToCharVec {
    fn to_char_vec(&self) -> Vec<char>;
}
impl ToCharVec for Digest {
    fn to_char_vec(&self) -> Vec<char> {
        format!("{:x}", self).chars().take(4).collect()
    }
}

trait IsOpen {
    fn is_open(&self) -> bool;
}
impl IsOpen for char {
    fn is_open(&self) -> bool {
        ('b'..='f').contains(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_open() {
        assert!(!'0'.is_open());
        assert!(!'1'.is_open());
        assert!(!'2'.is_open());
        assert!(!'3'.is_open());
        assert!(!'4'.is_open());
        assert!(!'5'.is_open());
        assert!(!'6'.is_open());
        assert!(!'7'.is_open());
        assert!(!'8'.is_open());
        assert!(!'9'.is_open());
        assert!(!'a'.is_open());
        assert!('b'.is_open());
        assert!('c'.is_open());
        assert!('d'.is_open());
        assert!('e'.is_open());
        assert!('f'.is_open());
        assert!(!'g'.is_open());
    }

    #[test]
    fn part1_example1() {
        assert_eq!("DDRRRD", shortest_path("ihgpwlah"));
    }

    #[test]
    fn part1_example2() {
        assert_eq!("DDUDRLRRUDRD", shortest_path("kglvqrro"));
    }

    #[test]
    fn part1_example3() {
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", shortest_path("ulqzkmiv"));
    }

    #[test]
    fn part1() {
        assert_eq!("DDRUDLRRRD", day17_part1());
    }

    #[test]
    fn part2_example1() {
        assert_eq!(370, longest_path_len("ihgpwlah"));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(492, longest_path_len("kglvqrro"));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(830, longest_path_len("ulqzkmiv"));
    }

    #[test]
    fn part2() {
        assert_eq!(398, day17_part2());
    }
}
