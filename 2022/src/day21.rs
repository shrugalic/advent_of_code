use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use MonkeyJob::*;
use MonkeyOp::*;
use NameOrValue::*;

const INPUT: &str = include_str!("../input/day21.txt");

pub(crate) fn day21_part1() -> isize {
    Tree::from(INPUT).part1()
}

pub(crate) fn day21_part2() -> isize {
    Tree::from(INPUT).part2()
}

const ROOT: &str = "root";
const HUMN: &str = "humn";

impl<'a> Tree<'a> {
    fn part1(&self) -> isize {
        self.evaluate(ROOT)
    }
    fn evaluate(&self, root: MonkeyName) -> isize {
        match self.tree.get(root).unwrap() {
            Number(n) => *n,
            Combine { lhs, rhs, op } => {
                let left = match lhs {
                    Name(left) => self.evaluate(left),
                    Value(left) => *left,
                };
                let right = match rhs {
                    Name(right) => self.evaluate(right),
                    Value(right) => *right,
                };
                op.apply(left, right)
            }
        }
    }
    fn part2(&mut self) -> isize {
        let values = self.get_non_human_key_value_pairs(ROOT);
        for (key, value) in values {
            remove_subtree(&mut self.tree, key);
            self.tree.insert(key, Number(value));
        }

        while self.simplified() {
            //
        }

        if let Combine { lhs, rhs, .. } = self.tree.get(ROOT).unwrap() {
            match (lhs, rhs) {
                (Name(l), Value(r)) => self.calculate_human_value_to_satisfy_equation(l, *r),
                (Value(l), Name(r)) => self.calculate_human_value_to_satisfy_equation(r, *l),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }

    fn simplified(&mut self) -> bool {
        let previous_tree_size = self.tree.len();
        // Find Number and Combine operations with numbers on both sides…
        let values_by_name: BTreeMap<MonkeyName, isize> = self
            .tree
            .iter()
            .filter(|(name, _)| name != &&HUMN)
            .filter_map(|(name, job)| match job {
                Number(value) => Some((*name, *value)),
                Combine {
                    lhs: Value(l),
                    rhs: Value(r),
                    op,
                } => Some((*name, op.apply(*l, *r))),
                _ => None,
            })
            .collect();

        // …and put them into other Combine operations…
        for job in self.tree.values_mut() {
            if let Combine { lhs, rhs, .. } = job {
                if let Name(name) = lhs {
                    if name != &HUMN {
                        if let Some(value) = values_by_name.get(name) {
                            *lhs = Value(*value);
                        }
                    }
                }
                if let Name(name) = rhs {
                    if name != &HUMN {
                        if let Some(value) = values_by_name.get(name) {
                            *rhs = Value(*value);
                        }
                    }
                }
            }
        }
        // …and then remove the now unnecessary nodes
        for (name, _) in values_by_name {
            self.tree.remove(name);
        }

        // Did the tree shrink?
        self.tree.len() < previous_tree_size
    }
    fn calculate_human_value_to_satisfy_equation(&self, root: MonkeyName, value: isize) -> isize {
        if root == HUMN {
            return value;
        }
        // One of the sides contains a human, and should be balanced with the other side
        match self.tree.get(root).unwrap() {
            Combine { lhs, rhs, op } => match (lhs, rhs) {
                (Name(l), Value(r)) => {
                    let value = match op {
                        Addition => value - r,
                        Subtraction => value + r,
                        Multiplication => value / r,
                        Division => value * r,
                    };
                    self.calculate_human_value_to_satisfy_equation(l, value)
                }
                (Value(l), Name(r)) => {
                    let value = match op {
                        Addition => value - l,
                        Subtraction => l - value,
                        Multiplication => value / l,
                        Division => l / value,
                    };
                    self.calculate_human_value_to_satisfy_equation(r, value)
                }
                (Value(_), Value(_)) | (Name(_), Name(_)) => {
                    unreachable!()
                }
            },
            Number(_) => {
                unreachable!()
            }
        }
    }
    fn get_non_human_key_value_pairs(&self, root: MonkeyName<'a>) -> Vec<(MonkeyName<'a>, isize)> {
        if root == HUMN {
            return vec![];
        }
        match self.tree.get(root).unwrap() {
            Number(value) => vec![(root, *value)],
            Combine { lhs, rhs, .. } => {
                let mut pairs: Vec<(MonkeyName, isize)> = vec![];
                if let Name(left) = rhs {
                    if let Some(value) = self.evaluate_unless_human(lhs) {
                        pairs.push((left, value));
                    }
                }
                if let Name(right) = rhs {
                    if let Some(value) = self.evaluate_unless_human(rhs) {
                        pairs.push((right, value));
                    }
                }
                pairs
            }
        }
    }
    fn evaluate_unless_human(&self, root: &NameOrValue) -> Option<isize> {
        match root {
            Name(name) => {
                if name == &HUMN {
                    return None;
                }
                match self.tree.get(name).unwrap() {
                    Number(n) => Some(*n),
                    Combine { lhs, rhs, op } => {
                        match (
                            self.evaluate_unless_human(lhs),
                            self.evaluate_unless_human(rhs),
                        ) {
                            (Some(left), Some(right)) => Some(op.apply(left, right)),
                            _ => None,
                        }
                    }
                }
            }
            Value(value) => Some(*value),
        }
    }
}

fn remove_subtree(tree: &mut BTreeMap<MonkeyName, MonkeyJob>, root: MonkeyName) {
    match tree.remove(root).unwrap() {
        Number(_) => {
            tree.remove(root);
        }
        Combine { lhs, rhs, .. } => {
            if let Name(left) = lhs {
                remove_subtree(tree, left);
            }
            if let Name(right) = rhs {
                remove_subtree(tree, right);
            }
        }
    }
}

#[derive(Debug)]
struct Tree<'a> {
    tree: BTreeMap<MonkeyName<'a>, MonkeyJob<'a>>,
}
impl<'a> From<&'a str> for Tree<'a> {
    fn from(input: &'a str) -> Self {
        Tree {
            tree: input
                .lines()
                .map(|line| line.split_once(": ").unwrap())
                .map(|(name, op)| (name, MonkeyJob::from(op)))
                .collect(),
        }
    }
}

