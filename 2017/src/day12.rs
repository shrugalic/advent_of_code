use crate::parse;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day12.txt");

pub(crate) fn day12_part1() -> usize {
    count_programs_in_group0(parse(INPUT))
}

pub(crate) fn day12_part2() -> usize {
    count_total_groups(parse(INPUT))
}

fn count_programs_in_group0(input: Vec<&str>) -> usize {
    count_group0_members_or_total_groups(input, true)
}

fn count_total_groups(input: Vec<&str>) -> usize {
    count_group0_members_or_total_groups(input, false)
}

fn count_group0_members_or_total_groups(input: Vec<&str>, get_group0_only: bool) -> usize {
    let connections = parse_connections(input);
    let mut group_size_by_root_node = HashMap::new();

    let mut pool: HashSet<_> = connections.keys().collect();
    while let Some(&min) = pool.iter().min() {
        let root = pool.take(&min).unwrap();
        let group = group_connected_to(root, &connections);

        if *root == 0 && get_group0_only {
            return group.len();
        }

        group_size_by_root_node.insert(root, group.len());
        pool = pool.difference(&group).cloned().collect();
    }
    group_size_by_root_node.len()
}

fn group_connected_to<'a>(
    root: &'a usize,
    connections: &'a HashMap<usize, HashSet<usize>>,
) -> HashSet<&'a usize> {
    let mut group = HashSet::new();
    let mut queue = vec![root];
    while let Some(num) = queue.pop() {
        if group.insert(num) {
            queue.extend(connections.get(&num).unwrap());
        }
    }
    group
}

fn parse_connections(input: Vec<&str>) -> HashMap<usize, HashSet<usize>> {
    let mut connections = HashMap::new();
    for line in input {
        let (source, destinations) = line.split_once(" <-> ").unwrap();
        let source: usize = source.parse().unwrap();
        let destinations: HashSet<usize> = destinations
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        connections.insert(source, destinations);
    }
    connections
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE1: &str = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn part1_example() {
        assert_eq!(6, count_programs_in_group0(parse(EXAMPLE1)));
    }

    #[test]
    fn part1_full() {
        assert_eq!(239, day12_part1());
    }

    #[test]
    fn part2_full() {
        assert_eq!(215, day12_part2());
    }
}
