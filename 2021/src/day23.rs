use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use Amphipod::*;

const INPUT: &str = include_str!("../input/day23.txt");
const INPUT2: &str = include_str!("../input/day23_2.txt");

pub(crate) fn day23_part1() -> usize {
    Burrow::from(INPUT).part1()
}

pub(crate) fn day23_part2() -> usize {
    Burrow::from(INPUT2).part2()
}

impl Burrow {
    fn part1(&mut self) -> Energy {
        let mut candidates = BinaryHeap::new();
        candidates.push(Reverse((0, self.initial_positions)));

        println!("{}", self.initial_positions);

        let mut seen: HashSet<Positions> = HashSet::new();

        let mut i = 0;
        while let Some(Reverse((energy, current))) = candidates.pop() {
            // println!(
            //     "------------- {} @ {}---------------\n{}\n",
            //     i, energy, current
            // );

            // 1. check if goal reached
            if current.is_finished() {
                println!(
                    "i = {}, {} candidates left\n{}",
                    i,
                    candidates.len(),
                    current
                );
                return energy;
            }
            // (state should not have been previously reached with lower energy, maybe check)
            if !seen.insert(current) {
                continue;
            }
            // 2. find amphipods that can move, and add their moves to the candidates
            for (cost, next) in current.get_next_possible_moves() {
                if !seen.contains(&next) {
                    candidates.push(Reverse((energy + cost, next)));
                }
            }
            i += 1;
            if i > 100_000 {
                println!(
                    "reached limit, # candidates {}, # seen {}\n{}",
                    candidates.len(),
                    seen.len(),
                    current
                );
                panic!()
            }
        }
        0
    }
    fn part2(&self) -> usize {
        0
    }
}
#[derive(Debug)]
struct Burrow {
    initial_positions: Positions,
}
impl From<&str> for Burrow {
    fn from(input: &str) -> Self {
        let mut lines = input.lines().skip(1);
        let h: Vec<_> = lines
            .next()
            .unwrap()
            .chars()
            .skip(1)
            .take(11)
            .map(|c| {
                if let Ok(pod) = Amphipod::try_from(c) {
                    Hallway::new(pod)
                } else {
                    Hallway::default()
                }
            })
            .collect();
        let room: Vec<_> = lines
            .take(2)
            .flat_map(|s| {
                s.replace('#', "")
                    .replace(' ', "")
                    .chars()
                    .map(|c| Amphipod::try_from(c).ok())
                    .collect::<Vec<Option<Amphipod>>>()
            })
            .collect();
        Burrow {
            initial_positions: Positions {
                hallway: [h[0], h[1], h[3], h[5], h[7], h[9], h[10]],
                rooms: [
                    Room {
                        top: room[0],
                        bottom: room[4],
                    },
                    Room {
                        top: room[1],
                        bottom: room[5],
                    },
                    Room {
                        top: room[2],
                        bottom: room[6],
                    },
                    Room {
                        top: room[3],
                        bottom: room[7],
                    },
                ],
            },
        }
    }
}

const TARGET_ROOMS: [Room; 4] = [
    Room {
        top: Some(Amber),
        bottom: Some(Amber),
    },
    Room {
        top: Some(Bronze),
        bottom: Some(Bronze),
    },
    Room {
        top: Some(Copper),
        bottom: Some(Copper),
    },
    Room {
        top: Some(Desert),
        bottom: Some(Desert),
    },
];

