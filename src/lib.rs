mod tests;

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Validator {
    allowed_strings: HashSet<String>,
}
impl Validator {
    fn is_valid(&self, message: &str) -> bool {
        self.allowed_strings.contains(message)
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

struct Resolver {
    choices: Choices,
    sequences: Sequences,
    resolved: ResolvedStrings,
}
impl Resolver {
    fn from(input: &[String]) -> Resolver {
        let mut choices: Choices = HashMap::new();
        let mut sequences: Sequences = HashMap::new();
        let mut resolved: ResolvedStrings = HashMap::new();
        let to_usize_vec = |s: &str| -> Vec<usize> {
            s.split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        };
        for rule in input {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    resolved.insert(index, vec![right.chars().nth(1).unwrap().to_string()]);
                } else if let Some((left, right)) = right.split_once(" | ") {
                    choices.insert(index, (to_usize_vec(left), to_usize_vec(right)));
                } else {
                    sequences.insert(index, to_usize_vec(right));
                }
            } else {
                panic!("Invalid input rule '{}'", rule)
            };
        }
        Resolver {
            choices,
            sequences,
            resolved,
        }
    }

    fn resolve(mut self) -> Validator {
        while !self.resolved.contains_key(&0) {
            self.resolve_choices();
            self.resolve_sequences();
        }
        if let Some(allowed_strings) = self.resolved.get(&0) {
            let allowed_strings = allowed_strings.iter().cloned().collect();
            Validator { allowed_strings }
        } else {
            panic!("Not solved!")
        }
    }

    fn resolve_choices(&mut self) {
        let mut unresolved: Choices = HashMap::new();
        for (i, choice) in self.choices.drain() {
            if choice.is_resolvable(&self.resolved) {
                let (left, right) = choice;
                self.resolved.insert(
                    i,
                    Resolver::concatenate(
                        Resolver::resolve_sequence(&self.resolved, &left),
                        Resolver::resolve_sequence(&self.resolved, &right),
                    ),
                );
            } else {
                unresolved.insert(i, choice);
            }
        }
        self.choices = unresolved;
    }

    fn resolve_sequences(&mut self) {
        let mut unresolved: Sequences = HashMap::new();
        for (i, sequence) in self.sequences.drain() {
            if sequence.is_resolvable(&self.resolved) {
                let multiplied = Resolver::resolve_sequence(&self.resolved, &sequence);
                self.resolved.insert(i, multiplied);
            } else {
                unresolved.insert(i, sequence);
            }
        }
        self.sequences = unresolved;
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

    let resolver = Resolver::from(rules);
    let validator = resolver.resolve();
    // println!("Valid messages: {:?}", validator.valid_messages);
    messages
        .iter()
        .filter(|message| validator.is_valid(message))
        .count()
}

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Index(usize),
    Choice(Box<Rule>, Box<Rule>),
    // sequences
    Single(Box<Rule>),
    Pair(Box<Rule>, Box<Rule>),
    Triple(Box<Rule>, Box<Rule>, Box<Rule>),
}
#[derive(Debug)]
struct Rules {
    rules: HashMap<usize, Rule>,
}
impl From<&[String]> for Rules {
    fn from(input: &[String]) -> Self {
        let mut rules = HashMap::new();
        let to_rule = |s: &str| -> Rule {
            let v: Vec<Rule> = s
                .split_ascii_whitespace()
                .filter_map(|s| s.parse().ok())
                .map(Rule::Index)
                .collect();
            match v.len() {
                1 => Rule::Single(Box::new(v[0].clone())),
                2 => Rule::Pair(Box::new(v[0].clone()), Box::new(v[1].clone())),
                3 => Rule::Triple(
                    Box::new(v[0].clone()),
                    Box::new(v[1].clone()),
                    Box::new(v[2].clone()),
                ),
                _ => panic!("Unsupported rules {:?}", v),
            }
        };
        for rule in input {
            if let Some((left, right)) = rule.split_once(": ") {
                let index: usize = left.parse().unwrap();
                if right.starts_with('\"') && right.ends_with('\"') {
                    rules.insert(index, Rule::Char(right.chars().nth(1).unwrap()));
                } else if let Some((left, right)) = right.split_once(" | ") {
                    rules.insert(
                        index,
                        Rule::Choice(Box::new(to_rule(left)), Box::new(to_rule(right))),
                    );
                } else {
                    rules.insert(index, to_rule(right));
                }
            } else {
                panic!("Invalid input rule '{}'", rule)
            };
        }
        Rules { rules }
    }
}

impl Rules {
    fn allow(&self, message: &String) -> bool {
        if let Some(rule) = self.rules.get(&0) {
            rule.allows(message, self)
        } else {
            false
        }
    }
}

impl Rule {
    fn allows(&self, message: &String, rules: &Rules) -> bool {
        // TODO start from rule 0 and see if the strings match
        false
    }
}

fn alternate_number_of_messages_matching_rule_0(input: &[String]) -> usize {
    let mut split = input.split(|line| line.is_empty());
    let (rules, messages) = (split.next().unwrap(), split.next().unwrap());
    // println!("Rules: {:?}", rules);
    // println!("Messages: {:?}", messages);

    let rules = Rules::from(rules);
    println!("{:?}", rules);

    messages.iter().filter(|m| rules.allow(m)).count()
}
