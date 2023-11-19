use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use Direction::*;

const INPUT: &str = include_str!("../input/day23.txt");

pub(crate) fn day23_part1() -> usize {
    Elves::from(INPUT).move_10_rounds().empty_tile_count()
}

pub(crate) fn day23_part2() -> usize {
    Elves::from(INPUT).count_rounds_until_stable()
}

type Coord = isize;
struct Elves {
    // Origin (0, 0) starts out at top left, x points right and y points down
    set: HashSet<Pos>,
}
impl Elves {
    fn move_10_rounds(mut self) -> Self {
        let mut directions = [N, S, W, E].into_iter().cycle();
        for _ in 1..=10 {
            self.move_1_round(&directions.clone().take(4).collect::<Vec<_>>());
            directions.next();
        }
        self
    }
    fn count_rounds_until_stable(mut self) -> usize {
        let mut directions = [N, S, W, E].into_iter().cycle();
        let mut count = 1;
        while self.move_1_round(&directions.clone().take(4).collect::<Vec<_>>()) {
            directions.next();
            count += 1;
        }
        count
    }
    fn move_1_round(&mut self, directions: &[Direction]) -> bool {
        let mut new_pos = HashSet::<Pos>::new();
        let mut old_pos_by_desired_pos: HashMap<Pos, Vec<Pos>> = HashMap::new();
        'LOOPING_OVER_ELVES: for curr_pos in &self.set {
            if curr_pos.has_no_neighbors(&self.set) {
                // Stay in the same position because there's no reason to move
                new_pos.insert(*curr_pos);
                continue 'LOOPING_OVER_ELVES;
            }
            // Otherwise check directions for a desired position
            if let Some(desired_pos) = curr_pos.first_valid_position(directions, &self.set) {
                let old_pos = old_pos_by_desired_pos.entry(desired_pos).or_default();
                old_pos.push(*curr_pos);
            } else {
                // Stay in the same position because the desired positions are too crowded
                new_pos.insert(*curr_pos);
            }
        }
        // Move to desired positions if only one, otherwise stay at old position
        for (desired_pos, old_pos) in old_pos_by_desired_pos {
            if old_pos.len() == 1 {
                new_pos.insert(desired_pos);
            } else {
                old_pos.into_iter().for_each(|old_pos| {
                    new_pos.insert(old_pos);
                });
            }
        }
        let something_moved = new_pos != self.set;
        if something_moved {
            self.set = new_pos;
        }
        something_moved
    }
    fn empty_tile_count(&self) -> usize {
        let x_range = self.x_range();
        let width = (x_range.end() - x_range.start() + 1) as usize;

        let y_range = self.y_range();
        let height = (y_range.end() - y_range.start() + 1) as usize;

        let tile_count = width * height;
        let elf_count = self.set.len();
        tile_count - elf_count
    }
    fn x_range(&self) -> RangeInclusive<Coord> {
        let min_x = *self.set.iter().map(|Pos { x, .. }| x).min().unwrap();
        let max_x = *self.set.iter().map(|Pos { x, .. }| x).max().unwrap();
        min_x..=max_x
    }
    fn y_range(&self) -> RangeInclusive<Coord> {
        let min_y = *self.set.iter().map(|Pos { y, .. }| y).min().unwrap();
        let max_y = *self.set.iter().map(|Pos { y, .. }| y).max().unwrap();
        min_y..=max_y
    }
}
impl From<&str> for Elves {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| Pos {
                        x: x as Coord,
                        y: y as Coord,
                    })
            })
            .collect();
        Elves { set: map }
    }
}
impl Display for Elves {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.y_range()
                .map(|y| self
                    .x_range()
                    .into_iter()
                    .map(|x| if self.set.contains(&Pos { x, y }) {
                        '#'
                    } else {
                        '.'
                    })
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: Coord,
    y: Coord,
}
impl Pos {
    fn has_no_neighbors(&self, set: &HashSet<Pos>) -> bool {
        vec![N, NE, E, SE, S, SW, W, NW]
            .into_iter()
            .map(|direction| self.moved_in(&direction))
            .all(|new_pos| !set.contains(&new_pos))
    }
    fn first_valid_position(
        &self,
        directions: &[Direction],
        occupied: &HashSet<Pos>,
    ) -> Option<Pos> {
        directions
            .iter()
            .filter(|direction| direction.is_available_for(self, occupied))
            .map(|direction| self.moved_in(direction))
            .next()
    }
    fn moved_in(&self, direction: &Direction) -> Pos {
        let offset = direction.to_offset();
        Pos {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Direction {
    fn to_offset(self) -> Pos {
        match self {
            N => Pos { x: 0, y: -1 },
            NE => Pos { x: 1, y: -1 },
            E => Pos { x: 1, y: 0 },
            SE => Pos { x: 1, y: 1 },
            S => Pos { x: 0, y: 1 },
            SW => Pos { x: -1, y: 1 },
            W => Pos { x: -1, y: 0 },
            NW => Pos { x: -1, y: -1 },
        }
    }
    fn is_available_for(&self, pos: &Pos, occupied: &HashSet<Pos>) -> bool {
        self.related_directions()
            .map(|direction| pos.moved_in(&direction))
            .all(|pos| !occupied.contains(&pos))
    }
    fn related_directions(&self) -> impl Iterator<Item = Direction> {
        match self {
            N => [NW, N, NE].into_iter(),
            E => [NE, E, SE].into_iter(),
            S => [SE, S, SW].into_iter(),
            W => [SW, W, NW].into_iter(),
            _ => panic!("only orthogonal directions should be considered"),
        }
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                N => "N",
                NE => "NE",
                E => "E",
                SE => "SE",
                S => "S",
                SW => "SW",
                W => "W",
                NW => "NW",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part1_example() {
        assert_eq!(
            110,
            Elves::from(EXAMPLE).move_10_rounds().empty_tile_count()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(3_874, day23_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(20, Elves::from(EXAMPLE).count_rounds_until_stable());
    }

    #[test]
    fn part2() {
        assert_eq!(948, day23_part2());
    }
}
