use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

pub(crate) fn invalid_error_sum(input: &[String]) -> usize {
    let (rules, _own_ticket, other_tickets_numbers) = parse_input(input);
    // println!("Rules:\n{:?}", rules);
    // println!("Own:\n{:?}", own_ticket);
    // println!("Others:\n{:?}", other_tickets_numbers);

    let there_is_no_valid_rule_for = |number| !rules.iter().any(|rule| rule.is_valid_for(number));
    other_tickets_numbers
        .iter()
        .map(|other_tickets| {
            other_tickets
                .iter()
                .filter(|number| there_is_no_valid_rule_for(number))
                .sum::<usize>()
        })
        .sum()
}
#[allow(unused)]
fn valid_tickets(input: &[String]) -> Vec<Vec<usize>> {
    let (rules, _own_ticket, other_tickets_numbers) = parse_input(input);
    // println!("Rules:\n{:?}", rules);
    // println!("Own:\n{:?}", own_ticket);
    // println!("Others:\n{:?}", other_tickets_numbers);
    remove_invalid_tickets(&rules, &other_tickets_numbers)
}

fn remove_invalid_tickets(rules: &[Rule], tickets: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let there_is_a_valid_rule_for = |number| rules.iter().any(|rule| rule.is_valid_for(number));
    tickets
        .iter()
        .filter(|numbers| {
            numbers
                .iter()
                .all(|number| there_is_a_valid_rule_for(number))
        })
        .cloned()
        .collect()
}

pub(crate) fn multiply_departure_fields(input: &[String]) -> usize {
    let (rules, own_ticket, other_tickets_numbers) = parse_input(input);
    let valid_tickets = remove_invalid_tickets(&rules, &other_tickets_numbers);

    assign_rules_to_positions2(rules, valid_tickets)
        .iter()
        .filter_map(|(rule_name, index)| {
            if rule_name.starts_with("departure") {
                Some(index)
            } else {
                None
            }
        })
        .map(|i| own_ticket[*i])
        .product()
}

#[allow(unused)]
fn assign_rules_to_positions(input: &[String]) -> HashMap<String, usize> {
    let (rules, _own_ticket, other_tickets_numbers) = parse_input(input);
    let valid_tickets = remove_invalid_tickets(&rules, &other_tickets_numbers);

    assign_rules_to_positions2(rules, valid_tickets)
}

fn assign_rules_to_positions2(
    rules: Vec<Rule>,
    valid_tickets: Vec<Vec<usize>>,
) -> HashMap<String, usize> {
    let mut candidate_indexes_of_rule_by_name = HashMap::new();
    for rule in rules {
        for index in 0..valid_tickets[0].len() {
            if valid_tickets
                .iter()
                .all(|numbers| rule.is_valid_for(&numbers[index]))
            {
                // all tickets' numbers at this index are valid for this rule,
                // so this might be the correct rule for this index
                let indexes = candidate_indexes_of_rule_by_name
                    .entry(rule.name.clone())
                    .or_insert_with(Vec::new);
                indexes.push(index);
                // println!("Inserting {} at {}", rule.name, index);
            }
        }
    }

    let mut uniques_removed = HashSet::new(); // Avoid attempting to remove the same index over and over
    while candidate_indexes_of_rule_by_name
        .values()
        .any(|v| v.len() > 1)
    {
        if let Some(unique_rule_index) = candidate_indexes_of_rule_by_name
            .values()
            .find(|v| v.len() == 1 && !uniques_removed.contains(&v[0]))
            .map(|v| v[0])
        {
            candidate_indexes_of_rule_by_name
                .values_mut()
                .filter(|v| v.len() > 1)
                .for_each(|v| {
                    if let Some(pos) = v.iter().position(|n| *n == unique_rule_index) {
                        // println!("Removing {} at {}", unique_rule_index, pos);
                        v.remove(pos);
                    }
                });
            uniques_removed.insert(unique_rule_index);
            // println!("Candidates: {:?}", candidate_indexes_of_rule_by_name);
        } else {
            unreachable!("Unless there is no rule with a unique index!")
        }
    }
    candidate_indexes_of_rule_by_name
        .into_iter()
        .map(|(k, v)| (k, v[0]))
        .collect()
}

fn parse_input(input: &[String]) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    let ranges = input
        .iter()
        .take_while(|line| !line.is_empty())
        .map(Rule::from)
        .collect();
    let your_ticket = num_string_to_vec(
        input
            .iter()
            .skip_while(|&line| line != "your ticket:")
            .nth(1)
            .unwrap(),
    );
    let nearby_tickets = input
        .iter()
        .skip_while(|&line| line != "nearby tickets:")
        .skip(1)
        .map(|line| num_string_to_vec(line))
        .collect();
    (ranges, your_ticket, nearby_tickets)
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl From<&String> for Rule {
    fn from(line: &String) -> Self {
        let (name, ranges) = line.split_once(": ").unwrap();
        let (range1, range2) = ranges.split_once(" or ").unwrap();
        let (start1, end1) = range1.split_once('-').unwrap();
        let (start2, end2) = range2.split_once('-').unwrap();
        Rule {
            name: name.to_string(),
            ranges: vec![
                start1.parse().unwrap()..=end1.parse().unwrap(),
                start2.parse().unwrap()..=end2.parse().unwrap(),
            ],
        }
    }
}

impl Rule {
    fn is_valid_for(&self, number: &usize) -> bool {
        self.ranges.iter().any(|range| range.contains(number))
    }
}

fn num_string_to_vec(s: &str) -> Vec<usize> {
    s.split(',').map(|c| c.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE1: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn part1_example() {
        assert_eq!(invalid_error_sum(&read_str_to_lines(EXAMPLE1)), 71);
    }

    #[test]
    fn part1() {
        assert_eq!(
            invalid_error_sum(&read_file_to_lines("input/day16.txt")),
            19240
        );
    }

    #[test]
    fn part2_example1_valid_tickets() {
        assert_eq!(valid_tickets(&read_str_to_lines(EXAMPLE1)), [[7, 3, 47]]);
    }

    const EXAMPLE2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn part2_example2_valid_tickets() {
        assert_eq!(
            valid_tickets(&read_str_to_lines(EXAMPLE2)),
            [[3, 9, 18], [15, 1, 5], [5, 14, 9]]
        );
    }

    #[test]
    fn part2_example2_assign_rules_to_positions() {
        assert_eq!(
            assign_rules_to_positions(&read_str_to_lines(EXAMPLE2)),
            [("seat", 2), ("row", 0), ("class", 1)]
                .iter()
                .map(|(s, i)| (s.to_string(), *i))
                .collect()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            multiply_departure_fields(&read_file_to_lines("input/day16.txt")),
            21095351239483
        );
    }
}
