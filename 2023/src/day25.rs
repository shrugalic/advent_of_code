#[allow(unused_imports)]
use petgraph::{algo, Graph};
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day25.txt");

pub(crate) fn part1() -> usize {
    solve_part1_with_graphviz(INPUT)
}

type Node<'a> = &'a str;
type EdgeIdx = usize;
type NodeIdx = usize;
type Edge = (EdgeIdx, EdgeIdx);

fn solve_part1_with_graphviz(input: &str) -> usize {
    let (nodes, edges) = parse(input);
    let node_by_index: Vec<Node> = nodes.into_iter().collect();
    let index_by_node: HashMap<Node, NodeIdx> = node_by_index
        .iter()
        .enumerate()
        .map(|(i, &node)| (node, i))
        .collect();
    let mut flattened_edges: Vec<Edge> = edges
        .into_iter()
        .flat_map(|(from, tos)| {
            tos.into_iter()
                .map(|to| (index_by_node[from], index_by_node[to]))
        })
        .collect();
    flattened_edges.sort_unstable();
    flattened_edges.reverse();
    let mut connections: HashMap<NodeIdx, HashSet<NodeIdx>> = HashMap::new();
    for (from, to) in &flattened_edges {
        connections.entry(*from).or_default().insert(*to);
        connections.entry(*to).or_default().insert(*from);
    }
    let total_node_count = node_by_index.len();

    let mut edges_to_ignore = HashSet::new();
    let sqh = index_by_node["sqh"];
    let jbz = index_by_node["jbz"];
    let nvg = index_by_node["nvg"];
    let vfj = index_by_node["vfj"];
    let fch = index_by_node["fch"];
    let fvh = index_by_node["fvh"];
    let edge1 = (sqh, jbz);
    let edge2 = (nvg, vfj);
    let edge3 = (fch, fvh);
    edges_to_ignore.insert(&edge1);
    edges_to_ignore.insert(&edge2);
    edges_to_ignore.insert(&edge3);
    if let Some((left_size, right_size)) =
        find_graph_parts_sizes(total_node_count, &connections, &edges_to_ignore)
    {
        return left_size * right_size;
    }

    unreachable!()
}

#[cfg(test)]
fn solve_part1_brute_force_home_brew(input: &str) -> usize {
    let (nodes, edges) = parse(input);
    // let the index be the node index
    let node_by_index: Vec<Node> = nodes.into_iter().collect();
    let index_by_node: HashMap<Node, NodeIdx> = node_by_index
        .iter()
        .enumerate()
        .map(|(i, &node)| (node, i))
        .collect();

    let mut flattened_edges: Vec<Edge> = edges
        .into_iter()
        .flat_map(|(from, tos)| {
            tos.into_iter()
                .map(|to| (index_by_node[from], index_by_node[to]))
        })
        .collect();
    flattened_edges.sort_unstable();
    flattened_edges.reverse();

    let mut connections: HashMap<NodeIdx, HashSet<NodeIdx>> = HashMap::new();
    for (from, to) in &flattened_edges {
        connections.entry(*from).or_default().insert(*to);
        connections.entry(*to).or_default().insert(*from);
    }

    let total_node_count = node_by_index.len();

    if true {
        // Test to see if example can be solved
        let mut edges_to_ignore = HashSet::new();
        let cmg = index_by_node["cmg"];
        let bvb = index_by_node["bvb"];
        let jqt = index_by_node["jqt"];
        let nvd = index_by_node["nvd"];
        let pzl = index_by_node["pzl"];
        let hfx = index_by_node["hfx"];
        let edge1 = (cmg, bvb);
        let edge2 = (jqt, nvd);
        let edge3 = (pzl, hfx);
        edges_to_ignore.insert(&edge1);
        edges_to_ignore.insert(&edge2);
        edges_to_ignore.insert(&edge3);
        if let Some((left_size, right_size)) =
            find_graph_parts_sizes(total_node_count, &connections, &edges_to_ignore)
        {
            // println!("last edge from {from} to {to}");
            return left_size * right_size;
        }
    }

    let mut edges_to_ignore = HashSet::new();
    for (i1, edge1) in flattened_edges.iter().enumerate() {
        edges_to_ignore.insert(edge1);
        for (i2, edge2) in flattened_edges.iter().enumerate().skip(i1 + 1) {
            edges_to_ignore.insert(edge2);
            for edge3 in flattened_edges.iter().skip(i2 + 1) {
                edges_to_ignore.insert(edge3);
                // check what happens to the graph with edges 1 to 3 removed
                if let Some((left_size, right_size)) =
                    find_graph_parts_sizes(total_node_count, &connections, &edges_to_ignore)
                {
                    // println!("last edge from {from} to {to}");
                    return left_size * right_size;
                }
                edges_to_ignore.remove(edge3);
            }
            edges_to_ignore.remove(edge2);
        }
        edges_to_ignore.remove(edge1);
    }
    unreachable!()
}

fn find_graph_parts_sizes(
    total_node_count: usize,
    connections: &HashMap<NodeIdx, HashSet<NodeIdx>>,
    edges_to_ignore: &HashSet<&Edge>,
) -> Option<(usize, usize)> {
    let (left_node, right_node) = edges_to_ignore.iter().next().unwrap();
    count_reachable_nodes(*left_node, total_node_count, connections, edges_to_ignore).and_then(
        |left_size| {
            count_reachable_nodes(*right_node, total_node_count, connections, edges_to_ignore)
                .map(|right_size| (left_size, right_size))
        },
    )
}

