use std::collections::HashSet;

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

type Index = usize;

#[derive(Debug)]
struct Resolver {
    choices: Vec<Option<(Vec<Index>, Vec<Index>)>>,
    sequences: Vec<Option<Vec<Index>>>,
    allowed_strings: Vec<Option<Vec<String>>>,
}

impl From<&[String]> for Resolver {
    fn from(input: &[String]) -> Resolver {
        let mut choices: Vec<Option<(Vec<Index>, Vec<Index>)>> = vec![None; input.len()];
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
            allowed_strings,
        }
    }
}
impl Resolver {
    fn is_resolvable(resolved: &[Option<Vec<String>>], indices: &[usize]) -> bool {
        indices.iter().all(|idx| resolved[*idx].is_some())
    }
    fn resolve(mut self) -> Validator {
        println!("{:?}", self);

        let mut counter = 0;
        let mut resolved_something = true;
        while (self.choices.iter().any(|choice| choice.is_some())
            || self.sequences.iter().any(|sequence| sequence.is_some()))
            && self.allowed_strings[0].is_none()
            && resolved_something
        {
            resolved_something = false;
            let mut allowed_strings = self.allowed_strings.clone();
            for (i, choice) in self.choices.iter_mut().enumerate() {
                if let Some((left, right)) = choice {
                    if Resolver::is_resolvable(&self.allowed_strings, left)
                        && Resolver::is_resolvable(&self.allowed_strings, right)
                    {
                        let left: Vec<Vec<String>> = left
                            .iter()
                            .map(|j| allowed_strings[*j].as_ref().unwrap())
                            .cloned()
                            .collect();
                        let right: Vec<Vec<String>> = right
                            .iter()
                            .map(|j| allowed_strings[*j].as_ref().unwrap())
                            .cloned()
                            .collect();
                        self.allowed_strings[i] =
                            Some(Resolver::create_allowed_strings(&left, &right));
                        *choice = None;
                        resolved_something = true;
                    } else {
                        println!("Choice {:?} not resolvable yet", choice);
                    }
                }
            }
            // println!("{:?}", self);
            // }
            allowed_strings = self.allowed_strings.clone();
            // while  {
            for (i, sequence) in self.sequences.iter_mut().enumerate() {
                if let Some(seq) = sequence {
                    if Resolver::is_resolvable(&self.allowed_strings, seq) {
                        let resolved: Vec<_> = seq
                            .iter()
                            .map(|j| allowed_strings[*j].as_ref().unwrap())
                            .collect();
                        // println!("Resolved = {:?}", resolved);
                        let multiplied = Resolver::multiply(resolved.as_slice());
                        // println!("Multiplied = {:?}", multiplied);
                        self.allowed_strings[i] = Some(multiplied);
                        *sequence = None;
                        resolved_something = true;
                    } else {
                        println!("Sequence {:?} not resolvable yet", sequence);
                    }
                }
            }
            allowed_strings = self.allowed_strings.clone();
            // break;
            println!("{:?}", self);
            counter += 1;
            println!("{} --------------------", counter);
            if counter > 100 {
                break;
            }
        }

        let valid_messages = self
            .allowed_strings
            .iter()
            .filter_map(|v| v.as_ref())
            .cloned()
            .flatten()
            .collect();
        Validator { valid_messages }
    }
    fn create_allowed_strings(left: &[Vec<String>], right: &[Vec<String>]) -> Vec<String> {
        if left.len() == 1 && right.len() == 1 {
            vec![
                // left[0].clone(), right[0].clone()
            ]
        } else if left.len() == 2 && right.len() == 2 {
            vec![
                // left[0].clone() + &l2[0],
                // left[0].clone() + &l2[1],
                // left[1].clone() + &l2[0],
                // left[1].clone() + &l2[1],
                // right[0].clone() + &r2[0],
                // right[0].clone() + &r2[1],
                // right[1].clone() + &r2[0],
                // right[1].clone() + &r2[1],
            ]
        } else {
            panic!("Unexpected lengths of {:?}, {:?}", left, right);
        }
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

    println!("Rules: {:?}", rules);
    println!("Messages: {:?}", messages);

    let validator = Resolver::from(rules).resolve();
    // println!("Valid messages: {:?}", validator.valid_messages);
    messages
        .iter()
        .filter(|message| validator.is_valid(message))
        .count()
}
