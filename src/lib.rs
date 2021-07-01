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

#[derive(Clone, Debug, PartialEq)]
enum Rule {
    Undefined,
    Choice((Vec<Index>, Vec<Index>)),
    Sequence(Vec<Index>),
    AllowedStrings(Vec<String>),
}

struct Resolver;
impl Resolver {
    fn input_to_rules(input: &[String]) -> Vec<Rule> {
        let mut rules: Vec<Rule> = vec![Rule::Undefined; input.len()];
        for rule in input {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    let char = right.chars().nth(1).unwrap();
                    rules[index] = Rule::AllowedStrings(vec![char.to_string()]);
                } else if let Some((left, right)) = right.split_once(" | ") {
                    let left: Vec<usize> = left
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    let right: Vec<usize> = right
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    rules[index] = Rule::Choice((left, right))
                } else {
                    let sequence: Vec<usize> = right
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    rules[index] = Rule::Sequence(sequence);
                }
            } else {
                panic!("Invalid input rule '{}'", rule)
            };
        }
        rules
    }
    fn is_resolvable(rules: &HashMap<usize, Vec<String>>, indices: &[usize]) -> bool {
        indices.iter().all(|idx| rules.contains_key(idx))
    }
    fn resolve(mut rules: Vec<Rule>) -> Validator {
        while !matches!(rules[0], Rule::AllowedStrings(_)) {
            Resolver::resolve_choices(&mut rules);
            Resolver::resolve_sequences(&mut rules);
        }
        if let Rule::AllowedStrings(valid_messages) = &rules[0] {
            println!("Solved with rules! 🥳");
            Validator {
                valid_messages: valid_messages.iter().cloned().collect(),
            }
        } else {
            panic!("Not solved!")
        }
    }

    fn resolve_choices(rules: &mut Vec<Rule>) {
        let mut strings_by_idx = Resolver::allowed_strings_by_index(rules);
        for (i, choice_rule) in rules.iter_mut().enumerate() {
            if let Rule::Choice((left, right)) = choice_rule {
                if Resolver::is_resolvable(&strings_by_idx, left)
                    && Resolver::is_resolvable(&strings_by_idx, right)
                {
                    let res_left = Resolver::resolve_sequence(&strings_by_idx, left);
                    let res_right = Resolver::resolve_sequence(&strings_by_idx, right);
                    *choice_rule = Rule::AllowedStrings(Resolver::concatenate_choice(
                        res_left.clone(),
                        res_right.clone(),
                    ));
                    strings_by_idx.insert(i, Resolver::concatenate_choice(res_left, res_right));
                }
            }
        }
    }

    fn allowed_strings_by_index(rules: &mut Vec<Rule>) -> HashMap<usize, Vec<String>> {
        rules
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                if let Rule::AllowedStrings(v) = r {
                    Some((i, v.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn resolve_sequences(rules: &mut Vec<Rule>) {
        let mut strings_by_idx = Resolver::allowed_strings_by_index(rules);
        for (i, seq_rule) in rules.iter_mut().enumerate() {
            if let Rule::Sequence(sequence) = seq_rule {
                if Resolver::is_resolvable(&strings_by_idx, sequence) {
                    let multiplied = Resolver::resolve_sequence(&strings_by_idx, sequence);
                    *seq_rule = Rule::AllowedStrings(multiplied.clone());
                    strings_by_idx.insert(i, multiplied);
                }
            }
        }
    }

    fn resolve_sequence(strings: &HashMap<usize, Vec<String>>, sequence: &[usize]) -> Vec<String> {
        let resolved: Vec<Vec<String>> = sequence
            .iter()
            .map(|i| strings.get(i).unwrap())
            .cloned()
            .collect();
        Resolver::generate_allowed_strings(&resolved)
    }
    fn concatenate_choice(mut left: Vec<String>, mut right: Vec<String>) -> Vec<String> {
        left.append(&mut right);
        left
    }
    fn generate_allowed_strings(sequences: &[Vec<String>]) -> Vec<String> {
        let first = &sequences[0];
        if sequences.len() == 1 {
            first.clone()
        } else if sequences.len() == 2 {
            let second = &sequences[1];
            let result = first
                .iter()
                .map(|a| {
                    second
                        .clone()
                        .iter()
                        .map(|b| a.clone() + b)
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            result
        } else {
            let second = Resolver::generate_allowed_strings(&sequences[1..]);
            Resolver::generate_allowed_strings(&[first.to_vec(), second])
        }
    }
}

fn number_of_messages_matching_rule_0(input: &[String]) -> usize {
    let mut split = input.split(|line| line.is_empty());
    let (rules, messages) = (split.next().unwrap(), split.next().unwrap());
    // println!("Rules: {:?}", rules);
    // println!("Messages: {:?}", messages);

    let rules = Resolver::input_to_rules(rules);
    let validator = Resolver::resolve(rules);
    // println!("Valid messages: {:?}", validator.valid_messages);
    messages
        .iter()
        .filter(|message| validator.is_valid(message))
        .count()
}
