use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day16.txt");

pub(crate) fn day16_part1() -> usize {
    let valves = Volcano::from(INPUT);
    valves.solve_part_1()
}

pub(crate) fn day16_part2() -> usize {
    let valves = Volcano::from(INPUT);
    valves.solve_part_1()
}

type ValveName = &'static str;
type FlowRate = usize;
type PressureReleased = usize;
type Time = u8;

struct Valve {
    flow_rate: FlowRate,
    reachable_valves: Vec<ValveName>,
}
struct Volcano {
    valves_by_name: HashMap<ValveName, Valve>,
}
type OpenValves = HashSet<ValveName>;
#[derive(Eq, PartialEq, Hash)]
struct State {
    name: ValveName,
    total: PressureReleased,
}
impl Volcano {
    fn solve_part_1(self) -> usize {
        // I would have preferred the states to be a HashSet of a tuple of te current valve,
        // the total pressure released, and the open valves. But the open valves cannot be put
        // into a HashSet, because it's a HashMap itself.
        // Thus they are separated into key and value of a HashMap instead
        let mut states: HashMap<State, OpenValves> = HashMap::new();
        states.insert(
            State {
                name: "AA",
                total: 0,
            },
            HashSet::new(),
        );

        // Simulate every path
        const TIME_LIMIT: Time = 30;
        for time in 0..TIME_LIMIT {
            let mut next_states = HashMap::new();
            for (State { name, mut total }, open) in states.drain() {
                let time = time + 1; // I takes 1 to move or open a valve

                // Move to a connected valve
                for name in self.valves_reachable_from(name) {
                    next_states.insert(State { name, total }, open.clone());
                }

                // Stay here and open valve
                let flow_rate = self.flow_rate_for(name);
                if flow_rate > 0 && !open.contains(name) {
                    let rem_time = TIME_LIMIT - time;
                    total += flow_rate * rem_time as usize;
                    let mut open = open.clone();
                    open.insert(name);
                    next_states.insert(State { name, total }, open.clone());
                }
            }
            states = next_states;
        }
        states
            .keys()
            .map(|&State { total, .. }| total)
            .max()
            .unwrap()
    }
    fn flow_rate_for(&self, name: ValveName) -> FlowRate {
        self.valves_by_name.get(name).unwrap().flow_rate
    }
    fn valves_reachable_from(&self, name: ValveName) -> Vec<ValveName> {
        self.valves_by_name
            .get(name)
            .unwrap()
            .reachable_valves
            .clone()
    }
}

impl From<&'static str> for Volcano {
    fn from(input: &'static str) -> Self {
        let valves_by_name = input
            .lines()
            .map(|line| {
                let (name, rest) = line
                    .strip_prefix("Valve ")
                    .unwrap()
                    .split_once(" has flow rate=")
                    .unwrap();
                let (flow_rate, rest) = rest.split_once(';').unwrap();
                let flow_rate: FlowRate = flow_rate.parse().unwrap();
                let reachable_valves =
                    if let Some(rest) = rest.strip_prefix(" tunnels lead to valves ") {
                        rest.split(", ").collect()
                    } else {
                        vec![rest.strip_prefix(" tunnel leads to valve ").unwrap()]
                    };
                (
                    name,
                    Valve {
                        flow_rate,
                        reachable_valves,
                    },
                )
            })
            .collect();
        Volcano { valves_by_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_example() {
        assert_eq!(1_651, Volcano::from(EXAMPLE).solve_part_1());
    }

    #[test]
    fn part1() {
        assert_eq!(1_488, day16_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(1_707, Volcano::from(EXAMPLE).solve_part_1());
    }

    #[test]
    fn part2() {
        assert_eq!(1, day16_part2());
    }
}
