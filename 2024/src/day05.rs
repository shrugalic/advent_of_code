use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../2024/input/day05.txt");

pub fn part1() -> u32 {
    solve_part1(INPUT)
}

pub fn part2() -> u32 {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|update| update.is_valid(&rules))
        .map(get_middle_page)
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|update| !update.is_valid(&rules))
        .map(|update| update.make_valid(&rules))
        .map(get_middle_page)
        .sum()
}

fn parse(input: &str) -> (PageOrderingRules, Vec<PageUpdate>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|n| n.parse::<Page>().ok())
                .collect()
        })
        .collect();
    (PageOrderingRules::from(rules), updates)
}

type Page = u32;
type PageUpdate = Vec<Page>;

fn get_middle_page(pages: PageUpdate) -> Page {
    pages[pages.len() / 2]
}

struct PageOrderingRules {
    ordered_pairs: HashSet<(Page, Page)>,
}
impl From<&str> for PageOrderingRules {
    fn from(rules: &str) -> Self {
        PageOrderingRules {
            ordered_pairs: rules
                .lines()
                .map(|line| {
                    let pair = line.split_once('|').unwrap();
                    let earlier = pair.0.parse::<Page>().unwrap();
                    let later = pair.1.parse::<Page>().unwrap();
                    (earlier, later)
                })
                .collect(),
        }
    }
}

trait PageUpdateOrder {
    fn is_valid(&self, rules: &PageOrderingRules) -> bool;
    fn make_valid(self, rules: &PageOrderingRules) -> PageUpdate;
}
impl PageUpdateOrder for PageUpdate {
    fn is_valid(&self, rules: &PageOrderingRules) -> bool {
        self == &self.clone().make_valid(rules)
    }
    fn make_valid(mut self, rules: &PageOrderingRules) -> PageUpdate {
        self.sort_by(|page_a, page_b| {
            if rules.ordered_pairs.contains(&(*page_a, *page_b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1_example() {
        assert_eq!(61 + 53 + 29, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(5_374, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(47 + 29 + 47, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4_260, solve_part2(INPUT));
    }
}
