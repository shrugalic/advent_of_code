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
    // Store counts of element pairs, instead of storing the full polymer (which can get long).
    // The polymer "NNCB" would be stored as pairs { "NN" = 1, "NC" = 1, "CB" = 1 } and last = 'B'.
    // Char frequencies are the sum of counts of each pair's first char, plus 1 for the last char:
    // 'N' = 2, 'C' = 1, 'B' = 1
    pairs: HashMap<Pair, usize>,
    last: Element,
    rules: HashMap<Pair, Element>,
}
impl Polymer {
    fn grow(mut self, step_count: usize) -> Self {
        for _i in 0..step_count {
            // println!("{}. {}", _i, self);
            self.insert_element_between_each_pair();
        }
        self
    }
    fn insert_element_between_each_pair(&mut self) {
        let mut pairs = HashMap::new();
        for (pair, count) in self.pairs.drain() {
            let elem = self.rules.get(&pair).unwrap();
            *pairs.entry((pair.0, *elem)).or_default() += count;
            *pairs.entry((*elem, pair.1)).or_default() += count;
        }
        self.pairs = pairs;
    }
    fn diff_between_most_and_least_frequent_element(&self) -> usize {
        let mut freq: HashMap<Element, usize> = HashMap::new();
        for (pair, count) in &self.pairs {
            *freq.entry(pair.0).or_default() += count;
        }
        *freq.entry(self.last).or_default() += 1;
        freq.values().max().unwrap() - freq.values().min().unwrap()
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
        let last = template.chars().last().unwrap();

        for connection in rule_lines.lines() {
            let (from, to) = connection.split_once(" -> ").unwrap();
            let from: Vec<char> = from.chars().collect();
            rules.insert((from[0], from[1]), to.chars().next().unwrap());
        }

        Polymer { pairs, last, rules }
    }
}
impl Display for Polymer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let format = |pair: &Pair| format!("{}{}", pair.0, pair.1);
        write!(
            f,
            "Element pairs:\n{}\nLast element:\n- {}\nRules:\n{}\n",
            self.pairs
                .iter()
                .map(|(pair, count)| format!("- {}: {}", format(pair), count))
                .collect::<Vec<_>>()
                .join("\n"),
            self.last,
            self.rules
                .iter()
                .map(|(pair, to)| format!("- {} -> {}", format(pair), *to))
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
