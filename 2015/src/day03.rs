use line_reader::read_file_to_lines;
use std::collections::HashSet;

pub(crate) fn day03_part1() -> usize {
    presents_delivered_by_santa(&read_file_to_lines("input/day03.txt")[0])
}

pub(crate) fn day03_part2() -> usize {
    presents_delivered_by_santa_and_robo_santa(&read_file_to_lines("input/day03.txt")[0])
}

fn presents_delivered_by_santa(path: &str) -> usize {
    presents_delivered(path, 1)
}

fn presents_delivered_by_santa_and_robo_santa(path: &str) -> usize {
    presents_delivered(path, 2)
}

fn presents_delivered(path: &str, courier_count: usize) -> usize {
    let path: Vec<_> = path.chars().map(Dir::from).collect();
    let mut pos = vec![Pos::default(); courier_count];
    let mut presents_delivered = HashSet::new();
    let mut courier_idx = 0;
    presents_delivered.insert(pos[courier_idx]);
    for dir in path {
        pos[courier_idx].go(&dir);
        presents_delivered.insert(pos[courier_idx]);
        courier_idx = (courier_idx + 1) % courier_count;
    }
    presents_delivered.len()
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}
impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}

impl Pos {
    fn go(&mut self, dir: &Dir) {
        match dir {
            Dir::N => self.y -= 1,
            Dir::S => self.y += 1,
            Dir::E => self.x += 1,
            Dir::W => self.x -= 1,
        }
    }
}
enum Dir {
    N,
    S,
    E,
    W,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '^' => Dir::N,
            'v' => Dir::S,
            '>' => Dir::E,
            '<' => Dir::W,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(2, presents_delivered_by_santa(">"));
        assert_eq!(4, presents_delivered_by_santa("^>v<"));
        assert_eq!(2, presents_delivered_by_santa("^v^v^v^v^v"));
    }

    #[test]
    fn part1() {
        assert_eq!(2565, day03_part1());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(3, presents_delivered_by_santa_and_robo_santa("^v"));
        assert_eq!(3, presents_delivered_by_santa_and_robo_santa("^>v<"));
        assert_eq!(11, presents_delivered_by_santa_and_robo_santa("^v^v^v^v^v"));
    }

    #[test]
    fn part2() {
        assert_eq!(2639, day03_part2());
    }
}
