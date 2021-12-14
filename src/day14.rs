use std::collections::HashMap;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day14.txt");

pub(crate) fn day14_part1() -> usize {
    Polymer::from(INPUT)
        .grow(10)
        .diff_between_most_and_least_frequent_element()
}

pub(crate) fn day14_part2() -> usize {
    Polymer::from(INPUT)
        .grow(40)
        .diff_between_most_and_least_frequent_element()
}

type Element = char;
type Pair = (Element, Element);

#[derive(Debug)]
struct Polymer {
    pairs: HashMap<Pair, usize>,
    last: Element,
    rules: HashMap<Pair, Element>,
}
impl Polymer {
    fn grow(mut self, step_count: usize) -> Self {
        for _i in 0..step_count {
            // println!("{}. {}", _i, self);
            self.step();
        }
        self
    }
    fn diff_between_most_and_least_frequent_element(&self) -> usize {
        let mut freq: HashMap<Element, usize> = HashMap::new();
        for (pair, count) in &self.pairs {
            *freq.entry(pair.0).or_default() += count;
        }
        *freq.entry(self.last).or_default() += 1;
        freq.values().max().unwrap() - freq.values().min().unwrap()
    }
    fn step(&mut self) {
        let mut pairs = HashMap::new();
        for (pair, count) in self.pairs.drain() {
            let elem = self.rules.get(&pair).unwrap();
            *pairs.entry((pair.0, *elem)).or_default() += count;
            *pairs.entry((*elem, pair.1)).or_default() += count;
        }
        self.pairs = pairs;
    }
}
impl From<&str> for Polymer {
    fn from(input: &str) -> Self {
        let mut pairs = HashMap::new();
        let mut rules = HashMap::new();

        let (template, rule_lines) = input.trim().split_once("\n\n").unwrap();
        for pair in template.as_bytes().windows(2) {
            *pairs.entry((pair[0] as char, pair[1] as char)).or_default() += 1;
        }
        let last = *template.as_bytes().last().unwrap() as char;

        for connection in rule_lines.lines() {
            let (from, to) = connection.split_once(" -> ").unwrap();
            let from = from.as_bytes();
            rules.insert(
                (from[0] as char, from[1] as char),
                *to.as_bytes().first().unwrap() as char,
            );
        }

        Polymer { pairs, last, rules }
    }
}
impl Display for Polymer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pts = |pair: &Pair| format!("{}{}", pair.0, pair.1);
        write!(
            f,
            "Element pairs:\n{}\nLast element:\n- {}\nRules:\n{}\n",
            self.pairs
                .iter()
                .map(|(pair, count)| format!("- {}: {}", pts(pair), count))
                .collect::<Vec<_>>()
                .join("\n"),
            self.last,
            self.rules
                .iter()
                .map(|(pair, to)| format!("- {} -> {}", pts(pair), *to))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1_example() {
        let polymer = Polymer::from(EXAMPLE).grow(10);
        assert_eq!(1588, polymer.diff_between_most_and_least_frequent_element());
    }

    #[test]
    fn part1() {
        assert_eq!(2068, day14_part1());
    }

    #[test]
    fn part2_example() {
        let polymer = Polymer::from(EXAMPLE).grow(40);
        assert_eq!(
            2_188_189_693_529,
            polymer.diff_between_most_and_least_frequent_element()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(2_158_894_777_814, day14_part2());
    }
}
