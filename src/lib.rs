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
    choices: Vec<Option<Vec<Vec<Index>>>>,
    sequences: Vec<Option<Vec<Index>>>,
    allowed_strings: Vec<Option<Vec<String>>>,
}

impl From<&[String]> for Resolver {
    fn from(input: &[String]) -> Resolver {
        let mut choices: Vec<Option<Vec<Vec<Index>>>> = vec![None; input.len()];
        let mut sequences: Vec<Option<Vec<Index>>> = vec![None; input.len()];
        let mut allowed_strings: Vec<Option<Vec<String>>> = vec![None; input.len()];
        for rule in input {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    let char = right.chars().nth(1).unwrap();
                    allowed_strings[index] = Some(vec![char.to_string()]);
                } else if let Some((left, right)) = right.split_once(" | ") {
                    let left: Vec<usize> = left
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    let right: Vec<usize> = right
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    choices[index] = Some(vec![left, right]);
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
            allowed_strings,
        }
    }
}
impl Resolver {
    fn is_resolvable(resolved: &[Option<Vec<String>>], indices: &[usize]) -> bool {
        indices.iter().all(|idx| resolved[*idx].is_some())
    }
    fn resolve(mut self) -> Validator {
        while self.allowed_strings[0].is_none() {
            self.resolve_choices();
            self.resolve_sequences()
        }

        let valid_messages = self.allowed_strings[0]
            .as_ref()
            .unwrap()
            .iter()
            .cloned()
            .collect();
        Validator { valid_messages }
    }

    fn resolve_choices(&mut self) {
        for (rule_idx, choice_of_seq) in self.choices.iter_mut().enumerate() {
            // This copy allows the following error:
            // 'cannot borrow `self` as immutable because it is also borrowed as mutable'
            let allowed_strings = self.allowed_strings.clone();
            if let Some(choice) = choice_of_seq {
                if choice
                    .iter()
                    .all(|indices| Resolver::is_resolvable(&allowed_strings, indices))
                {
                    let resolved_choices: Vec<Vec<String>> = choice
                        .iter()
                        .map(|sequence| Resolver::resolve_sequence(&allowed_strings, sequence))
                        .collect();
                    self.allowed_strings[rule_idx] = Some(Resolver::concatenate(resolved_choices));
                    *choice_of_seq = None;
                }
            }
        }
    }

    fn resolve_sequences(&mut self) {
        for (rule_idx, sequence) in self.sequences.iter_mut().enumerate() {
            // This copy allows the following error:
            // 'cannot borrow `self` as immutable because it is also borrowed as mutable'
            let allowed_strings = self.allowed_strings.clone();
            if let Some(seq) = sequence {
                if Resolver::is_resolvable(&self.allowed_strings, seq) {
                    let resolved: Vec<_> = seq
                        .iter()
                        .map(|j| allowed_strings[*j].as_ref().unwrap())
                        .cloned()
                        .collect();
                    let multiplied = Resolver::multiply(resolved.as_slice());
                    self.allowed_strings[rule_idx] = Some(multiplied);
                    *sequence = None;
                }
            }
        }
    }

    fn resolve_sequence(
        allowed_strings: &[Option<Vec<String>>],
        sequence: &[usize],
    ) -> Vec<String> {
        let resolved: Vec<Vec<String>> = sequence
            .iter()
            .map(|i| allowed_strings[*i].clone().unwrap())
            .collect();
        Resolver::multiply(&resolved)
    }
    fn concatenate(mut choices: Vec<Vec<String>>) -> Vec<String> {
        assert_eq!(choices.len(), 2);
        let mut right = choices.remove(1);
        let mut left = choices.remove(0);
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

    let validator = Resolver::from(rules).resolve();
    // println!("Valid messages: {:?}", validator.valid_messages);
    messages
        .iter()
        .filter(|message| validator.is_valid(message))
        .count()
}
