use std::ops::RangeInclusive;

fn invalid_error_sum(input: &[String]) -> usize {
    let (rules, _own_ticket, other_tickets_numbers) = parse_input(input);
    // println!("Rules:\n{:?}", rules);
    // println!("Own:\n{:?}", own_ticket);
    // println!("Others:\n{:?}", other_tickets_numbers);

    other_tickets_numbers
        .iter()
        .map(|other_ticket_numbers| {
            other_ticket_numbers
                .iter()
                .filter(|number| {
                    rules
                        .iter()
                        .all(|rule| !rule.ranges.iter().any(|range| range.contains(number)))
                })
                .sum::<usize>()
        })
        .sum()
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

fn num_string_to_vec(s: &str) -> Vec<usize> {
    s.split(',').map(|c| c.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "class: 1-3 or 5-7
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
        assert_eq!(invalid_error_sum(&read_str_to_lines(EXAMPLE)), 71);
    }

    #[test]
    fn part1() {
        assert_eq!(invalid_error_sum(&read_file_to_lines("input.txt")), 19240);
    }
}
