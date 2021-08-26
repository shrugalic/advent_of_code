use line_reader::read_file_to_lines;
use std::collections::{HashMap, HashSet};

pub(crate) fn day7_part1() -> String {
    root_node(read_file_to_lines("input/day07.txt"))
}

pub(crate) fn day7_part2() -> usize {
    fixed_weight_of_imbalancing_program(read_file_to_lines("input/day07.txt"))
}

fn root_node(lines: Vec<String>) -> String {
    let mut parents = HashSet::new();
    let mut children = HashSet::new();
    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        // ktlj (57)
        parents.insert(parts[0]);
        for part in parts.into_iter().skip(3) {
            // fwft (72) -> ktlj, cntj, xhth
            children.insert(part.trim_end_matches(','));
        }
    });
    let diff: HashSet<_> = parents.difference(&children).collect();
    diff.iter().next().unwrap().to_string()
}

fn name_and_weight_from(line: &str) -> (Name, Weight) {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();
    let name = parts[0];
    let weight = parts[1]
        .trim_start_matches('(')
        .trim_end_matches(')')
        .parse()
        .unwrap();
    (name.to_string(), weight)
}

type Name = String;
type Weight = usize;
type WeightByName = HashMap<Name, WeightOfSubtree>;

type ChildrenNames = HashSet<Name>;
type Connection = (Name, ChildrenNames);
type ChildrenNamesByParentName = HashMap<Name, ChildrenNames>;

#[derive(Clone)]
struct WeightOfSubtree {
    parent: Weight,
    children: Weight,
}
impl WeightOfSubtree {
    fn new(parent: Weight) -> Self {
        WeightOfSubtree {
            parent,
            children: 0,
        }
    }
    fn total(&self) -> Weight {
        self.parent + self.children
    }
    fn matches(&self, other: &WeightOfSubtree) -> bool {
        self.total() == other.total()
    }
}

struct Weights {
    weights: Vec<WeightOfSubtree>,
}

impl Weights {
    fn from(names: ChildrenNames, weights: &WeightByName) -> Self {
        let weights = names
            .iter()
            .filter_map(|child| weights.get(child))
            .cloned()
            .collect::<Vec<_>>();
        Weights { weights }
    }
    fn first_node(&self) -> &WeightOfSubtree {
        self.weights.first().unwrap()
    }
    fn number_of_nodes_matching_reference_weight(&self) -> usize {
        let reference = self.first_node();
        self.weights
            .iter()
            .filter(|own| own.matches(reference))
            .count()
    }
    fn are_all_equal(&self) -> bool {
        let reference = self.first_node();
        self.weights.iter().all(|own| own.matches(reference))
    }
    fn sum(&self) -> Weight {
        self.weights.iter().map(WeightOfSubtree::total).sum()
    }
    fn corrected_weight_of_node(&self) -> Weight {
        let reference = self.first_node();
        let number_of_nodes_matching_ref_weight = self.number_of_nodes_matching_reference_weight();
        let other = self.first_node_unlike(reference);
        if number_of_nodes_matching_ref_weight == 1 {
            // the reference's parent needs correcting
            other.parent + other.children - reference.children
        } else {
            // another node's parent needs correcting
            reference.parent + reference.children - other.children
        }
    }
    fn first_node_unlike(&self, reference: &WeightOfSubtree) -> &WeightOfSubtree {
        self.weights
            .iter()
            .find(|other| !other.matches(reference))
            .unwrap()
    }
}

fn fixed_weight_of_imbalancing_program(lines: Vec<String>) -> Weight {
    let (mut weights, mut connections) = parse_weights_and_parent_child_connections(lines);

    while !connections.is_empty() {
        let mut leaf_nodes = find_leaves(&mut connections);
        for (parent, children) in leaf_nodes.drain(..) {
            connections.remove(&parent);
            let children_weights = Weights::from(children, &weights);
            if children_weights.are_all_equal() {
                let weight = weights.get_mut(&parent).unwrap();
                weight.children = children_weights.sum()
            } else {
                return children_weights.corrected_weight_of_node();
            }
        }
    }
    unreachable!()
}

fn find_leaves(children_by_parent: &mut ChildrenNamesByParentName) -> Vec<Connection> {
    children_by_parent
        .iter()
        .filter(|(_parent, children)| {
            !children
                .iter()
                .any(|child| children_by_parent.contains_key(child))
        })
        .map(|(parent, children)| (parent.clone(), children.clone()))
        .collect::<Vec<_>>()
}

fn parse_weights_and_parent_child_connections(
    lines: Vec<String>,
) -> (WeightByName, ChildrenNamesByParentName) {
    let mut weights = HashMap::new();
    let mut children_by_parent = HashMap::new();
    lines.iter().for_each(|line| {
        if let Some((parent, children)) = line.split_once(" -> ") {
            let (parent, weight) = name_and_weight_from(parent);
            weights.insert(parent.clone(), WeightOfSubtree::new(weight));
            let children: HashSet<_> = children.split(", ").map(|n| n.to_string()).collect();
            children_by_parent.insert(parent, children);
        } else {
            let (parent, weight) = name_and_weight_from(line);
            weights.insert(parent, WeightOfSubtree::new(weight));
        }
    });
    (weights, children_by_parent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn example_part1() {
        assert_eq!("tknk", root_node(read_str_to_lines(EXAMPLE)));
    }

    #[test]
    fn part1() {
        assert_eq!("eqgvf", day7_part1());
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            60,
            fixed_weight_of_imbalancing_program(read_str_to_lines(EXAMPLE))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(757, day7_part2());
    }
}
