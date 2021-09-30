use line_reader::read_file_to_lines;
use std::collections::{HashMap, HashSet};

pub(crate) fn day19_part1() -> usize {
    let (replacements, molecule) = parse_day19_input();
    let replacements = parse_replacements(&replacements);
    results_of_one_replacement(&molecule, &replacements).len()
}

pub(crate) fn day19_part2() -> usize {
    let (replacements, medicine) = parse_day19_input();
    let replacements = parse_replacements(&replacements);
    number_of_replacements(STARTING_MOLECULE, &medicine, &replacements)
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
    let mut replacements = Replacements::new();
    input.iter().for_each(|line| {
        let (from, to) = line.split_once(" => ").unwrap();
        replacements
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
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

fn number_of_replacements(start: &str, target: &str, replacements: &Replacements) -> usize {
    replacement_count(start, target, replacements).unwrap()
}

fn replacement_count(start: &str, target: &str, replacements: &Replacements) -> Option<usize> {
    if start == target {
        Some(0)
    } else if start.len() > target.len() {
        None
    } else {
        results_of_one_replacement(start, replacements)
            .into_iter()
            .filter_map(|new_start| replacement_count(&new_start, target, replacements))
            .map(|c| c + 1)
            .min()
    }
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

    const EXAMPLE2_TARGET1: &str = "HOH";
    const EXAMPLE2_TARGET2: &str = "HOHOHO";

    #[test]
    fn part2_examples() {
        let input = read_str_to_lines(EXAMPLE2_REPLACEMENTS);
        let replacements = parse_replacements(&input);
        assert_eq!(
            3,
            number_of_replacements(STARTING_MOLECULE, EXAMPLE2_TARGET1, &replacements)
        );
        assert_eq!(
            6,
            number_of_replacements(STARTING_MOLECULE, EXAMPLE2_TARGET2, &replacements)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(0, day19_part2());
    }
}