type Energy = usize;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Positions {
    hallway: [Hallway; 7],
    rooms: [Room; 4],
}
impl Display for Positions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // #############
        // #...........#
        // ###B#C#B#D###
        //   #A#D#C#A#
        //   #########
        let h = &self.hallway;
        let hallway = format!(
            "#{}{}.{}.{}.{}.{}{}#",
            h[0], h[1], h[2], h[3], h[4], h[5], h[6]
        );
        let r: Vec<char> = self.rooms.iter().map(|r| r.top.to_char()).collect();
        let rooms_top = format!("###{}#{}#{}#{}###", r[0], r[1], r[2], r[3]);
        let r: Vec<char> = self.rooms.iter().map(|r| r.bottom.to_char()).collect();
        let rooms_bottom = format!("  #{}#{}#{}#{}#  ", r[0], r[1], r[2], r[3]);
        write!(
            f,
            "#############\n{}\n{}\n{}\n  #########  ",
            hallway, rooms_top, rooms_bottom
        )
    }
}
impl Positions {
    fn is_finished(&self) -> bool {
        self.rooms == TARGET_ROOMS
    }
    fn get_next_possible_moves(&self) -> Vec<(Energy, Positions)> {
        // - Amphipods will never stop on the space immediately outside any room. They can move into
        //   that space so long as they immediately continue moving. (Specifically, this refers to the
        //   four open spaces in the hallway that are directly above an amphipod starting position.)
        //   -> These spaces don't appear in our graph

        // - Amphipods will never move from the hallway into a room unless that room is their destination
        //   room and that room contains no amphipods which do not also have that room as their own
        //   destination. If an amphipod's starting room is not its destination room, it can stay in
        //   that room until it leaves the room. (For example, an Amber amphipod will not move from
        //   the hallway into the right three rooms, and will only move into the leftmost room if
        //   that room is empty or if it only contains other Amber amphipods.)

        // - Once an amphipod stops moving in the hallway, it will stay in that spot until it can move
        //   into a room. (That is, once any amphipod starts moving, any other amphipods currently in
        //   the hallway are locked in place and will not move again until they can move fully into a room.)

        let mut positions = vec![];

        let hallway = self.occupied_hallways();
        let room_tops = self.occupied_room_tops();
        let room_bottoms = self.occupied_room_bottoms();

        // println!("{}", self);
        // println!("pods_in_hallway {:?}", hallway);
        // println!("pods_on_room_tops {:?}", room_tops);
        // println!("pods_in_room_bottoms {:?}", room_bottoms);

        for from in &hallway {
            for to in self.reachable_room_tops_left_of(*from) {
                positions.push(self.move_from_hallway_to_room_top(*from, to));
            }
            // println!("hallway from {}", from);
            for to in self.reachable_room_tops_right_of(*from) {
                // println!("reachable_room_tops_right_of: to = {}", to);
                positions.push(self.move_from_hallway_to_room_top(*from, to));
            }

            for to in self.reachable_room_bottoms_left_of(*from) {
                positions.push(self.move_from_hallway_to_room_bottom(*from, to));
            }
            for to in self.reachable_room_bottoms_right_of(*from) {
                positions.push(self.move_from_hallway_to_room_bottom(*from, to));
            }
        }
        for from in room_tops {
            for to in self.reachable_hallways_left_of(from + 2) {
                positions.push(self.move_from_room_top_to_hallway(from, to));
            }
            for to in self.reachable_hallways_right_of(from + 1) {
                positions.push(self.move_from_room_top_to_hallway(from, to));
            }
        }
        for from in room_bottoms {
            if self.rooms[from].has_top() {
                continue; // can't get out :)
            }
            for to in self.reachable_hallways_left_of(from + 2) {
                positions.push(self.move_from_room_bottom_to_hallway(from, to));
            }
            for to in self.reachable_hallways_right_of(from + 1) {
                positions.push(self.move_from_room_bottom_to_hallway(from, to));
            }
        }
        positions
    }

    fn occupied_hallways(&self) -> Vec<usize> {
        self.hallway
            .iter()
            .enumerate()
            .filter(|(_, h)| h.is_occupied())
            .map(|(i, _)| i)
            .collect()
    }

