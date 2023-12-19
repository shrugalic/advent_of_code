use std::collections::HashMap;

use Category::*;
use Inequality::*;

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

const INPUT: &str = include_str!("../input/day19.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &'static str) -> usize {
    let (workflows_by_name, mut parts) = parse(input);
    let mut accepted = vec![];
    while let Some(part) = parts.pop() {
        let mut name = "in";
        while let Some(workflow) = workflows_by_name.get(&name) {
            name = workflow
                .rules
                .iter()
                .find(|rule| rule.applies_to(&part))
                .map(|rule| rule.target)
                .unwrap_or(workflow.fallback);
            if name == ACCEPTED {
                accepted.push(part);
                break;
            } else if name == REJECTED {
                break;
            }
        }
    }
    accepted.iter().map(Part::sum_of_ratings).sum()
}

fn solve_part2(input: &'static str) -> usize {
    let (mut workflows_by_name, _parts) = parse(input);

    // Follow all paths of the tree from the root, and remember the conditions taken at each fork.
    // If a path ends in an ACCEPTED node, it will be considered later

    let mut accepted: Vec<_> = vec![];
    let initial_conditions_by_rating = vec![vec![]; 4]; // (Category as usize) as index
    let mut node_queue = vec![("in", initial_conditions_by_rating)];
    while let Some((name, mut conditions)) = node_queue.pop() {
        let mut add_to_queue_or_accepted = |target, conditions| {
            if target == ACCEPTED {
                accepted.push(conditions);
            } else if target != REJECTED {
                node_queue.push((target, conditions));
            }
        };
        if let Some(workflow) = workflows_by_name.remove(&name) {
            for rule in &workflow.rules {
                // rule is matched -> continue at target workflow (with the matched condition added)
                let mut match_conditions = conditions.clone();
                match_conditions[rule.category as usize].push(rule.condition.clone());
                add_to_queue_or_accepted(rule.target, match_conditions);

                // rule is unmatched -> add negated condition and continue with next rule
                conditions[rule.category as usize].push(rule.condition.negated());
            }
            // rules are exhausted -> go to fallback
            add_to_queue_or_accepted(workflow.fallback, conditions);
        } else {
            unreachable!("There are multiple rules with target {name}!")
        }
    }

    accepted
        .into_iter()
        .map(|conditions_by_rating| {
            conditions_by_rating
                .into_iter()
                .map(convert_to_count_of_acceptable_ratings)
                .product::<usize>()
        })
        .sum()
}

fn convert_to_count_of_acceptable_ratings(mut conditions: Vec<Inequality>) -> usize {
    if conditions.is_empty() {
        return 4000;
    }
    conditions.push(GreaterThan(0));
    conditions.push(LessThan(4001));
    conditions.sort_unstable_by_key(|x| x.value());
    // Sorted into something like this:
    // [GreaterThan(0), LessThan(1000), GreaterThan(2000), LessThan(3000), LessThan(4001)]
    // Then consolidated into a range by looking at neighboring pairs from left to right
    // A pair of (LessThan, _) or (_, GreaterThan) means it was already included in a previous pair
    // The above example resolves into the ranges 1..1000 and 2001..3000
    conditions
        .windows(2)
        .filter_map(|w| match (&w[0], &w[1]) {
            (LessThan(_), _) | (_, GreaterThan(_)) => None, // already included
            (GreaterThan(below), LessThan(above)) => Some((above - below - 1) as usize),
        })
        .sum()
}

fn parse(input: &'static str) -> (HashMap<WorkflowName, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.trim().split_once("\n\n").unwrap();
    let workflows: Vec<_> = workflows.lines().map(Workflow::from).collect();
    let parts = parts.lines().map(Part::from).collect();
    let workflows_by_name = workflows.into_iter().map(|w| (w.name, w)).collect();
    (workflows_by_name, parts)
}

type Value = u16;
type WorkflowName<'a> = &'a str;

