use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day16.txt");

pub(crate) fn day16_part1() -> usize {
    let valves = Volcano::from(INPUT);
    valves.solve_part_1()
}

pub(crate) fn day16_part2() -> usize {
    let valves = Volcano::from(INPUT);
    valves.solve_part_2()
}

type ValveName = &'static str;
type FlowRate = usize;
type PressureReleased = usize;
type Time = u8;

type OpenValves = HashSet<ValveName>;

#[derive(Eq, PartialEq, Hash)]
struct Part1State {
    name: ValveName,
    total: PressureReleased,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Part2State {
    name1: ValveName,
    name2: ValveName,
    total: PressureReleased,
}

struct Valve {
    flow_rate: FlowRate,
    reachable_valves: Vec<ValveName>,
}

struct Volcano {
    valves_by_name: HashMap<ValveName, Valve>,
}
impl Volcano {
    fn solve_part_1(self) -> usize {
        let mut states: HashMap<Part1State, OpenValves> = HashMap::new();
        states.insert(
            Part1State {
                name: "AA",
                total: 0,
            },
            HashSet::new(),
        );

        // Simulate every path
        const TIME_LIMIT: Time = 30;
        for time in 0..TIME_LIMIT {
            let mut next_states = HashMap::new();
            for (Part1State { name, mut total }, open) in states.drain() {
                let time = time + 1; // It takes 1 to move or open a valve

                // Move to a connected valve
                for name in self.valves_reachable_from(name) {
                    next_states.insert(Part1State { name, total }, open.clone());
                }

                // Stay here and open valve
                let flow_rate = self.flow_rate_for(name);
                if flow_rate > 0 && !open.contains(name) {
                    let rem_time = TIME_LIMIT - time;
                    total += flow_rate * rem_time as usize;
                    let mut open = open.clone();
                    open.insert(name);
                    next_states.insert(Part1State { name, total }, open.clone());
                }
            }
            states = next_states;
        }
        states
            .keys()
            .map(|&Part1State { total, .. }| total)
            .max()
            .unwrap()
    }
    fn solve_part_2(self) -> usize {
        let mut states: HashMap<Part2State, OpenValves> = HashMap::new();
        states.insert(
            Part2State {
                name1: "AA",
                name2: "AA",
                total: 0,
            },
            HashSet::new(),
        );

        let non_zero_valve_count = self
            .valves_by_name
            .values()
            .filter(|Valve { flow_rate, .. }| flow_rate > &0)
            .count();
        // println!(
        //     "{non_zero_valve_count}/{} valves have non-0 flow-rate.",
        //     self.valves_by_name.len()
        // );

        // Simulate every path
        const TIME_LIMIT: Time = 26;
        for time in 0..TIME_LIMIT {
            let mut next_states: HashMap<Part2State, OpenValves> = HashMap::new();
            for (
                _state @ Part2State {
                    name1,
                    name2,
                    total,
                },
                open,
            ) in states.drain()
            {
                // if open.len() == non_zero_valve_count {
                //     next_states.insert(state, open);
                //     println!("{time} skipped {name1} and {name2} because all valves open");
                //     continue;
                // }

                let time = time + 1; // It takes 1 to move or open a valve
                let rem_time = TIME_LIMIT - time;

                // println!("{time}: {name1} {name2} {total}");

                let mut options1: Vec<_> = self
                    // a) Move to a connected valve
                    .valves_reachable_from(name1)
                    .into_iter()
                    .map(|name1| (name1, None /* Opened */))
                    .chain({
                        // Or b) Stay here and open valve
                        let flow_rate = self.flow_rate_for(name1);
                        if flow_rate > 0 && !open.contains(name1) {
                            Some((name1, Some(flow_rate)))
                        } else {
                            None
                        }
                    })
                    .collect();
                // println!("- Options 1 ({name1}): {:?};\t{:?}", options1, open);

                let options2: Vec<_> = self
                    // a) Move to a connected valve
                    .valves_reachable_from(name2)
                    .into_iter()
                    .map(|name2| (name2, None /* Opened */))
                    .chain({
                        // Or b) Stay here and open valve
                        let flow_rate = self.flow_rate_for(name2);
                        if flow_rate > 0 && !open.contains(name2) {
                            Some((name2, Some(flow_rate)))
                        } else {
                            None
                        }
                    })
                    .collect();
                // println!("- Options 2 ({name2}): {:?};\t{:?}", options2, open);

                while let Some((name1, rate1)) = options1.pop() {
                    for (name2, rate2) in &options2 {
                        let mut diff = 0;
                        let mut currently_open = open.clone();
                        match (rate1, rate2) {
                            (Some(rate1), Some(rate2)) => {
                                if &name1 == name2 {
                                    // Both opening the same node is invalid.
                                    // One of them opening it should be handled on another path
                                    continue;
                                }
                                assert_ne!(&name1, name2);
                                assert_eq!(rate1, self.flow_rate_for(name1));
                                assert_eq!(rate2, &self.flow_rate_for(name2));
                                diff = (rate1 + *rate2) * rem_time as usize;
                                assert!(currently_open.insert(name1));
                                assert!(currently_open.insert(*name2));
                            }
                            (Some(rate1), None) => {
                                diff = rate1 * rem_time as usize;
                                assert_eq!(rate1, self.flow_rate_for(name1));
                                assert!(currently_open.insert(name1));
                            }
                            (None, Some(rate2)) => {
                                assert_eq!(rate2, &self.flow_rate_for(name2));
                                diff = rate2 * rem_time as usize;
                                assert!(currently_open.insert(*name2));
                            }
                            (None, None) => {}
                        }

                        // Sort names to reduce problem space
                        let total = total + diff;
                        // if diff > 0 {
                        //     println!(
                        //         "{time}: added {diff} from {name1} {:?} or {name2} {:?}: {:?}",
                        //         rate1, rate2, currently_open
                        //     );
                        // }
                        if total > 2106 {
                            /* example MAX */
                            panic!(
                                "{time}: added {diff} for {total} from {name1} {:?} or {name2} {:?}: {:?}",
                                rate1, rate2, currently_open
                            );
                        }
                        next_states.insert(
                            Part2State {
                                name1: if name1 < name2 { name1 } else { name2 },
                                name2: if name1 >= name2 { name1 } else { name2 },
                                total,
                            },
                            currently_open,
                        );
                    }
                }
            }
            states = next_states;
            if states
                .iter()
                .all(|(_, open)| open.len() == non_zero_valve_count)
            {
                println!("All valves open for all configurations at time {time}");
                break;
            }
        }
        states
            .keys()
            .map(|&Part2State { total, .. }| total)
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
        assert_eq!(1_707, Volcano::from(EXAMPLE).solve_part_2());
    }

    #[test]
    fn part2() {
        assert_eq!(2_111, day16_part2());
    }
}
