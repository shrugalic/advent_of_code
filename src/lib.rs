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
enum RuleType {
    Choice((Vec<Index>, Vec<Index>)),
    Sequence(Vec<Index>),
    AllowedStrings(Vec<String>),
}
struct Rule {
    index: usize,
    rule: RuleType,
}
impl Rule {
    fn is_resolvable(&self, resolved: &HashMap<usize, Vec<String>>) -> bool {
        match &self.rule {
            RuleType::Choice((left, right)) => {
                left.iter().all(|i| resolved.contains_key(i))
                    && right.iter().all(|i| resolved.contains_key(i))
            }
            RuleType::Sequence(seq) => seq.iter().all(|i| resolved.contains_key(i)),
            RuleType::AllowedStrings(_) => true,
        }
    }
}

struct Resolver;
impl Resolver {
    fn input_to_rules(input: &[String]) -> Vec<Rule> {
        let mut rules: Vec<Rule> = vec![];
        for rule in input {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    let char = right.chars().nth(1).unwrap();
                    rules.push(Rule {
                        index,
                        rule: RuleType::AllowedStrings(vec![char.to_string()]),
                    });
                } else if let Some((left, right)) = right.split_once(" | ") {
                    let left: Vec<usize> = left
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    let right: Vec<usize> = right
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    rules.push(Rule {
                        index,
                        rule: RuleType::Choice((left, right)),
                    });
                } else {
                    let sequence: Vec<usize> = right
                        .split_ascii_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    rules.push(Rule {
                        index,
                        rule: RuleType::Sequence(sequence),
                    });
                }
            } else {
                panic!("Invalid input rule '{}'", rule)
            };
        }
        rules
    }

    fn resolve(mut rules: Vec<Rule>) -> Validator {
        while !rules
            .iter()
            .any(|r| r.index == 0 && matches!(r.rule, RuleType::AllowedStrings(_)))
        {
            Resolver::resolve_choices(&mut rules);
            Resolver::resolve_sequences(&mut rules);
        }
        if let Some(Rule {
            index: _,
            rule: RuleType::AllowedStrings(valid_messages),
        }) = &rules.iter().find(|r| r.index == 0)
        {
            Validator {
                valid_messages: valid_messages.iter().cloned().collect(),
            }
        } else {
            panic!("Not solved!")
        }
    }

    fn resolve_choices(rules: &mut Vec<Rule>) {
        let strings_by_idx = Resolver::strings_by_idx(rules);
        for rule in rules.iter_mut().filter(|r| {
            matches!(r.rule, RuleType::Choice((_, _))) && r.is_resolvable(&strings_by_idx)
        }) {
            if let Rule {
                index: _,
                rule: RuleType::Choice((left, right)),
            } = rule
            {
                rule.rule = RuleType::AllowedStrings(Resolver::concatenate_choice(
                    Resolver::resolve_sequence(&strings_by_idx, left).clone(),
                    Resolver::resolve_sequence(&strings_by_idx, right).clone(),
                ));
            }
        }
    }

    fn strings_by_idx(rules: &mut Vec<Rule>) -> HashMap<usize, Vec<String>> {
        rules
            .iter()
            .filter_map(|r| match &r.rule {
                RuleType::AllowedStrings(s) => Some((r.index, s.clone())),
                _ => None,
            })
            // .cloned()
            .collect()
    }

    fn resolve_sequences(rules: &mut Vec<Rule>) {
        let strings_by_idx = Resolver::strings_by_idx(rules);
        for rule in rules
            .iter_mut()
            .filter(|r| matches!(r.rule, RuleType::Sequence(_)) && r.is_resolvable(&strings_by_idx))
        {
            if let Rule {
                index: _,
                rule: RuleType::Sequence(sequence),
            } = rule
            {
                let multiplied = Resolver::resolve_sequence(&strings_by_idx, sequence);
                rule.rule = RuleType::AllowedStrings(multiplied);
            }
        }
    }

    fn resolve_sequence(
        strings_by_idx: &HashMap<usize, Vec<String>>,
        sequence: &[usize],
    ) -> Vec<String> {
        let resolved: Vec<Vec<String>> = sequence
            .iter()
            .filter_map(|i| strings_by_idx.get(i).clone())
            .cloned()
            .collect();
        Resolver::generate_strings_by_idx(resolved)
    }
    fn concatenate_choice(mut left: Vec<String>, mut right: Vec<String>) -> Vec<String> {
        left.append(&mut right);
        left
    }
    fn generate_strings_by_idx(mut sequences: Vec<Vec<String>>) -> Vec<String> {
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
            let second = Resolver::generate_strings_by_idx(sequences);
            Resolver::generate_strings_by_idx(vec![first, second])
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
