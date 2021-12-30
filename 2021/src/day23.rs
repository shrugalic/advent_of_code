use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;
use Amphipod::*;

const INPUT: &str = include_str!("../input/day23.txt");
const INPUT2: &str = include_str!("../input/day23_2.txt");

pub(crate) fn day23_part1() -> usize {
    Burrow::from(INPUT).solve()
}

pub(crate) fn day23_part2() -> usize {
    Burrow::from(INPUT2).solve()
}

impl Burrow {
    fn solve(self) -> Energy {
        let mut candidates = BinaryHeap::new();
        candidates.push(Reverse((0, self)));

        let mut best: HashMap<Burrow, Energy> = HashMap::new();

        while let Some(Reverse((energy, current))) = candidates.pop() {
            // 1. check if goal reached
            if current.is_finished() {
                return energy;
            }
            // Discard positions that have previously been reached with lower energy
            if let Some(prev) = best.get(&current) {
                if *prev < energy {
                    continue;
                }
            }

            // 2. find amphipods that can move, and add their moves to the candidates
            for (cost, next) in current.get_next_possible_moves(energy, &best) {
                if let Some(prev) = best.get(&next) {
                    if *prev < energy + cost {
                        continue;
                    }
                }
                best.insert(next.clone(), energy + cost);
                candidates.push(Reverse((energy + cost, next)));
            }
        }
        unreachable!()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Burrow {
    grid: Grid,
}
impl From<&str> for Burrow {
    fn from(input: &str) -> Self {
        Burrow {
            grid: input
                .trim()
                .lines()
                .map(|line| line.chars().map(|c| Amphipod::try_from(c).ok()).collect())
                .collect(),
        }
    }
}
impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .enumerate()
                .map(|(y, row)| row
                    .iter()
                    .enumerate()
                    .map(|(x, pod)| pod
                        .map(|pod| pod.to_char())
                        .unwrap_or_else(|| match (x, y) {
                            (_, 0) => '#',
                            (1..=11, HALLWAY_Y) => '.',
                            (0 | 1 | 11 | 12, 3..=6) => ' ',
                            (3 | 5 | 7 | 9, 2..=3) => '.',
                            (_, _) => '#',
                        }))
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

type Energy = usize;
type Grid = Vec<Vec<Option<Amphipod>>>;
type Move = (Energy, Burrow);
type X = usize;
type Y = usize;

const ROOM_XS: [usize; 4] = [3, 5, 7, 9];
const VALID_HALLWAY_XS: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];
const ALL_HALLWAY_XS: [usize; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const HALLWAY_Y: usize = 1;

impl Burrow {
    fn is_finished(&self) -> bool {
        ROOM_XS
            .iter()
            .all(|&room_x| self.is_room_full_and_sorted(room_x))
    }
    fn is_room_full_and_sorted(&self, room_x: usize) -> bool {
        self.room_ys().all(|room_y| {
            if let Some(pod) = self.grid[room_y][room_x] {
                pod.has_destination(room_x)
            } else {
                false
            }
        })
    }
    fn room_ys(&self) -> Range<usize> {
        2..self.grid.len() - 1
    }
    fn get_next_possible_moves(&self, energy: Energy, best: &HashMap<Burrow, Energy>) -> Vec<Move> {
        let mut moves = vec![];

        for (from_x, from_y, pod) in self.occupied_rooms() {
            let rooms = self.reachable_rooms_from_room(from_x, pod);
            // Prioritize rooms over hallways
            if !rooms.is_empty() {
                for to in rooms {
                    moves.push(self.move_from_room_to_room(from_x, from_y, to, pod));
                }
            } else {
                for to in self.reachable_hallways_from_room(from_x) {
                    moves.push(self.move_from_room_to_hallway(from_x, from_y, to, pod));
                }
            }
        }
        for (from, pod) in self.occupied_hallways() {
            for to in self.reachable_rooms_from_hallway(from, pod) {
                moves.push(self.move_from_hallway_to_room(from, to, pod));
            }
        }

        // Return only the moves that lead to better results
        moves
            .iter()
            .filter(|(cost, next)| {
                best.get(next)
                    .map(|total| energy + *cost < *total)
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    }

    fn occupied_hallways(&self) -> Vec<(X, Amphipod)> {
        self.grid[1]
            .iter()
            .enumerate()
            .filter_map(|(x, pod)| pod.map(|pod| (x, pod)))
            .collect()
    }

    fn occupied_rooms(&self) -> Vec<(X, Y, Amphipod)> {
        ROOM_XS
            .into_iter()
            .filter_map(|x| self.topmost_occupied_room(x).map(|(y, pod)| (x, y, pod)))
            .filter(|(x, _y, _pod)| !self.is_room_full_and_sorted(*x))
            .collect()
    }

    fn reachable_rooms_from_room(&self, from: X, pod: Amphipod) -> Vec<usize> {
        self.reachable_rooms_from_hallway(from, pod)
            .into_iter()
            .filter(|&to| to != from)
            .collect()
    }

    fn reachable_hallways_from_room(&self, from: usize) -> Vec<usize> {
        VALID_HALLWAY_XS
            .into_iter()
            .rev()
            .skip_while(|&x| from <= x)
            .take_while(|&left| self.is_empty_hallway(left))
            .chain(
                VALID_HALLWAY_XS
                    .into_iter()
                    .skip_while(|&x| x <= from)
                    .take_while(|&right| self.is_empty_hallway(right)),
            )
            .collect()
    }

    fn is_empty_hallway(&self, x: X) -> bool {
        self.grid[HALLWAY_Y][x].is_none()
    }

    fn reachable_rooms_from_hallway(&self, from: X, pod: Amphipod) -> Vec<X> {
        let left = ALL_HALLWAY_XS
            .into_iter()
            .rev()
            .skip_while(|&x| from <= x)
            .take_while(|&left| self.is_empty_hallway(left));
        let right = ALL_HALLWAY_XS
            .into_iter()
            .skip_while(|&x| x <= from)
            .take_while(|&right| self.is_empty_hallway(right));
        left.chain(right)
            .filter(|x| ROOM_XS.contains(x))
            .filter(|room| pod.has_destination(*room))
            .filter(|x| self.all_occupants_match(*x, &pod))
            .collect()
    }

    fn move_from_room_to_room(&self, from: X, y: Y, to: X, pod: Amphipod) -> (Energy, Self) {
        let via_hallway = (from + to) / 2;
        let (e1, intermediate) = self.move_from_room_to_hallway(from, y, via_hallway, pod);
        let (e2, next) = intermediate.move_from_hallway_to_room(via_hallway, to, pod);
        (e1 + e2, next)
    }

    fn move_from_room_to_hallway(&self, from: X, y: Y, to: X, pod: Amphipod) -> (Energy, Self) {
        let mut next = self.clone();
        next.grid[y][from] = None;
        next.grid[HALLWAY_Y][to] = Some(pod);

        let y_steps = y - HALLWAY_Y;
        let x_steps = if from < to { to - from } else { from - to };

        (pod.energy_for(y_steps + x_steps), next)
    }

    fn move_from_hallway_to_room(&self, from: X, to: X, pod: Amphipod) -> (Energy, Self) {
        let mut next = self.clone();
        let room_y = next.top_empty_room_y(to);
        next.grid[HALLWAY_Y][from] = None;
        next.grid[room_y][to] = Some(pod);

        let x_steps = if from < to { to - from } else { from - to };
        let y_steps = room_y - HALLWAY_Y;

        (pod.energy_for(y_steps + x_steps), next)
    }

    fn topmost_occupied_room(&self, x: X) -> Option<(Y, Amphipod)> {
        self.room_ys()
            .filter_map(|y| self.grid[y][x].map(|pod| (y, pod)))
            .next()
    }

    fn all_occupants_match(&self, room: X, expected: &Amphipod) -> bool {
        self.room_ys().all(|y| {
            self.grid[y][room]
                .map(|occupant| occupant == *expected)
                .unwrap_or(true)
        })
    }

    fn top_empty_room_y(&self, room: X) -> usize {
        self.topmost_occupied_room(room)
            .map(|(y, _)| y)
            .unwrap_or(self.grid.len() - 1)
            - 1
    }
    #[cfg(test)]
    fn get_pod(&self, room_x: X) -> Amphipod {
        self.grid[HALLWAY_Y][room_x].unwrap()
    }
}

trait ToChar {
    fn to_char(&self) -> char;
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}
impl TryFrom<char> for Amphipod {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Amber),
            'B' => Ok(Bronze),
            'C' => Ok(Copper),
            'D' => Ok(Desert),
            c => Err(format!("Illegal Amphipod char '{}'", c)),
        }
    }
}
impl ToChar for Amphipod {
    fn to_char(&self) -> char {
        match self {
            Amber => 'A',
            Bronze => 'B',
            Copper => 'C',
            Desert => 'D',
        }
    }
}
impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
impl Amphipod {
    fn has_destination(&self, room_x: usize) -> bool {
        room_x == self.destination_room()
    }
    fn destination_room(&self) -> usize {
        match self {
            Amber => 3,
            Bronze => 5,
            Copper => 7,
            Desert => 9,
        }
    }
    fn energy_per_step(&self) -> Energy {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
    fn energy_for(&self, steps: usize) -> Energy {
        steps * self.energy_per_step()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#  
  #########";

    const EXAMPLE2: &str = "\
#############
#...........#
###B#C#B#D###
  #D#C#B#A#  
  #D#B#A#C#  
  #A#D#C#A#  
  #########";

    #[test]
    fn part1_test_all_example_next_steps() {
        let mut burrow = Burrow::from(EXAMPLE);
        let expecteds = vec![
            (
                40,
                Burrow::from(
                    "\
#############
#...B.......#
###B#C#.#D###
  #A#D#C#A#  
  #########  ",
                ),
            ),
            (
                400,
                Burrow::from(
                    "\
#############
#...B.......#
###B#.#C#D###
  #A#D#C#A#  
  #########  ",
                ),
            ),
            (
                3000,
                Burrow::from(
                    "\
#############
#...B.D.....#
###B#.#C#D###
  #A#.#C#A#  
  #########  ",
                ),
            ),
            (
                30,
                Burrow::from(
                    "\
#############
#.....D.....#
###B#.#C#D###
  #A#B#C#A#  
  #########  ",
                ),
            ),
            (
                40,
                Burrow::from(
                    "\
#############
#.....D.....#
###.#B#C#D###
  #A#B#C#A#  
  #########  ",
                ),
            ),
            (
                2000,
                Burrow::from(
                    "\
#############
#.....D.D...#
###.#B#C#.###
  #A#B#C#A#  
  #########  ",
                ),
            ),
            (
                3,
                Burrow::from(
                    "\
#############
#.....D.D.A.#
###.#B#C#.###
  #A#B#C#.#  
  #########  ",
                ),
            ),
            (
                3000,
                Burrow::from(
                    "\
#############
#.....D...A.#
###.#B#C#.###
  #A#B#C#D#  
  #########  ",
                ),
            ),
            (
                4000,
                Burrow::from(
                    "\
#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#  
  #########  ",
                ),
            ),
            (
                8,
                Burrow::from(
                    "\
#############
#...........#
###A#B#C#D###
  #A#B#C#D#  
  #########  ",
                ),
            ),
        ];

        for (exp_e, exp) in expecteds {
            let mut one_matched = false;
            println!("expected {}\n{}\n", exp_e, exp);
            for (next_e, next) in burrow.get_next_possible_moves(0, &HashMap::new()) {
                if next == exp {
                    assert_eq!(next_e, exp_e);
                    one_matched = true;
                    burrow = next;
                    break;
                }
            }
            if !one_matched {
                println!(
                    "did not get match for next of\n{}\n\nbut instead got\n",
                    burrow
                );
                let next = burrow.get_next_possible_moves(0, &HashMap::new());
                if next.is_empty() {
                    println!("nothing")
                } else {
                    for (next_e, next_pos) in next {
                        println!("{}\n{}\n", next_e, next_pos)
                    }
                }
            }
            assert!(one_matched, "nothing matched");
        }
    }

    #[test]
    fn part1_debug() {
        let burrow = Burrow::from(
            "\
#############
#.B.....D...#
###.#C#B#.###
  #A#D#C#A#  
  #########  ",
        );
        println!("{}\n", burrow);
        let next = burrow.get_next_possible_moves(0, &HashMap::new());
        let len = next.len();
        for (i, (e, next)) in next.into_iter().enumerate() {
            println!("{}/{} @ {}:\n{}\n", i + 1, len, e, next);
        }
    }

    #[test]
    fn part1_example_manual() {
        let burrow = Burrow::from(EXAMPLE);

        let mut energy = 0;
        let (y, pod) = burrow.topmost_occupied_room(7).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(7, y, 4, pod);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(5).unwrap();
        let (e, burrow) = burrow.move_from_room_to_room(5, y, 7, pod);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(5).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(5, y, 6, pod);
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(4, 5, burrow.get_pod(4));
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(3).unwrap();
        let (e, burrow) = burrow.move_from_room_to_room(3, y, 5, pod);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(9).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(9, y, 8, pod);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(9).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(9, y, 10, pod);
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(8, 9, burrow.get_pod(8));
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(6, 9, burrow.get_pod(6));
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(10, 3, burrow.get_pod(10));
        energy += e;
        assert_eq!(energy, 12521);
        assert!(burrow.is_finished());
    }

    #[test]
    fn part1_manual() {
        let burrow = Burrow::from(INPUT);
        let mut energy = 0;

        let (y, pod) = burrow.topmost_occupied_room(7).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(7, y, 8, pod);
        assert_eq!(e, 200);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(7).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(7, y, 4, pod);
        assert_eq!(e, 50);
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(8, 7, burrow.get_pod(8));
        assert_eq!(e, 300);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(9).unwrap();
        let (e, burrow) = burrow.move_from_room_to_room(9, y, 7, pod);
        assert_eq!(e, 400);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(9).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(9, y, 11, pod);
        assert_eq!(e, 4);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(5).unwrap();
        let (e, burrow) = burrow.move_from_room_to_room(5, y, 9, pod);
        assert_eq!(e, 7000);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(5).unwrap();
        let (e, burrow) = burrow.move_from_room_to_hallway(5, y, 10, pod);
        assert_eq!(e, 7);
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(4, 5, burrow.get_pod(4));
        assert_eq!(e, 30);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(3).unwrap();
        let (e, burrow) = burrow.move_from_room_to_room(3, y, 9, pod);
        assert_eq!(e, 8000);
        energy += e;

        let (y, pod) = burrow.topmost_occupied_room(3).unwrap();
        let (e, burrow) = burrow.move_from_room_to_room(3, y, 5, pod);
        assert_eq!(e, 50);
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(10, 3, burrow.get_pod(10));
        assert_eq!(e, 9);
        energy += e;

        let (e, burrow) = burrow.move_from_hallway_to_room(11, 3, burrow.get_pod(11));
        assert_eq!(e, 9);
        energy += e;

        assert_eq!(energy, 16059);
        assert!(burrow.is_finished());
    }

    #[test]
    fn part1_example() {
        assert_eq!(12521, Burrow::from(EXAMPLE).solve());
    }

    #[test]
    fn part1() {
        assert_eq!(16059, day23_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(44169, Burrow::from(EXAMPLE2).solve());
    }

    #[test]
    fn part2() {
        assert_eq!(43117, day23_part2());
    }
}

// Part 1 manually solved
//
// #############
// #...........#
// ###D#D#C#C###
//   #B#A#B#A#
//   #########
//
// #############
// #...B...C...# 200 + 50
// ###D#D#.#C###
//   #B#A#.#A#
//   #########
//
// #############
// #...B.......# 700
// ###D#D#C#.###
//   #B#A#C#A#
//   #########
//
// #############
// #...B.....AA# 11 + 7000
// ###D#.#C#.###
//   #B#.#C#D#
//   #########
//
// #############
// #.........AA# 30 + 8000
// ###.#.#C#D###
//   #B#B#C#D#
//   #########
//
// #############
// #.........AA# 50 + 18
// ###A#B#C#D###
//   #A#B#C#D#
//   #########
//
// 16059
