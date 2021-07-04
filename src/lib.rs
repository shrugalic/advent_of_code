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
impl<T> From<&[T]> for Rules
where
    T: AsRef<str>,
{
    fn from(input: &[T]) -> Self {
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
            if let Some((left, right)) = rule.as_ref().split_once(": ") {
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
                panic!("Invalid input rule '{}'", rule.as_ref())
            };
        }
        Rules { rules }
    }
}

impl Rules {
    fn allow<T>(&self, msg: T) -> bool
    where
        T: AsRef<str>,
    {
        // println!("Rules = {:?}", self.rules);
        let msg_chars: Vec<char> = msg.as_ref().chars().collect();
        if let Some(rule) = self.rules.get(&0) {
            if let Some(remaining) = rule.allows(&msg_chars, self) {
                // println!("\nMatched '{}'\n", msg.as_ref());
                return remaining.iter().any(|rem| rem.is_empty());
            }
        }
        false
    }

    fn get_rule(&self, index: &usize) -> Option<&Rule> {
        self.rules.get(index)
    }

    // fn rule_at_index_allows<'a>(&self, index: &usize, message: &'a [char]) -> Option<&'a [char]> {
    //     self.get_rule(index)
    //         .and_then(|rule| rule.allows(message, self))
    // }
}

impl Rule {
    /// Returns Some(possible_tails) of the message if the head was matched, or None
    fn allows<'a>(&self, msg: &'a [char], rules: &Rules) -> Option<Vec<&'a [char]>> {
        // println!("Rule = {:?}, message = {:?}", self, msg);
        if msg.is_empty() {
            return None;
        }
        match self {
            Rule::Char(c) => {
                if *c == msg[0] {
                    Some(vec![&msg[1..]])
                } else {
                    None
                }
            }
            Rule::Index(index) => rules
                .get_rule(index)
                .and_then(|rule| rule.allows(msg, rules)),
            // Rule::Index(index) => rules.rule_at_index_allows(index, message),
            Rule::Choice(one, two) => {
                let res_one = one.allows(msg, rules);
                let res_two = two.allows(msg, rules);
                match (res_one, res_two) {
                    (Some(mut res1), Some(res2)) => {
                        res1.extend(res2);
                        Some(res1)
                    }
                    (Some(res), None) | (None, Some(res)) => Some(res),
                    (None, None) => None,
                }
            }
            Rule::Single(rule) => rule.allows(msg, rules),
            Rule::Pair(first, second) => first.allows(msg, rules).map(|rems| {
                rems.iter()
                    .filter_map(|&rem| second.allows(&rem, rules))
                    .flatten()
                    .collect()
            }),
            Rule::Triple(first, second, third) => first
                .allows(msg, rules)
                .map(|rems| {
                    rems.iter()
                        .filter_map(|&rem| second.allows(&rem, rules))
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .map(|rems: Vec<&[char]>| {
                    rems.iter()
                        .filter_map(|&rem| third.allows(&rem, rules))
                        .flatten()
                        .collect()
                }),
        }
    }
}

fn alternate_number_of_messages_matching_rule_0<T>(input: &[T]) -> usize
where
    T: AsRef<str>,
{
    let mut split = input.split(|line| line.as_ref().is_empty());
    let (rules, messages) = (split.next().unwrap(), split.next().unwrap());
    // println!("Rules: {:?}", rules);
    // println!("Messages: {:?}", messages);

    let rules = Rules::from(rules);
    // println!("{:?}", rules);

    count_messages_matching_rules(messages, rules)
}

fn count_messages_matching_rules<T>(messages: &[T], rules: Rules) -> usize
where
    T: AsRef<str>,
{
    messages.iter().filter(|m| rules.allow(m)).count()
}
