mod tests;

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Validator {
    valid_messages: HashSet<String>,
}
impl Validator {
    fn is_valid(&self, message: &str) -> bool {
        self.valid_messages.contains(message)
    }
}

type Index = usize;
type Choice = (Vec<Index>, Vec<Index>);
type Sequence = Vec<Index>;
type AllowedStrings = Vec<String>;
type Choices = HashMap<Index, Choice>;
type Sequences = HashMap<Index, Sequence>;
type ResolvedStrings = HashMap<Index, AllowedStrings>;

trait Resolvable {
    fn is_resolvable(&self, resolved: &ResolvedStrings) -> bool;
}
impl Resolvable for Choice {
    fn is_resolvable(&self, resolved: &ResolvedStrings) -> bool {
        self.0.iter().all(|i| resolved.contains_key(i))
            && self.1.iter().all(|i| resolved.contains_key(i))
    }
}
impl Resolvable for Sequence {
    fn is_resolvable(&self, resolved: &ResolvedStrings) -> bool {
        self.iter().all(|i| resolved.contains_key(i))
    }
}

struct Resolver;
impl Resolver {
    fn input_to_rules(input: &[String]) -> (Choices, Sequences, ResolvedStrings) {
        let mut choices: Choices = HashMap::new();
        let mut sequences: Sequences = HashMap::new();
        let mut resolved: ResolvedStrings = HashMap::new();

        for rule in input {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    resolved.insert(index, vec![right.chars().nth(1).unwrap().to_string()]);
                } else if let Some((left, right)) = right.split_once(" | ") {
                    choices.insert(
                        index,
                        (
                            left.split_ascii_whitespace()
                                .filter_map(|s| s.parse().ok())
                                .collect(),
                            right
                                .split_ascii_whitespace()
                                .filter_map(|s| s.parse().ok())
                                .collect(),
                        ),
                    );
                } else {
                    sequences.insert(
                        index,
                        right
                            .split_ascii_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect(),
                    );
                }
            } else {
                panic!("Invalid input rule '{}'", rule)
            };
        }
        (choices, sequences, resolved)
    }

    fn resolve(
        mut choices: Choices,
        mut sequences: Sequences,
        mut resolved: ResolvedStrings,
    ) -> Validator {
        while !resolved.contains_key(&0) {
            Resolver::resolve_choices(&mut choices, &mut resolved);
            Resolver::resolve_sequences(&mut sequences, &mut resolved);
        }
        if let Some(allowed_strings) = resolved.get(&0) {
            Validator {
                valid_messages: allowed_strings.iter().cloned().collect(),
            }
        } else {
            panic!("Not solved!")
        }
    }

    fn resolve_choices(choices: &mut Choices, resolved: &mut ResolvedStrings) {
        let mut unresolved: Choices = HashMap::new();
        for (i, choice) in choices.drain() {
            if choice.is_resolvable(&resolved) {
                let (left, right) = choice;
                resolved.insert(
                    i,
                    Resolver::concatenate(
                        Resolver::resolve_sequence(&resolved, &left),
                        Resolver::resolve_sequence(&resolved, &right),
                    ),
                );
            } else {
                unresolved.insert(i, choice);
            }
        }
        *choices = unresolved;
    }

    fn resolve_sequences(sequences: &mut Sequences, resolved: &mut ResolvedStrings) {
        let mut unresolved: Sequences = HashMap::new();
        for (i, sequence) in sequences.drain() {
            if sequence.is_resolvable(&resolved) {
                let multiplied = Resolver::resolve_sequence(&resolved, &sequence);
                resolved.insert(i, multiplied);
            } else {
                unresolved.insert(i, sequence);
            }
        }
        *sequences = unresolved;
    }

    fn resolve_sequence(resolved: &ResolvedStrings, sequence: &[usize]) -> AllowedStrings {
        let resolved_sequence: Vec<AllowedStrings> = sequence
            .iter()
            .filter_map(|i| resolved.get(i))
            .cloned()
            .collect();
        Resolver::generate_allowed_strings(resolved_sequence)
    }
    fn concatenate(mut left: AllowedStrings, mut right: AllowedStrings) -> AllowedStrings {
        left.append(&mut right);
        left
    }
    fn generate_allowed_strings(mut sequences: Vec<AllowedStrings>) -> AllowedStrings {
        let first = sequences.remove(0);
        if sequences.is_empty() {
            first
        } else if sequences.len() == 1 {
            let second = sequences.remove(0);
            first
                .iter()
                .map(|a| {
                    second
                        .clone()
                        .iter()
                        .map(|b| a.clone() + b)
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect()
        } else {
            // len > 1
            let second = Resolver::generate_allowed_strings(sequences);
            Resolver::generate_allowed_strings(vec![first, second])
        }
    }
}

fn number_of_messages_matching_rule_0(input: &[String]) -> usize {
    let mut split = input.split(|line| line.is_empty());
    let (rules, messages) = (split.next().unwrap(), split.next().unwrap());
    // println!("Rules: {:?}", rules);
    // println!("Messages: {:?}", messages);

    let (choices, sequences, resolved) = Resolver::input_to_rules(rules);
    let validator = Resolver::resolve(choices, sequences, resolved);
    // println!("Valid messages: {:?}", validator.valid_messages);
    messages
        .iter()
        .filter(|message| validator.is_valid(message))
        .count()
}
