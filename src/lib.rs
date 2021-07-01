mod tests;

use std::collections::HashSet;

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

#[derive(Debug)]
struct Resolver {
    choices: Vec<Option<(Vec<Index>, Vec<Index>)>>,
    sequences: Vec<Option<Vec<Index>>>,
    allowed: Vec<Option<Vec<String>>>,
}

impl From<&[String]> for Resolver {
    fn from(input: &[String]) -> Resolver {
        let mut choices: Vec<Option<(Vec<Index>, Vec<Index>)>> = vec![None; input.len()];
        let mut sequences: Vec<Option<Vec<Index>>> = vec![None; input.len()];
        let mut allowed: Vec<Option<Vec<String>>> = vec![None; input.len()];
        for rule in input {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    let char = right.chars().nth(1).unwrap();
                    allowed[index] = Some(vec![char.to_string()]);
                } else if let Some((left, right)) = right.split_once(" | ") {
                    let left: Vec<usize> = left
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    let right: Vec<usize> = right
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    choices[index] = Some((left, right));
                } else {
                    let sequence: Vec<usize> = right
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    sequences[index] = Some(sequence);
                }
            } else {
                panic!("Invalid input rule '{}'", rule)
            };
        }
        Resolver {
            choices,
            sequences,
            allowed,
        }
    }
}
impl Resolver {
    fn is_resolvable(resolved: &[Option<Vec<String>>], indices: &[usize]) -> bool {
        indices.iter().all(|idx| resolved[*idx].is_some())
    }
    fn resolve(
        mut choices: Vec<Option<(Vec<Index>, Vec<Index>)>>,
        mut sequences: Vec<Option<Vec<Index>>>,
        mut allowed: Vec<Option<Vec<String>>>,
    ) -> Validator {
        while allowed[0].is_none() {
            Resolver::resolve_choices(&mut choices, &mut allowed);
            Resolver::resolve_sequences(&mut sequences, &mut allowed);
        }

        let valid_messages = allowed[0].as_ref().unwrap().iter().cloned().collect();
        Validator { valid_messages }
    }

    fn resolve_choices(
        choices: &mut Vec<Option<(Vec<Index>, Vec<Index>)>>,
        allowed: &mut Vec<Option<Vec<String>>>,
    ) {
        for (rule_idx, choice_of_seq) in choices.iter_mut().enumerate() {
            if let Some((left, right)) = choice_of_seq {
                if Resolver::is_resolvable(&allowed, left)
                    && Resolver::is_resolvable(&allowed, right)
                {
                    let res_left = Resolver::resolve_sequence(&allowed, left);
                    let res_right = Resolver::resolve_sequence(&allowed, right);
                    allowed[rule_idx] = Some(Resolver::concatenate_choice(res_left, res_right));
                    *choice_of_seq = None;
                }
            }
        }
    }

    fn resolve_sequences(
        sequences: &mut Vec<Option<Vec<Index>>>,
        allowed: &mut Vec<Option<Vec<String>>>,
    ) {
        for (rule_idx, sequence) in sequences.iter_mut().enumerate() {
            if let Some(seq) = sequence {
                if Resolver::is_resolvable(&allowed, seq) {
                    let resolved: Vec<_> = seq
                        .iter()
                        .map(|j| allowed[*j].as_ref().unwrap())
                        .cloned()
                        .collect();
                    let multiplied = Resolver::multiply(resolved.as_slice());
                    allowed[rule_idx] = Some(multiplied);
                    *sequence = None;
                }
            }
        }
    }

    fn resolve_sequence(allowed: &[Option<Vec<String>>], sequence: &[usize]) -> Vec<String> {
        let resolved: Vec<Vec<String>> = sequence
            .iter()
            .map(|i| allowed[*i].clone().unwrap())
            .collect();
        Resolver::multiply(&resolved)
    }
    fn concatenate_choice(mut left: Vec<String>, mut right: Vec<String>) -> Vec<String> {
        left.append(&mut right);
        left
    }
    fn multiply(sequences: &[Vec<String>]) -> Vec<String> {
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
            let second = Resolver::multiply(&sequences[1..]);
            Resolver::multiply(&[first.to_vec(), second])
        }
    }
}

fn number_of_messages_matching_rule_0(input: &[String]) -> usize {
    let mut split = input.split(|line| line.is_empty());
    let (rules, messages) = (split.next().unwrap(), split.next().unwrap());
    // println!("Rules: {:?}", rules);
    // println!("Messages: {:?}", messages);

    let r = Resolver::from(rules);
    let validator = Resolver::resolve(r.choices, r.sequences, r.allowed);
    // println!("Valid messages: {:?}", validator.valid_messages);
    messages
        .iter()
        .filter(|message| validator.is_valid(message))
        .count()
}
