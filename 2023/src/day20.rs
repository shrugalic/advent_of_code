use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

use ModuleType::*;
use Pulse::*;

const INPUT: &str = include_str!("../input/day20.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";

fn solve_part1(input: &'static str) -> usize {
    let mut modules_by_name = parse_modules_by_name(input);

    let mut signal_counts = PulseCounts::default();
    for _button_press_count in 1..=1000 {
        let mut inputs = vec![OutMessage::from_button_press()];
        signal_counts.counts_by_type[Low as usize] += 1;
        while !inputs.is_empty() {
            inputs = process(inputs, &mut modules_by_name);
        }
    }
    for module in modules_by_name.values() {
        signal_counts += module.stats;
    }
    signal_counts.counts_by_type[Low as usize] * signal_counts.counts_by_type[High as usize]
}

fn solve_part2(input: &'static str) -> usize {
    // The "rx" module did not produce a low output within a reasonable time frame,
    // so I analyzed the modules. There's a single Conjunction module leading to "rx":
    // &vf -> rx

    // Got get "rx" low, all "vf"'s inputs must be high, at the same time.
    // "vf"'s inputs are:
    // &pm -> vf
    // &mk -> vf
    // &pk -> vf
    // &hf -> vf

    // Each of them produces a "high" pulse periodically (around ~4k button presses),
    // but each of their periods is different and all are prime numbers,
    // so all of them are high only after lcm(3881,3889,4013,4021) button presses,
    // which in this case (periods are prime numbers) is equal to the product of their periods

    let signals: Vec<_> = ["pm", "mk", "pk", "hf"]
        .into_iter()
        .map(|name| InMessage::new(name, High))
        .collect();
    let mut periods = vec![None; 4];

    let mut modules_by_name = parse_modules_by_name(input);
    for button_press_count in 1usize..5000 {
        let mut inputs = vec![OutMessage::from_button_press()];
        while !inputs.is_empty() {
            inputs = process(inputs, &mut modules_by_name);
            inputs.iter().for_each(|out| {
                for (i, wanted) in signals.iter().enumerate() {
                    if wanted == &out.signal && periods[i].is_none() {
                        periods[i] = Some(button_press_count);
                    }
                }
            });
            if periods.iter().all(|p| p.is_some()) {
                return periods.into_iter().flatten().product::<usize>();
            }
        }
    }
    unreachable!()
}

fn parse_modules_by_name(input: &'static str) -> HashMap<ModuleName, Module> {
    parse_module_types(input)
        .into_iter()
        .map(|mod_type| (mod_type.name(), Module::from(mod_type)))
        .collect()
}

fn process(
    mut inputs: Vec<OutMessage>,
    modules_by_name: &mut HashMap<ModuleName, Module>,
) -> Vec<OutMessage> {
    inputs
        .drain(..)
        .flat_map(|output| {
            if let Some(module) = modules_by_name.get_mut(&output.destination) {
                module.process(output.signal)
            } else {
                // Ignore "FAKE" destination from "rx" and "output" modules
                vec![]
            }
        })
        .collect()
}

type ModuleName = &'static str;

#[derive(Default, Debug, PartialEq)]
enum State {
    On,
    #[default]
    Off,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq)]
struct InMessage {
    source: ModuleName,
    pulse: Pulse,
}

#[derive(Debug)]
struct OutMessage {
    signal: InMessage,
    destination: ModuleName,
}

#[derive(Debug, PartialEq)]
enum ModuleType {
    Broadcaster(Vec<ModuleName>),
    FlipFlop(ModuleName, State, Vec<ModuleName>),
    Conjunction(ModuleName, HashMap<ModuleName, Pulse>, Vec<ModuleName>),
    Untyped(ModuleName),
}

#[derive(Default, Clone, Copy)]
struct PulseCounts {
    // Index is Pulse as usize
    counts_by_type: [usize; 2],
}

struct Module {
    mod_type: ModuleType,
    stats: PulseCounts,
}

impl State {
    fn toggle(&mut self) {
        *self = match self {
            State::On => State::Off,
            State::Off => State::On,
        };
    }
}

impl InMessage {
    fn new(source: &'static str, pulse: Pulse) -> Self {
        InMessage { source, pulse }
    }
}

impl OutMessage {
    fn new(source: &'static str, pulse: Pulse, destination: &'static str) -> Self {
        OutMessage {
            signal: InMessage::new(source, pulse),
            destination,
        }
    }
    fn from_button_press() -> OutMessage {
        OutMessage {
            signal: InMessage {
                pulse: Low,
                source: BUTTON,
            },
            destination: BROADCASTER,
        }
    }
}

impl ModuleType {
    fn name(&self) -> ModuleName {
        match self {
            Broadcaster(_) => BROADCASTER,
            FlipFlop(name, ..) | Conjunction(name, ..) | Untyped(name) => name,
        }
    }
    fn outputs(&self) -> Vec<ModuleName> {
        match self {
            Broadcaster(outputs) | FlipFlop(.., outputs) | Conjunction(.., outputs) => {
                outputs.to_vec()
            }
            Untyped(_) => vec![],
        }
    }
    fn process(&mut self, received: InMessage) -> Vec<OutMessage> {
        match self {
            Broadcaster(outputs) => outputs
                .iter()
                .map(|output| OutMessage::new(BROADCASTER, received.pulse, output))
                .collect(),
            FlipFlop(name, state, outputs) => match received.pulse {
                High => vec![],
                Low => {
                    let pulse = match state {
                        State::On => Low,
                        State::Off => High,
                    };
                    state.toggle();
                    outputs
                        .iter()
                        .map(|output| OutMessage::new(name, pulse, output))
                        .collect()
                }
            },
            Conjunction(name, inputs, outputs) => {
                *inputs
                    .get_mut(&received.source)
                    .expect("conjunction should know its inputs") = received.pulse;
                let pulse = if inputs.values().all(|p| p == &High) {
                    Low
                } else {
                    High
                };
                outputs
                    .iter()
                    .map(|output| OutMessage::new(name, pulse, output))
                    .collect()
            }
            Untyped(name) => {
                vec![OutMessage::new(name, received.pulse, "FAKE")]
            }
        }
    }
}