#[derive(Debug, PartialEq)]
struct Workflow<'a> {
    name: WorkflowName<'a>,
    rules: Vec<Rule<'a>>,
    fallback: WorkflowName<'a>,
}

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    category: Category,
    condition: Inequality,
    target: WorkflowName<'a>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
enum Inequality {
    LessThan(Value),
    GreaterThan(Value),
}

#[derive(Debug, PartialEq)]
struct Part {
    x: Value,
    m: Value,
    a: Value,
    s: Value,
}

impl Rule<'_> {
    fn applies_to(&self, part: &Part) -> bool {
        let value = &match self.category {
            X => part.x,
            M => part.m,
            A => part.a,
            S => part.s,
        };
        self.condition.holds_for(value)
    }
}

impl Inequality {
    fn holds_for(&self, value: &Value) -> bool {
        match self {
            LessThan(threshold) => value < threshold,
            GreaterThan(threshold) => value > threshold,
        }
    }
    fn value(&self) -> Value {
        *match self {
            LessThan(value) => value,
            GreaterThan(value) => value,
        }
    }
    fn negated(&self) -> Self {
        match self {
            LessThan(value) => GreaterThan(value - 1),
            GreaterThan(value) => LessThan(value + 1),
        }
    }
}

impl Part {
    fn sum_of_ratings(&self) -> usize {
        (self.x + self.m + self.a + self.s) as usize
    }
}

impl From<&'static str> for Workflow<'_> {
    fn from(line: &'static str) -> Self {
        // px{a<2006:qkq,m>2090:A,rfg}
        let (name, rest) = line.strip_suffix('}').unwrap().split_once('{').unwrap();
        let mut parts: Vec<_> = rest.split(',').collect();
        let fallback = parts.remove(parts.len() - 1);
        let rules = parts.into_iter().map(Rule::from).collect();
        Workflow {
            name,
            rules,
            fallback,
        }
    }
}
impl From<&'static str> for Rule<'_> {
    fn from(line: &'static str) -> Self {
        // a<2006:qkq
        let (condition, target) = line.split_once(':').unwrap();
        let category = Category::from(&condition[..1]);
        let condition = Inequality::from(&condition[1..]);
        Rule {
            category,
            condition,
            target,
        }
    }
}
impl From<&str> for Category {
    fn from(line: &str) -> Self {
        match line {
            "x" => X,
            "m" => M,
            "a" => A,
            "s" => S,
            _ => unreachable!("Illegal category {line}"),
        }
    }
}
impl From<&str> for Inequality {
    fn from(line: &str) -> Self {
        let value = line[1..].parse().unwrap();
        if line.starts_with('<') {
            LessThan(value)
        } else if line.starts_with('>') {
            GreaterThan(value)
        } else {
            unreachable!("Illegal relation {line}")
        }
    }
}
impl From<&str> for Part {
    fn from(line: &str) -> Self {
        // {x=787,m=2655,a=1222,s=2876}
        let mut it = line
            .split(&['{', '}', '=', ',', 'x', 'm', 'a', 's'])
            .filter_map(|n| n.parse().ok());
        Part {
            x: it.next().unwrap(),
            m: it.next().unwrap(),
            a: it.next().unwrap(),
            s: it.next().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_parsing_a_workflow_from_str() {
        assert_eq!(
            Workflow::from("px{a<2006:qkq,m>2090:A,rfg}"),
            Workflow {
                name: "px",
                rules: vec![
                    Rule {
                        category: A,
                        condition: LessThan(2006),
                        target: "qkq"
                    },
                    Rule {
                        category: M,
                        condition: GreaterThan(2090),
                        target: ACCEPTED
                    }
                ],
                fallback: "rfg"
            }
        );
    }

    #[test]
    fn test_parsing_a_part_from_str() {
        assert_eq!(
            Part::from("{x=787,m=2655,a=1222,s=2876}"),
            Part {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876,
            }
        );
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(19_114, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(319_062, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(167_409_079_868_000, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(118_638_369_682_135, solve_part2(INPUT));
    }
}
