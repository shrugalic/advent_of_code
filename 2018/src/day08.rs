use crate::parse;

type Entry = usize;
type NodeCount = Entry;
type DataCount = Entry;
type Metadata = Entry;
type ChildValue = Entry;

const INPUT: &str = include_str!("../input/day08.txt");

pub(crate) fn day8_part1() -> Metadata {
    input_metadata_sum(parse(INPUT)[0])
}

pub(crate) fn day8_part2() -> Metadata {
    input_value(parse(INPUT)[0])
}

fn input_metadata_sum(input: &str) -> Metadata {
    let entries: Vec<Entry> = to_vec(input);
    entries_metadata_sum(&entries, 1)
}

fn to_vec(input: &str) -> Vec<Entry> {
    input.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn entries_metadata_sum(entries: &[Entry], sibling_count: NodeCount) -> Metadata {
    let (sum, tail) = tree_metadata_sum(&entries, sibling_count);
    assert!(tail.is_empty());
    sum
}

fn tree_metadata_sum(entries: &[Entry], sibling_count: NodeCount) -> (Metadata, &[Entry]) {
    // sibling_count describes the remaining number of nodes on this level of the tree
    if sibling_count == 0 {
        return (0, &entries);
    }
    let (header, tail) = entries.split_at(2);
    let (child_count, data_len): (NodeCount, DataCount) = (header[0], header[1]);

    let (children_sum, tail) = tree_metadata_sum(tail, child_count);
    let (own_sum, tail) = node_metadata_sum(data_len, tail);
    let (sibling_sum, tail) = tree_metadata_sum(&tail, sibling_count - 1);

    (children_sum + own_sum + sibling_sum, &tail)
}

fn node_metadata_sum(data_len: DataCount, tail: &[Entry]) -> (Metadata, &[Entry]) {
    let (metadata, tail) = tail.split_at(data_len);
    (metadata.iter().sum(), tail)
}

fn input_value(input: &str) -> Metadata {
    let entries: Vec<Entry> = to_vec(input);
    entries_value(&entries, 1)
}

fn entries_value(entries: &[Entry], sibling_count: NodeCount) -> Metadata {
    let (child_values, tail) = tree_value(&entries, sibling_count);
    assert!(tail.is_empty());
    assert_eq!(child_values.len(), 1);
    child_values[0]
}

fn tree_value(entries: &[Entry], sibling_count: NodeCount) -> (Vec<ChildValue>, &[Entry]) {
    // sibling_count describes the remaining number of nodes on this level of the tree
    if sibling_count == 0 {
        return (vec![], &entries);
    }
    let (header, tail) = entries.split_at(2);
    let (child_count, data_len): (NodeCount, DataCount) = (header[0], header[1]);

    let (children_values, tail) = tree_value(tail, child_count);
    let (metadata, tail) = tail.split_at(data_len);
    let own_value = if child_count == 0 {
        metadata.iter().sum()
    } else {
        metadata
            .iter()
            .filter_map(|md_idx| children_values.get(md_idx - 1))
            .sum()
    };
    let (mut sibling_values, tail) = tree_value(&tail, sibling_count - 1);
    sibling_values.insert(0, own_value);

    (sibling_values, &tail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE_1: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn example_1() {
        assert_eq!(138, input_metadata_sum(EXAMPLE_1));
    }

    #[test]
    fn single_node() {
        assert_eq!(99, input_metadata_sum("0 1 99"));
    }
    #[test]
    fn two_vertical_nodes() {
        assert_eq!(101, input_metadata_sum("1 1 0 1 99 2"));
    }

    #[test]
    fn two_side_by_side_nodes() {
        assert_eq!(35, entries_metadata_sum(&to_vec("0 3 10 11 12 0 1 2"), 2));
    }

    #[test]
    fn two_side_by_side_nodes_and_one_below_the_right() {
        assert_eq!(
            134,
            entries_metadata_sum(&to_vec("0 3 10 11 12 1 1 0 1 99 2"), 2)
        );
    }

    #[test]
    fn two_side_by_side_nodes_and_one_below_the_left() {
        assert_eq!(
            134,
            entries_metadata_sum(&to_vec("1 3 0 1 99 10 11 12 0 1 2"), 2)
        );
    }

    #[test]
    fn two_side_by_side_nodes_and_one_below_each() {
        assert_eq!(
            183,
            entries_metadata_sum(&to_vec("1 3 0 1 49 10 11 12 1 1 0 1 99 2"), 2)
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(42146, input_metadata_sum(&parse(INPUT)[0]));
    }

    #[test]
    fn example_1_part_2() {
        assert_eq!(66, input_value(EXAMPLE_1));
    }

    #[test]
    fn part_2() {
        assert_eq!(26753, input_value(&parse(INPUT)[0]));
    }
}