    fn occupied_room_tops(&self) -> Vec<usize> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|(_, r)| r.has_top())
            .filter(|(i, r)| !r.is_at_target(*i))
            .map(|(i, _)| i)
            .collect()
    }

    fn occupied_room_bottoms(&self) -> Vec<usize> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|(_, r)| r.has_bottom())
            .filter(|(i, r)| !r.is_at_target(*i))
            .map(|(i, _)| i)
            .collect()
    }

    fn reachable_hallways_left_of(&self, i: usize) -> Vec<usize> {
        self.hallway
            .iter()
            .enumerate()
            .rev()
            .skip(7 - i)
            .take_while(|(_, h)| h.is_empty())
            .map(|(i, _)| i)
            .collect()
    }

    fn reachable_hallways_right_of(&self, start: usize) -> Vec<usize> {
        self.hallway
            .iter()
            .enumerate()
            .skip(start + 1)
            .take_while(|(_, h)| h.is_empty())
            .map(|(i, _)| i)
            .collect()
    }

    fn room_left_of(hallway: usize) -> usize {
        match hallway {
            2 => 0,
            3 => 1,
            4 => 2,
            5..=6 => 3,
            _ => unreachable!(),
        }
    }
    fn room_right_of(hallway: usize) -> usize {
        match hallway {
            0..=1 => 0,
            2 => 1,
            3 => 2,
            4 => 3,
            _ => unreachable!(),
        }
    }

    fn reachable_room_tops_left_of(&self, from: usize) -> Vec<usize> {
        std::iter::once(from) // add self to get immediate next room
            .chain(self.reachable_hallways_left_of(from))
            .filter(|via| *via > 1)
            .filter_map(|via| {
                let room = Self::room_left_of(via);
                self.map_top(room, from)
            })
            .collect()
    }

    fn reachable_room_tops_right_of(&self, from: usize) -> Vec<usize> {
        iter::once(from) // add self to get immediate next room
            .chain(self.reachable_hallways_right_of(from))
            .filter(|via| *via < 5)
            .filter_map(|via| {
                let room = Self::room_right_of(via);
                // println!(
                //     "reachable_room_tops_right_of from {} via {} has right room {}",
                //     from, via, room
                // );
                self.map_top(room, from)
            })
            .collect()
    }

    fn reachable_room_bottoms_left_of(&self, from: usize) -> Vec<usize> {
        iter::once(from) // add self to get immediate next room
            .chain(self.reachable_hallways_right_of(from))
            .filter(|via| *via > 1)
            .filter_map(|via| {
                let room = Self::room_left_of(via);
                self.map_bottom(room, from)
            })
            .collect()
    }

    fn reachable_room_bottoms_right_of(&self, from: usize) -> Vec<usize> {
        iter::once(from) // add self to get immediate next room
            .chain(self.reachable_hallways_right_of(from))
            .into_iter()
            .filter(|via| *via < 5)
            .filter_map(|via| {
                let room = Self::room_right_of(via);
                self.map_bottom(room, from)
            })
            .collect()
    }

    fn map_top(&self, room: usize, from_hallway: usize) -> Option<usize> {
        let pod = self.hallway[from_hallway].amphipod.unwrap();
        let is_top = self.rooms[room].is_top_empty() && self.rooms[room].bottom.eq(&Some(pod));
        // println!("pod {} wants({}) = {}", pod, room, pod.wants(room));
        // println!(
        //     "self.rooms[room].is_top_empty() {}",
        //     self.rooms[room].is_top_empty()
        // );
        // println!(
        //     "self.rooms[room].bottom.eq(&Some(pod)) {}",
        //     self.rooms[room].bottom.eq(&Some(pod))
        // );
        if pod.wants(room) && is_top {
            Some(room)
        } else {
            None
        }
    }

    fn map_bottom(&self, room: usize, from_hallway: usize) -> Option<usize> {
        let pod = self.hallway[from_hallway].amphipod.unwrap();
        if pod.wants(room) && self.rooms[room].is_empty() {
            Some(room)
        } else {
            None
        }
    }

    fn hallway_step_count(from: usize, to: usize) -> usize {
        assert_ne!(from, to);
        let mut steps = 2 * if from < to { to - from } else { from - to };
        if from == 0 || from == 6 {
            steps -= 1;
        }
        if to == 0 || to == 6 {
            steps -= 1;
        }
        steps
    }

    fn room_top_to_hallway_step_count(from_room: usize, to_hallway: usize) -> usize {
        // determine the hallway equivalent
        let from_hallway = match (from_room, to_hallway) {
            (0, 0..=1) => 2,
            (0, 2..=6) => 1,
            (1, 0..=2) => 3,
            (1, 3..=6) => 2,
            (2, 0..=3) => 4,
            (2, 4..=6) => 3,
            (3, 0..=4) => 5,
            (3, 5..=6) => 4,
            _ => unreachable!("from {} to {}", from_room, to_hallway),
        };
        // println!("{} ~ {} (room ~ hallway)", from_room, from_hallway);
        Self::hallway_step_count(from_hallway, to_hallway)
    }

    fn move_from_room_top_to_hallway(&self, from: usize, to: usize) -> (Energy, Self) {
        let mut result = *self;
        if let Some(pod) = result.rooms[from].top.take() {
            assert_eq!(result.hallway[to].amphipod, None);
            result.hallway[to].amphipod = Some(pod);
            let steps = Self::room_top_to_hallway_step_count(from, to);
            let energy = pod.energy_for(steps);
            // println!(
            //     "move {} from room top {} to hallway {} in {} steps @ energy {}\n{}\n",
            //     pod, from, to, steps, energy, result
            // );
            (energy, result)
        } else {
            unreachable!("move_from_room_top_to_hallway({}, {})\n{}", from, to, self);
        }
    }

    fn move_from_hallway_to_room_top(&self, from: usize, to: usize) -> (Energy, Self) {
        let mut result = *self;
        if let Some(pod) = result.hallway[from].amphipod.take() {
            assert_eq!(result.rooms[to].top, None);
            result.rooms[to].top = Some(pod);

            let steps = Self::room_top_to_hallway_step_count(to, from);
            let energy = pod.energy_for(steps);
            // println!(
            //     "move {} from hallway {} to room top {} in {} steps @ energy {}\n{}\n",
            //     pod, from, to, steps, energy, result
            // );
            (energy, result)
        } else {
            unreachable!("move_from_hallway_to_room_top({},{})", from, to);
        }
    }

    fn move_from_hallway_to_room_bottom(&self, from: usize, to: usize) -> (Energy, Self) {
        let mut result = *self;
        if let Some(pod) = result.hallway[from].amphipod.take() {
            assert_eq!(result.rooms[to].top, None);
            assert_eq!(result.rooms[to].bottom, None);
            result.rooms[to].bottom = Some(pod);

            let steps = 1 + Self::room_top_to_hallway_step_count(to, from);
            let energy = pod.energy_for(steps);
            // println!(
            //     "move {} from hallway {} to room bottom {} in {} steps @ energy {}\n{}\n",
            //     pod, from, to, steps, energy, result
            // );
            (energy, result)
        } else {
            unreachable!("move_from_hallway_to_room_top({},{})", from, to);
        }
    }

    fn move_from_room_bottom_to_hallway(&self, from: usize, to: usize) -> (Energy, Self) {
        let mut result = *self;
        assert_eq!(result.rooms[from].top, None);
        if let Some(pod) = result.rooms[from].bottom.take() {
            assert_eq!(result.hallway[to].amphipod, None);
            result.hallway[to].amphipod = Some(pod);
            let steps = 1 + Self::room_top_to_hallway_step_count(from, to);
            let energy = pod.energy_for(steps);
            // println!(
            //     "move {} from room bottom {} to hallway {} in {} steps @ energy {}\n{}\n",
            //     pod, from, to, steps, energy, result
            // );
            (energy, result)
        } else {
            unreachable!("move_from_room_bottom_to_hallway({}, {})", from, to);
        }
    }
}
impl Debug for Positions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Hallway {
    amphipod: Option<Amphipod>,
}
impl Hallway {
    fn new(pod: Amphipod) -> Self {
        Hallway {
            amphipod: Some(pod),
        }
    }
    fn is_empty(&self) -> bool {
        self.amphipod.is_none()
    }
    fn is_occupied(&self) -> bool {
        self.amphipod.is_some()
    }
}
impl Display for Hallway {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.amphipod.to_char())
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}
impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Amber => 'A',
                Bronze => 'B',
                Copper => 'C',
                Desert => 'D',
            }
        )
    }
}
impl Amphipod {
    fn wants(&self, room: usize) -> bool {
        room == self.wanted_room()
    }

