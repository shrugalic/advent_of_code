use crate::parse;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("../input/day11.txt");

pub(crate) fn day11_part1() -> usize {
    let mut facility = Facility::from(parse(INPUT));
    facility.steps_to_bring_everything_to_floor_3()
}

pub(crate) fn day11_part2() -> usize {
    let mut facility = Facility::from(parse(INPUT));
    facility.microchips.push(0);
    facility.generators.push(0);
    facility.microchips.push(0);
    facility.generators.push(0);
    facility.steps_to_bring_everything_to_floor_3()
}

type Steps = usize;
type Floor = usize;
type Element = usize;
#[derive(PartialEq, Debug)]
struct Microchip {
    element: Element,
}
impl Microchip {
    fn matches(&self, generator: &Generator) -> bool {
        self.element == generator.element
    }
    // A chip is safe if there are no generators, or if its matching generator is nearby
    fn is_safe(&self, generators: &[Generator]) -> bool {
        generators.is_empty() || generators.iter().any(|other| self.matches(other))
    }
}
#[derive(PartialEq, Debug)]
struct Generator {
    element: Element,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    steps: Steps,
    facility: Facility,
}
impl State {
    fn take_elevator(
        &self,
        gens: &[&Generator],
        chips: &[&Microchip],
        next_floor: Floor,
    ) -> Option<State> {
        let facility = self.facility.take_elevator(gens, chips, next_floor);
        if facility.is_safe() {
            Some(State {
                steps: self.steps + 1,
                facility,
            })
        } else {
            None
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Prefer fewer steps, or higher elevator
        // (because the goal is to move everything to the top floor)
        match self.steps.cmp(&other.steps) {
            Ordering::Equal => self
                .facility
                .elevator_floor
                .cmp(&other.facility.elevator_floor),
            steps => steps.reverse(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Facility {
    generators: Vec<Floor>,
    microchips: Vec<Floor>,
    elevator_floor: Floor,
}
impl From<Vec<&str>> for Facility {
    fn from(input: Vec<&str>) -> Self {
        let mut elements = vec![];
        input
            .iter()
            .filter(|line| line.contains("compatible"))
            .for_each(|line| {
                line.split(|c| c == ' ' || c == '-')
                    .collect::<Vec<_>>()
                    .windows(2)
                    .for_each(|pair| {
                        if pair[1] == "compatible" {
                            elements.push(pair[0]);
                        }
                    })
            });
        let mut generators = vec![0; elements.len()];
        let separators = |c| c == ' ' || c == '-' || c == ',' || c == '.';
        let element_idx = |s| elements.iter().position(|&el| el == s).unwrap();

        input.iter().enumerate().for_each(|(floor, line)| {
            line.split(separators)
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|pair| {
                    if pair[1] == "generator" {
                        generators[element_idx(pair[0])] = floor;
                    }
                })
        });

        let mut microchips = vec![0; elements.len()];
        input.iter().enumerate().for_each(|(floor, line)| {
            line.split(separators)
                .collect::<Vec<_>>()
                .windows(3)
                .for_each(|triple| {
                    if triple[2] == "microchip" {
                        microchips[element_idx(triple[0])] = floor;
                    }
                })
        });
        Facility {
            generators,
            microchips,
            elevator_floor: 0,
        }
    }
}
impl Facility {
    fn state(&self) -> Vec<usize> {
        let mut counts = vec![self.elevator_floor];
        for floor in 0..4 {
            counts.push(self.microchips_on(&floor).len());
            counts.push(self.generators_on(&floor).len());
        }
        counts
    }
    fn is_safe(&self) -> bool {
        for floor in 0..4 {
            let generators = self.generators_on(&floor);
            if !self
                .microchips_on(&floor)
                .into_iter()
                .all(|chip| chip.is_safe(&generators))
            {
                return false;
            }
        }
        true
    }
    fn take_elevator(
        &self,
        gens: &[&Generator],
        chips: &[&Microchip],
        next_floor: Floor,
    ) -> Facility {
        let mut next = self.clone();
        next.elevator_floor = next_floor;
        for gen in gens {
            next.generators[gen.element] = next_floor;
        }
        for chip in chips {
            next.microchips[chip.element] = next_floor;
        }
        next
    }
    fn element_count(&self) -> usize {
        self.generators.len()
    }
    fn is_done(&self) -> bool {
        self.elevator_floor == 3
            && self.microchips_on_elevator_floor().len() == self.element_count()
            && self.generators_on_elevator_floor().len() == self.element_count()
    }
    fn steps_to_bring_everything_to_floor_3(&mut self) -> usize {
        let mut states: BinaryHeap<State> = BinaryHeap::new();
        states.push(State {
            facility: self.clone(),
            steps: 0,
        });
        let mut seen = HashSet::new();
        while let Some(curr) = states.pop() {
            // println!("\nPopped state {:?}", curr);
            let state = curr.facility.state();
            if seen.contains(&state) {
                continue;
            } else {
                seen.insert(state);
            }
            if curr.facility.is_done() {
                return curr.steps;
            }
            // We can change one floor at a time
            for next in curr.facility.next_floor_choices() {
                // Bring a generator by itself
                for gen in curr.facility.generators_on_elevator_floor() {
                    if let Some(next) = curr.take_elevator(&[&gen], &[], next) {
                        states.push(next)
                    }

                    // Bring a generator and a microchip
                    for chip in curr.facility.microchips_on_elevator_floor() {
                        if let Some(next) = curr.take_elevator(&[&gen], &[&chip], next) {
                            states.push(next)
                        }
                    }

                    // Bring two generators
                    for gen2 in curr.facility.generators_on_elevator_floor() {
                        if gen2 != gen {
                            if let Some(next) = curr.take_elevator(&[&gen, &gen2], &[], next) {
                                states.push(next)
                            }
                        }
                    }
                }

                // Bring a microchip by itself
                for chip in curr.facility.microchips_on_elevator_floor() {
                    if let Some(next) = curr.take_elevator(&[], &[&chip], next) {
                        states.push(next)
                    }

                    // Bring two microchips
                    for chip2 in curr.facility.microchips_on_elevator_floor() {
                        if chip2 != chip {
                            if let Some(next) = curr.take_elevator(&[], &[&chip, &chip2], next) {
                                states.push(next)
                            }
                        }
                    }
                }
            }
        }
        unreachable!()
    }
    fn next_floor_choices(&self) -> Vec<Floor> {
        match self.elevator_floor {
            0 => vec![1],
            1 => vec![0, 2],
            2 => vec![1, 3],
            3 => vec![2],
            _ => unreachable!(),
        }
    }
    fn generators_on_elevator_floor(&self) -> Vec<Generator> {
        self.generators_on(&self.elevator_floor)
    }
    fn generators_on(&self, other: &Floor) -> Vec<Generator> {
        self.generators
            .iter()
            .enumerate()
            .filter(|(_, floor)| floor == &other)
            .map(|(element, _)| Generator { element })
            .collect()
    }
    fn microchips_on_elevator_floor(&self) -> Vec<Microchip> {
        self.microchips_on(&self.elevator_floor)
    }
    fn microchips_on(&self, other: &Floor) -> Vec<Microchip> {
        self.microchips
            .iter()
            .enumerate()
            .filter(|(_, floor)| floor == &other)
            .map(|(element, _)| Microchip { element })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "\
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn part1_example() {
        let mut facility = Facility::from(parse(EXAMPLE));
        assert_eq!(11, facility.steps_to_bring_everything_to_floor_3());
    }

    #[test]
    fn part1() {
        assert_eq!(37, day11_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(61, day11_part2());
    }
}