fn count_reachable_nodes(
    start: NodeIdx,
    total_node_count: usize,
    connections: &HashMap<NodeIdx, HashSet<NodeIdx>>,
    edges_to_ignore: &HashSet<&Edge>,
) -> Option<usize> {
    let mut visited: HashSet<NodeIdx> = HashSet::new();
    let mut queue = vec![start];
    while let Some(node) = queue.pop() {
        visited.insert(node);
        if visited.len() == total_node_count {
            return None;
        }
        let next_nodes: Vec<_> = connections[&node]
            .iter()
            .filter(|&to| {
                let visited = visited.contains(to);
                let ignore_from = edges_to_ignore.contains(&(node, *to));
                let ignore_reverse = edges_to_ignore.contains(&(*to, node));
                let queued = queue.contains(to);
                !visited && !ignore_from && !ignore_reverse && !queued
            })
            .collect();
        queue.extend(next_nodes);
    }

    println!("Reached {} nodes from {start}", visited.len());
    Some(visited.len())
}

fn parse(input: &str) -> (HashSet<&str>, HashMap<&str, HashSet<&str>>) {
    let mut nodes: HashSet<&str> = HashSet::new();
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.trim().lines() {
        let (from, right) = line.split_once(": ").unwrap();
        nodes.insert(from);
        for to in right.split_ascii_whitespace() {
            nodes.insert(to);
            edges.entry(from).or_default().insert(to);
            // edges.entry(to).or_default().insert(from);
        }
    }
    (nodes, edges)
}

#[cfg(test)]
fn solve_part1_brute_force_with_petgraph(input: &str) -> usize {
    let (nodes, edges) = parse(input);
    // graph
    let mut graph: Graph<&str, ()> = Graph::new();
    // nodes
    let index_by_node: HashMap<_, _> = nodes
        .iter()
        .map(|node| (*node, graph.add_node(node)))
        .collect();
    let node_by_index: HashMap<_, _> = index_by_node
        .iter()
        .map(|(node, index)| (*index, *node))
        .collect();
    // edges
    let index_by_edge: HashMap<_, _> = edges
        .iter()
        .flat_map(|(from, tos)| tos.iter().map(|to| (*from, *to)))
        .map(|(from, to)| (index_by_node[from], index_by_node[to]))
        .map(|edge @ (from, to)| (edge, graph.add_edge(from, to, ())))
        .collect();
    let edge_by_index: HashMap<_, _> = index_by_edge
        .iter()
        .map(|(edge, index)| (*index, *edge))
        .collect();
    let edge_indices: Vec<_> = index_by_edge.values().cloned().collect();

    // Print graph as dot
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // brute force: remove 3 nodes and see if the graph is split into 2 parts
    for (i, edge1) in edge_indices.iter().enumerate() {
        let mut removed_1 = graph.clone();
        removed_1.remove_edge(*edge1);
        if algo::connected_components(&removed_1) != 1 {
            continue;
        }
        for (k, edge2) in edge_indices[i + 1..].iter().enumerate() {
            let mut removed_2 = removed_1.clone();
            removed_2.remove_edge(*edge2);
            if algo::connected_components(&removed_2) != 1 {
                continue;
            }
            for edge3 in edge_indices[k + 1..].iter() {
                let mut removed_3 = removed_2.clone();
                removed_3.remove_edge(*edge3);
                if algo::connected_components(&removed_3) == 2 {
                    let pair1 = edge_by_index[edge1];
                    let pair1 = (node_by_index[&pair1.0], node_by_index[&pair1.1]);
                    let pair2 = edge_by_index[edge2];
                    let pair2 = (node_by_index[&pair2.0], node_by_index[&pair2.1]);
                    let pair3 = edge_by_index[edge3];
                    let pair3 = (node_by_index[&pair3.0], node_by_index[&pair3.1]);
                    let edges: HashSet<_> = edges
                        .into_iter()
                        .flat_map(|(from, tos)| tos.into_iter().map(move |to| (from, to)))
                        .filter(|pair| {
                            pair != &(pair1.0, pair1.1)
                                && pair != &(pair1.1, pair1.0)
                                && pair != &(pair2.0, pair2.1)
                                && pair != &(pair2.1, pair2.0)
                                && pair != &(pair3.0, pair3.1)
                                && pair != &(pair3.1, pair3.0)
                        })
                        .collect();
                    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
                    for (from, to) in edges {
                        connections.entry(from).or_default().insert(to);
                        connections.entry(to).or_default().insert(from);
                    }

                    let from_count = visit(&mut connections, pair3.0);
                    let to_count = visit(&mut connections, pair3.1);

                    println!("last edge from {} to {}", pair3.0, pair3.1);
                    return from_count * to_count;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
fn visit<'a>(connections: &mut HashMap<&'a str, HashSet<&'a str>>, start: &'a str) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![start];
    while let Some(node) = queue.pop() {
        visited.insert(node);
        queue.extend(
            connections
                .entry(node)
                .or_default()
                .iter()
                .filter(|next| !visited.contains(**next)),
        );
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn test_part1_example_brute_force_home_brew() {
        assert_eq!(6 * 9, solve_part1_brute_force_home_brew(EXAMPLE));
    }

    #[test]
    fn test_part1_example_brute_force_with_petgraph() {
        assert_eq!(6 * 9, solve_part1_brute_force_with_petgraph(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(705 * 776 /*547_080*/, solve_part1_with_graphviz(INPUT));
    }
}