    fn wanted_room(&self) -> usize {
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }
}
trait ToChar {
    fn to_char(&self) -> char;
}
impl ToChar for Option<Amphipod> {
    fn to_char(&self) -> char {
        match self {
            None => '.',
            Some(pod) => pod.to_string().chars().next().unwrap(),
        }
    }
}
impl TryFrom<char> for Amphipod {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Amber),
            'B' => Ok(Bronze),
            'C' => Ok(Copper),
            'D' => Ok(Desert),
            _ => Err(format!("Unexpected character '{}'", value)),
        }
    }
}
impl Amphipod {
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
struct Room {
    top: Option<Amphipod>,
    bottom: Option<Amphipod>,
}
impl Room {
    fn new(top: Amphipod, bottom: Amphipod) -> Self {
        Room {
            top: Some(top),
            bottom: Some(bottom),
        }
    }
    fn is_top_empty(&self) -> bool {
        self.top.is_none()
    }
    fn is_bottom_empty(&self) -> bool {
        self.bottom.is_none()
    }
    fn is_empty(&self) -> bool {
        self.top.is_none() && self.bottom.is_none()
    }
    fn has_top(&self) -> bool {
        self.top.is_some()
    }
    fn has_bottom(&self) -> bool {
        self.bottom.is_some()
    }
    fn is_bottom_done(&self, room: usize) -> bool {
        self.bottom.map(|pod| pod.wants(room)).unwrap_or(false)
    }
    fn is_at_target(&self, room: usize) -> bool {
        self.top.map(|pod| pod.wants(room)).unwrap_or(false) && self.is_bottom_done(room)
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
        let Burrow {
            initial_positions: mut positions,
        } = Burrow::from(EXAMPLE);
        let expected = vec![
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
                420,
                Burrow::from(
                    "\
#############
#...B.C.....#
###B#.#.#D###
  #A#D#C#A#  
  #########  ",
                ),
            ),
            (
                440,
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
                3440,
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
                3470,
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
                3490,
                Burrow::from(
                    "\
#############
#...B.D.....#
###.#.#C#D###
  #A#B#C#A#  
  #########  ",
                ),
            ),
            (
                3510,
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
                5513,
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
                12513,
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
                12491,
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

        for i in 0..7 {
            let mut one_matched = false;
            let (
                exp_e,
                Burrow {
                    initial_positions: exp_pos,
                },
            ) = expected[i];
            println!("expected {}\n{}\n", exp_e, exp_pos);
            for (next_e, next_pos) in positions.get_next_possible_moves() {
                if next_pos == exp_pos {
                    one_matched = true;
                    positions = next_pos;
                    break;
                }
            }
            if !one_matched {
                println!("did not get match for next of\n{}got\n", positions);
                for (next_e, next_pos) in positions.get_next_possible_moves() {
                    println!("{}\n{}\n", next_e, next_pos)
                }
            }
            assert!(one_matched);
        }
    }

    #[test]
    fn part1_debug() {
        let Burrow {
            initial_positions: positions,
        } = Burrow::from(
            "\
#############
#...B.D.....#
###B#.#C#D###
  #A#.#C#A#  
  #########  ",
        );
        println!("{}\n", positions);
        let next = positions.get_next_possible_moves();
        let len = next.len();
        for (i, (e, next)) in next.into_iter().enumerate() {
            println!("{}/{}:\n{}\n", i, len, next);
        }
    }

    #[test]
    fn part1_example_manual() {
        let Burrow {
            initial_positions: positions,
        } = Burrow::from(EXAMPLE);

        let mut energy = 0;
        let (e, positions) = positions.move_from_room_top_to_hallway(2, 2);
        energy += e;
        let (e, positions) = positions.move_from_room_top_to_hallway(1, 3);
        energy += e;
        let (e, positions) = positions.move_from_hallway_to_room_top(3, 2);
        energy += e;
        let (e, positions) = positions.move_from_room_bottom_to_hallway(1, 3);
        energy += e;
        let (e, positions) = positions.move_from_hallway_to_room_bottom(2, 1);
        energy += e;
        let (e, positions) = positions.move_from_room_top_to_hallway(0, 2);
        energy += e;
        let (e, positions) = positions.move_from_hallway_to_room_top(2, 1);
        energy += e;
        let (e, positions) = positions.move_from_room_top_to_hallway(3, 4);
        energy += e;
        let (e, positions) = positions.move_from_room_bottom_to_hallway(3, 5);
        energy += e;
        let (e, positions) = positions.move_from_hallway_to_room_bottom(4, 3);
        energy += e;
        let (e, positions) = positions.move_from_hallway_to_room_top(3, 3);
        energy += e;
        let (e, positions) = positions.move_from_hallway_to_room_top(5, 0);
        energy += e;
        assert_eq!(energy, 12521);
        assert!(positions.is_finished());
    }

    #[test]
    fn part1_example() {
        assert_eq!(12521, Burrow::from(EXAMPLE).part1());
    }

    #[test]
    fn part1() {
        // 16089 is too high
        assert_eq!(16059, day23_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(44169, Burrow::from(EXAMPLE2).part2());
    }

    #[test]
    fn part2() {
        assert_eq!(1, day23_part2());
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
