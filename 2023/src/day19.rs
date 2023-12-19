use std::collections::HashMap;
use std::ops::RangeInclusive;

use Category::*;
use Inequality::*;
use RuleResult::*;
use RuleType::*;

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
            match workflow
                .rules
                .iter()
                .find(|rule| rule.applies_to(&part))
                .map(|rule| rule.result)
                .expect("fallback rule to always match")
            {
                Accepted => {
                    accepted.push(part);
                    break;
                }
                Rejected => break,
                Workflow(id) => {
                    name = id;
                }
            }
        }
    }
    accepted.iter().map(Part::sum_of_ratings).sum()
}

fn solve_part2(input: &'static str) -> usize {
    let (mut workflows_by_name, _parts) = parse(input);
    // Recursively traverse the full workflow tree, and narrow down the maximum ranges as we go
    count_combinations("in", RangesByCategory::maximum(), &mut workflows_by_name)
}

fn count_combinations(
    id: WorkflowName,
    mut possible_ranges: RangesByCategory,
    workflows_by_name: &mut HashMap<WorkflowName, Workflow>,
) -> usize {
    let mut workflow = workflows_by_name
        .remove(id)
        .expect("single target pointing to workflow");
    let mut combination_count = 0;
    for rule in workflow.rules.drain(..) {
        match &rule.rule_type {
            Conditional {
                category,
                condition,
            } => {
                // Count matched -> continue at target workflow (with the matched condition applied)
                let new_ranges = possible_ranges.narrowed_down_by(condition, category);
                combination_count += match rule.result {
                    Accepted => new_ranges.convert_to_count_of_acceptable_ratings(),
                    Workflow(id) => count_combinations(id, new_ranges, workflows_by_name),
                    Rejected => 0,
                };

                // Count unmatched -> apply negated condition and continue with next rule
                let inequality = condition.negated();
                possible_ranges.ranges[*category as usize].exclude(inequality);
            }
            UnconditionalFallBack => {
                // No narrowing down any ranges on this last rule of a workflow
                return combination_count
                    + match rule.result {
                        Accepted => possible_ranges.convert_to_count_of_acceptable_ratings(),
                        Workflow(id) => count_combinations(id, possible_ranges, workflows_by_name),
                        Rejected => 0,
                    };
            }
        }
    }
    combination_count
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
}

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    rule_type: RuleType,
    result: RuleResult<'a>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RuleResult<'a> {
    Accepted,
    Rejected,
    Workflow(WorkflowName<'a>),
}

#[derive(Debug, PartialEq)]
enum RuleType {
    Conditional {
        category: Category,
        condition: Inequality,
    },
    UnconditionalFallBack,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
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
        match &self.rule_type {
            Conditional {
                category,
                condition,
            } => {
                let value = &match category {
                    X => part.x,
                    M => part.m,
                    A => part.a,
                    S => part.s,
                };
                condition.holds_for(value)
            }
            UnconditionalFallBack => true,
        }
    }
}

impl Inequality {
    fn holds_for(&self, value: &Value) -> bool {
        match self {
            LessThan(threshold) => value < threshold,
            GreaterThan(threshold) => value > threshold,
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

#[derive(Clone, Debug)]
struct RangesByCategory {
    // indexed by (Category as usize)
    ranges: [RangeInclusive<Value>; 4],
}

impl RangesByCategory {
    fn narrowed_down_by(&self, inequality: &Inequality, category: &Category) -> Self {
        let mut new_ranges = self.clone();
        new_ranges.ranges[*category as usize].exclude(*inequality);
        new_ranges
    }
    fn convert_to_count_of_acceptable_ratings(self) -> usize {
        self.ranges
            .iter()
            .map(RangeInclusive::len)
            .product::<usize>()
    }
    fn maximum() -> Self {
        Self {
            ranges: [1..=4000, 1..=4000, 1..=4000, 1..=4000],
        }
    }
}
trait ExcludeRange {
    fn exclude(&mut self, inequality: Inequality);
}

impl ExcludeRange for RangeInclusive<Value> {
    fn exclude(&mut self, inequality: Inequality) {
        match inequality {
            LessThan(value) => {
                if self.contains(&value) {
                    *self = *self.start()..=(value - 1);
                }
            }
            GreaterThan(value) => {
                if self.contains(&value) {
                    *self = value + 1..=*self.end();
                }
            }
        }
    }
}

impl From<&'static str> for Workflow<'_> {
    fn from(line: &'static str) -> Self {
        // px{a<2006:qkq,m>2090:A,rfg}
        let (name, rest) = line.strip_suffix('}').unwrap().split_once('{').unwrap();
        let parts: Vec<_> = rest.split(',').collect();
        let rules = parts.into_iter().map(Rule::from).collect();
        Workflow { name, rules }
    }
}
impl From<&'static str> for Rule<'_> {
    fn from(line: &'static str) -> Self {
        // a<2006:qkq or rfg
        let (rule_type, target) = if let Some((condition, target)) = line.split_once(':') {
            let category = Category::from(&condition[..1]);
            let condition = Inequality::from(&condition[1..]);
            (
                Conditional {
                    category,
                    condition,
                },
                target,
            )
        } else {
            (UnconditionalFallBack, line)
        };
        let result = match target {
            "A" => Accepted,
            "R" => Rejected,
            id => Workflow(id),
        };
        Rule { rule_type, result }
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
                        rule_type: Conditional {
                            category: A,
                            condition: LessThan(2006)
                        },
                        result: Workflow("qkq")
                    },
                    Rule {
                        rule_type: Conditional {
                            category: M,
                            condition: GreaterThan(2090)
                        },
                        result: Accepted
                    },
                    Rule {
                        rule_type: UnconditionalFallBack,
                        result: Workflow("rfg")
                    }
                ],
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
