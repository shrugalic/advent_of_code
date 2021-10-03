use line_reader::read_file_to_lines;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub(crate) fn day19_part1() -> usize {
    let (replacements, molecule) = parse_day19_input();
    let replacements = parse_replacements(&replacements);
    results_of_one_replacement(&molecule, &replacements).len()
}

pub(crate) fn day19_part2() -> usize {
    let (replacements, medicine) = parse_day19_input();
    let replacements = parse_reverse_replacements(&replacements);
    count_number_of_replacements(&medicine, &replacements)
}

const STARTING_MOLECULE: &str = "e";

fn parse_day19_input() -> (Vec<String>, String) {
    let input = read_file_to_lines("input/day19.txt");
    let parts: Vec<_> = input.split(|element| element.is_empty()).collect();
    (parts[0].to_vec(), parts[1][0].to_string()) // (replacements, molecule)
}

type Replacements = HashMap<String, Vec<String>>;
type Results = HashSet<String>;

fn parse_replacements(input: &[String]) -> Replacements {
    replacements_from(input, &|line| line.split_once(" => ").unwrap())
}

fn parse_reverse_replacements(input: &[String]) -> Replacements {
    replacements_from(input, &|line| {
        line.split_once(" => ").map(|(l, r)| (r, l)).unwrap()
    })
}

fn replacements_from(input: &[String], split: &dyn Fn(&String) -> (&str, &str)) -> Replacements {
    let mut replacements = Replacements::new();
    input.iter().for_each(|line| {
        let (left, right) = split(line);
        replacements
            .entry(left.to_string())
            .or_insert_with(Vec::new)
            .push(right.to_string())
    });
    replacements
}

fn results_of_one_replacement(molecule: &str, replacements: &Replacements) -> Results {
    let mut results = Results::new();

    for source in replacements.keys() {
        let mut start = 0;
        while let Some(pos) = molecule[start..].find(source) {
            let left = molecule[..start + pos].to_string();
            let right = molecule[(start + pos + source.len())..].to_string();
            if let Some(outs) = replacements.get(source) {
                outs.iter().for_each(|result| {
                    results.insert(format!("{}{}{}", left, result, right));
                });
            }
            start += pos + source.len();
        }
    }
    results
}

#[derive(PartialEq, Eq)]
struct ReplacementStep {
    count: usize,
    molecule: String,
}
impl Ord for ReplacementStep {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.molecule.len().cmp(&other.molecule.len()) {
            Ordering::Equal => self.count.cmp(&other.count).reverse(),
            shorter_is_better => shorter_is_better.reverse(),
        }
    }
}
impl PartialOrd for ReplacementStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn count_number_of_replacements(curr: &str, replacements: &Replacements) -> usize {
    let mut candidates = BinaryHeap::new();
    candidates.push(ReplacementStep {
        count: 0,
        molecule: curr.to_string(),
    });

    while let Some(curr) = candidates.pop() {
        if curr.molecule == STARTING_MOLECULE {
            return curr.count;
        }
        results_of_one_replacement(&curr.molecule, replacements)
            .into_iter()
            .for_each(|next| {
                candidates.push(ReplacementStep {
                    count: curr.count + 1,
                    molecule: next,
                })
            });
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE1_REPLACEMENTS: &str = "\
H => HO
H => OH
O => HH";
    const EXAMPLE1_START: &str = "HOH";

    #[test]
    fn part1_example() {
        let input = read_str_to_lines(EXAMPLE1_REPLACEMENTS);
        let replacements = parse_replacements(&input);
        let results = results_of_one_replacement(EXAMPLE1_START, &replacements);
        assert_eq!(4, results.len());
    }

    #[test]
    fn part1() {
        assert_eq!(535, day19_part1());
    }

    const EXAMPLE2_REPLACEMENTS: &str = "\
e => H
e => O
H => HO
H => OH
O => HH";

    const EXAMPLE2_MOLECULE1: &str = "HOH";
    const EXAMPLE2_MOLECULE2: &str = "HOHOHO";

    #[test]
    fn part2_examples() {
        let input = read_str_to_lines(EXAMPLE2_REPLACEMENTS);
        let replacements = parse_reverse_replacements(&input);
        assert_eq!(
            3,
            count_number_of_replacements(EXAMPLE2_MOLECULE1, &replacements)
        );
        assert_eq!(
            6,
            count_number_of_replacements(EXAMPLE2_MOLECULE2, &replacements)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(212, day19_part2());
    }
}