type MonkeyName<'a> = &'a str;

#[derive(Debug)]
enum MonkeyJob<'a> {
    Number(isize),
    Combine {
        lhs: NameOrValue<'a>,
        op: MonkeyOp,
        rhs: NameOrValue<'a>,
    },
}
#[derive(Debug)]
enum NameOrValue<'a> {
    Name(MonkeyName<'a>),
    Value(isize),
}

#[derive(Debug)]
enum MonkeyOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
impl MonkeyOp {
    fn apply(&self, l: isize, r: isize) -> isize {
        match self {
            Addition => l + r,
            Subtraction => l - r,
            Multiplication => l * r,
            Division => l / r,
        }
    }
}
impl Display for MonkeyOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Addition => "+",
                Subtraction => "-",
                Multiplication => "*",
                Division => "/",
            }
        )
    }
}
impl<'a> From<&'a str> for MonkeyJob<'a> {
    fn from(s: &'a str) -> Self {
        let p: Vec<_> = s.split(' ').collect();
        if p.len() == 1 {
            Number(p[0].parse().unwrap())
        } else {
            let lhs = Name(p[0]);
            let rhs = Name(p[2]);
            let op = match p[1] {
                "+" => Addition,
                "-" => Subtraction,
                "*" => Multiplication,
                "/" => Division,
                _ => unimplemented!("Unknown operation {}", p[1]),
            };
            Combine { lhs, rhs, op }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_example() {
        assert_eq!(152, Tree::from(EXAMPLE).part1());
    }

    #[test]
    fn part1() {
        assert_eq!(110_181_395_003_396, day21_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(301, Tree::from(EXAMPLE).part2());
    }

    #[test]
    fn part2() {
        assert_eq!(3_721_298_272_959, day21_part2());
    }
}
