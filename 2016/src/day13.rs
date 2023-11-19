use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
const PUZZLE_INPUT: usize = 1350;

pub(crate) fn day13_part1() -> usize {
    shortest_path(Coord::default(), Coord { x: 31, y: 39 }, PUZZLE_INPUT)
}

pub(crate) fn day13_part2() -> usize {
    reachable_with_steps(Coord::default(), 50, PUZZLE_INPUT)
}

type MagicNumber = usize;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}
impl Default for Coord {
    fn default() -> Self {
        Coord { x: 1, y: 1 }
    }
}
impl Coord {
    fn is_open(&self, fav: MagicNumber) -> bool {
        is_open(self.x, self.y, fav)
    }
    fn offset_by(&self, x: isize, y: isize) -> Option<Coord> {
        let x = self.x as isize + x;
        let y = self.y as isize + y;
        if x >= 0 && y >= 0 {
            Some(Coord {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
    fn neighbors(&self) -> Vec<Coord> {
        vec![
            self.offset_by(1, 0),
            self.offset_by(0, 1),
            self.offset_by(-1, 0),
            self.offset_by(0, -1),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn is_open(x: usize, y: usize, fav: MagicNumber) -> bool {
    let n = x * x + 3 * x + 2 * x * y + y + y * y + fav;
    n.count_ones() % 2 == 0
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    count: usize,
    pos: Coord,
}
impl State {
    fn new(count: usize, pos: Coord) -> Self {
        State { count, pos }
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // fewer steps is better
        self.count.cmp(&other.count).reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(from: Coord, to: Coord, fav: MagicNumber) -> usize {
    let mut next = BinaryHeap::new();
    next.push(State {
        count: 0,
        pos: from,
    });
    let mut visited = HashSet::new();
    while let Some(State { count, pos }) = next.pop() {
        if pos == to {
            return count;
        }
        if !visited.insert(pos) {
            continue;
        }
        for neighbor in pos.neighbors() {
            if neighbor.is_open(fav) {
                next.push(State::new(count + 1, neighbor));
            }
        }
    }
    unreachable!()
}

fn reachable_with_steps(from: Coord, max_steps: usize, fav: MagicNumber) -> usize {
    let mut next = BinaryHeap::new();
    next.push(State {
        count: 0,
        pos: from,
    });
    let mut visited = HashSet::new();
    while let Some(State { count, pos }) = next.pop() {
        if !visited.insert(pos) {
            continue;
        }
        for neighbor in pos.neighbors() {
            if neighbor.is_open(fav) && count < max_steps {
                next.push(State::new(count + 1, neighbor));
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_grid() {
        let grid = create_grid(10, 7, 10);
        assert_eq!(
            grid,
            "\
.#.####.##
..#..#...#
#....##...
###.#.###.
.##..#..#.
..##....#.
#...##.###"
        );
    }

    fn create_grid(width: usize, height: usize, fav: MagicNumber) -> String {
        (0..height)
            .into_iter()
            .map(|y| {
                (0..width)
                    .into_iter()
                    .map(|x| if is_open(x, y, fav) { '.' } else { '#' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn part1_example_shortest_path() {
        let from = Coord::default();
        let to = Coord { x: 7, y: 4 };
        let fav = 10;
        assert_eq!(11, shortest_path(from, to, fav));
    }

    #[test]
    fn part1() {
        assert_eq!(92, day13_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(124, day13_part2());
    }
}
