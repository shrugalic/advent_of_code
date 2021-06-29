use std::collections::{HashMap, HashSet};

mod tests;

#[derive(Debug, PartialEq)]
struct Validator {
    valid_messages: HashSet<String>,
}
impl Validator {
    fn is_valid(&self, message: &str) -> bool {
        self.valid_messages.contains(message)
    }
}

#[derive(Debug)]
struct Resolver {
    resolved_rules: Vec<Option<Vec<String>>>,
    unresolved_rules: HashMap<usize, Vec<Vec<usize>>>,
}
impl From<&[String]> for Resolver {
    fn from(rules: &[String]) -> Resolver {
        let mut resolved: Vec<Option<Vec<String>>> = vec![None; rules.len()];
        let mut unresolved: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
        for rule in rules {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    let char = right.chars().nth(1).unwrap();
                    resolved[index] = Some(vec![char.to_string()]);
                } else {
                    let sequences: Vec<Vec<usize>> = right
                        .split(" | ")
                        .map(|seq| {
                            seq.split_ascii_whitespace()
                                .filter_map(|c| c.parse().ok())
                                .collect()
                        })
                        .collect();
                    unresolved.insert(index, sequences);
                }
            } else {
                panic!("Invalid input rule '{}'", rule)
            };
        }
        Resolver {
            resolved_rules: resolved,
            unresolved_rules: unresolved,
        }
    }
}
impl Resolver {
    fn is_resolvable(resolved: &Vec<Option<Vec<String>>>, sequences: &[Vec<usize>]) -> bool {
        sequences
            .iter()
            .flat_map(|indices| indices.iter())
            .all(|idx| resolved[*idx].is_some())
    }
    fn resolve(mut self) -> Validator {
        println!("{:?}", self);
        while !self.unresolved_rules.is_empty() {
            // Moved to avoid borrow-checker error when accessing self.resolved_rules in the loop
            let mut to_resolve = self.unresolved_rules;
            self.unresolved_rules = HashMap::new();
            for (index, sequences) in to_resolve.drain() {
                println!("Trying to resolve ({}, {:?})", index, sequences);
                if Resolver::is_resolvable(&self.resolved_rules, &sequences) {
                    let mut outer_strings: Vec<String> = vec![];
                    for indices in &sequences {
                        let resolved: Vec<&Vec<String>> = indices
                            .iter()
                            .map(|i| self.resolved_rules[*i].as_ref().unwrap())
                            .collect();
                        println!("Resolved = {:?}", resolved);
                        let inner_strings = Resolver::multiply(&resolved);
                        println!("Resolved inner {:?} to {:?}", indices, inner_strings);
                        outer_strings.extend(inner_strings);
                    }
                    println!("Resolved outer {:?} to {:?}", sequences, outer_strings);
                    self.resolved_rules[index] = Some(outer_strings);
                } else {
                    // cannot fully resolve this rule yet
                    println!("Skipping and re-adding ({}, {:?})", index, sequences);
                    self.unresolved_rules.insert(index, sequences);
                }
            }
            println!("{:?}", self);
        }

        let rule_0 = self.resolved_rules[0].as_ref().unwrap();
        // println!("rule_0 = {:?}", rule_0);
        let valid_messages = rule_0.iter().cloned().collect();
        Validator { valid_messages }
    }
    fn multiply(sequences: &[&Vec<String>]) -> Vec<String> {
        let first = sequences[0];
        if sequences.len() == 1 {
            vec![first.join("")]
        } else if sequences.len() == 2 {
            let second = sequences[1];
            // println!("a = {:?}", first);
            // println!("b = {:?}", second);
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
            // println!("c = {:?}\n-----", result);
            result
        } else {
            let second = Resolver::multiply(&sequences[1..]);
            Resolver::multiply(&[first, &second])
        }
    }
    fn join_sequences(sequences: &[&Vec<String>]) -> Vec<String> {
        sequences.iter().map(|sequence| sequence.join("")).collect()
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