impl Module {
    fn process(&mut self, signal: InMessage) -> Vec<OutMessage> {
        let messages = self.mod_type.process(signal);
        for out in &messages {
            if out.signal.source != "rx" && out.signal.source != "output" {
                self.stats.counts_by_type[out.signal.pulse as usize] += 1
            }
        }
        messages
    }
}

impl AddAssign for PulseCounts {
    fn add_assign(&mut self, rhs: Self) {
        self.counts_by_type[0] += rhs.counts_by_type[0];
        self.counts_by_type[1] += rhs.counts_by_type[1];
    }
}

impl Add for PulseCounts {
    type Output = PulseCounts;

    fn add(self, rhs: Self) -> Self::Output {
        let mut counts = self;
        counts += rhs;
        counts
    }
}

fn parse_module_types(input: &'static str) -> Vec<ModuleType> {
    let mut modules: Vec<_> = input.trim().lines().map(ModuleType::from).collect();
    // Initialize conjunction inputs
    let mut inputs_by_conjunction_name: HashMap<ModuleName, Vec<ModuleName>> = HashMap::new();
    modules.iter().for_each(|module| {
        module.outputs().iter().for_each(|output| {
            inputs_by_conjunction_name
                .entry(output)
                .or_default()
                .push(module.name());
        });
    });
    modules.iter_mut().for_each(|module| {
        if let Conjunction(name, inputs, ..) = module {
            *inputs = inputs_by_conjunction_name
                .remove(name)
                .unwrap_or_else(|| panic!("every conjunction has inputs, missing {name}"))
                .into_iter()
                .map(|input| (input, Low))
                .collect();
        }
    });

    // Create Untyped modules
    let output_names: HashSet<ModuleName> =
        modules.iter().flat_map(|module| module.outputs()).collect();
    let module_names: HashSet<ModuleName> = modules.iter().map(|module| module.name()).collect();
    output_names.difference(&module_names).for_each(|name| {
        modules.push(Untyped(name));
    });
    modules
}

impl From<ModuleType> for Module {
    fn from(mod_type: ModuleType) -> Self {
        Module {
            mod_type,
            stats: PulseCounts::default(),
        }
    }
}
impl From<&'static str> for ModuleType {
    fn from(value: &'static str) -> Self {
        let (type_and_name, outputs) = value.split_once(" -> ").expect("should have ' -> '");
        let outputs: Vec<_> = outputs.split(", ").collect();
        match type_and_name {
            "broadcaster" => Broadcaster(outputs),
            name if name.starts_with('%') => FlipFlop(&name[1..], State::Off, outputs),
            name if name.starts_with('&') => Conjunction(&name[1..], HashMap::new(), outputs),
            _ => unreachable!("Untyped {type_and_name}"),
        }
    }
}

impl Display for OutMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-> {}", self.signal, self.destination)
    }
}
impl Display for InMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}", self.source, self.pulse)
    }
}
impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Low => "low",
                High => "high",
            }
        )
    }
}
impl Display for ModuleType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Broadcaster(outputs) => format!("broadcaster -> {}", outputs.join(", ")),
                FlipFlop(name, _, outputs) => format!("{name} -> {}", outputs.join(", ")),
                Conjunction(name, _, outputs) => format!("{name} -> {}", outputs.join(", ")),
                Untyped(name) => name.to_string(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
    const EXAMPLE_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn test_parsing_of_example1() {
        assert_eq!(
            parse_module_types(EXAMPLE_1),
            vec![
                Broadcaster(vec!["a", "b", "c"]),
                FlipFlop("a", State::Off, vec!["b"]),
                FlipFlop("b", State::Off, vec!["c"]),
                FlipFlop("c", State::Off, vec!["inv"]),
                Conjunction("inv", [("c", Low)].into_iter().collect(), vec!["a"]),
            ]
        );
    }

    #[test]
    fn test_parsing_of_example2() {
        assert_eq!(
            parse_module_types(EXAMPLE_2),
            vec![
                Broadcaster(vec!["a"]),
                FlipFlop("a", State::Off, vec!["inv", "con"]),
                Conjunction("inv", [("a", Low)].into_iter().collect(), vec!["b"]),
                FlipFlop("b", State::Off, vec!["con"]),
                Conjunction(
                    "con",
                    [("a", Low), ("b", Low)].into_iter().collect(),
                    vec!["output"]
                ),
                Untyped("output"),
            ]
        );
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(8000 * 4000, solve_part1(EXAMPLE_1));
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(4250 * 2750, solve_part1(EXAMPLE_2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(16_510 * 41_204 /* 680_278_040 */, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            3881 * 3889 * 4013 * 4021, /* 243_548_140_870_057 */
            solve_part2(INPUT)
        );
    }
}
