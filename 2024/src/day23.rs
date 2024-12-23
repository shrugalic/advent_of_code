use std::collections::{BTreeSet, HashMap, HashSet};

const INPUT: &str = include_str!("../../2024/input/day23.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> String {
    solve_part2(INPUT)
}

type Name<'a> = &'a str;
// Using BTreeSet because HashSet<HashSet<&str>> is not allowed,
// because HashSet<&str> does not impl Hash
type Tuple<'a> = BTreeSet<Name<'a>>;
type TupleSet<'a> = HashSet<Tuple<'a>>;

// ~9 ms
fn solve_part1(input: &str) -> usize {
    let pairs = parse_pairs_into_tuple_set(input);
    let connections = parse_pairs_into_connections(input);
    let triples = increase_connected_sets_by_one(pairs, &connections);
    triples
        .iter()
        .filter(|triple| triple.iter().any(|c| c.starts_with('t')))
        .count()
}

// ~16 ms
pub fn solve_part1_following_connections(input: &str) -> usize {
    let connections: HashMap<Name, Tuple> = parse_pairs_into_connections(input);
    let mut triples: TupleSet = TupleSet::new();
    for (source, targets) in connections
        .iter()
        .filter(|(_src, targets)| targets.len() >= 2)
    {
        // source connects to targets. If two targets are themselves connected, they form a triple
        for a in targets {
            for b in targets.iter().filter(|b| b != &a) {
                if connections[a].contains(b) {
                    triples.insert([source, a, b].into_iter().cloned().collect());
                }
            }
        }
    }
    triples
        .iter()
        .filter(|triple| triple.iter().any(|c| c.starts_with('t')))
        .count()
}

// ~1.06s
fn solve_part2(input: &str) -> String {
    let mut current_sets = parse_pairs_into_tuple_set(input);
    let connections: HashMap<Name, Tuple> = parse_pairs_into_connections(input);
    while current_sets.len() > 1 {
        let next_sets = increase_connected_sets_by_one(current_sets, &connections);
        current_sets = next_sets;
    }
    let mut names: Vec<_> = current_sets
        .into_iter()
        .next()
        .unwrap()
        .into_iter()
        .collect();
    names.sort_unstable();
    names.join(",")
}

// ~1.1 ms
pub fn solve_part2_andre_optimized(input: &str) -> String {
    // To avoid using HashMaps as much, this translates the strings to indices
    let (idx_to_str, connections) = parse_pairs_into_connections2(input);

    let mut largest_set = BTreeSet::new();
    for (seed_node, targets) in connections.iter().enumerate() {
        if targets.len() < largest_set.len() {
            // Unnecessary optimization: Don't even bother with sparse nodes
            continue;
        }
        let mut set = BTreeSet::from([seed_node]);

        for candidate in targets {
            if set
                .iter()
                .all(|node| connections[*candidate].contains(node))
            {
                set.insert(*candidate);
            }
        }
        if set.len() > largest_set.len() {
            largest_set = set;
        }
    }

    let names = largest_set
        .into_iter()
        .map(|i| idx_to_str[i])
        .collect::<Vec<&str>>();
    names.join(",")
}

// ~2 ms
pub fn solve_part2_andre_orig(input: &str) -> String {
    let connections: HashMap<Name, Tuple> = parse_pairs_into_connections(input);

    let mut largest_set = Tuple::new();
    for (seed_node, targets) in &connections {
        if targets.len() < largest_set.len() {
            // Unnecessary optimization: Don't even bother with sparse nodes
            continue;
        }
        let mut set = Tuple::from([*seed_node]);

        for candidate in targets {
            if set.iter().all(|node| connections[candidate].contains(node)) {
                set.insert(*candidate);
            }
        }
        if set.len() > largest_set.len() {
            largest_set = set;
        }
    }

    let names: Vec<_> = largest_set.into_iter().collect();
    names.join(",")
}

fn increase_connected_sets_by_one<'a>(
    current_sets: TupleSet<'a>,
    connections: &'a HashMap<Name, Tuple>,
) -> TupleSet<'a> {
    let mut next_sets = TupleSet::new();
    for current_set in current_sets {
        let mut it = current_set.iter();
        let first_name = it.next().cloned().unwrap();
        let mut common_connections = connections[first_name].clone();
        for name in it {
            common_connections = common_connections
                .intersection(&connections[name])
                .cloned()
                .collect();
            if common_connections.is_empty() {
                break;
            }
        }
        for common_connection in common_connections {
            let mut next_set = current_set.clone();
            next_set.insert(common_connection);
            next_sets.insert(next_set);
        }
    }
    next_sets
}

fn parse_pairs_into_connections(input: &str) -> HashMap<Name, Tuple> {
    let mut connections: HashMap<Name, Tuple> = HashMap::new();
    for (a, b) in input.trim().lines().filter_map(|line| line.split_once('-')) {
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }
    connections
}

fn parse_pairs_into_connections2(input: &str) -> (Vec<&str>, Vec<Vec<usize>>) {
    let pairs: Vec<_> = input
        .trim()
        .lines()
        .filter_map(|line| line.split_once('-'))
        .collect();

    // To avoid using HashMaps as much, translate the strings to indices
    let mut idx_to_str: Vec<_> = pairs.iter().flat_map(|(a, b)| [a, b]).cloned().collect();
    idx_to_str.sort_unstable(); // dedup needs the entries to be sorted
    idx_to_str.dedup();

    // This HashMap to initially translate the strings to indices I was unable to avoid.
    // Using the `idx_to_str` vec directly, with .iter().position(…) lookups is way slower,
    // and using a BTreeMap is also slightly slower
    let str_to_idx: HashMap<&str, usize> = idx_to_str
        .iter()
        .enumerate()
        .map(|(i, &s)| (s, i))
        .collect();

    let mut connections: Vec<Vec<usize>> = vec![vec![]; idx_to_str.len()];
    for (a_str, b_str) in pairs {
        let a = str_to_idx[a_str];
        let b = str_to_idx[b_str];
        connections[a].push(b);
        connections[b].push(a);
    }
    (idx_to_str, connections)
}

fn parse_pairs_into_tuple_set(input: &str) -> TupleSet {
    input
        .trim()
        .lines()
        .map(|line| line.split('-').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn test_part1_example() {
        assert_eq!(7, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1_248, solve_part1(INPUT));
    }

    #[test]
    fn test_part1_following_connections() {
        assert_eq!(1_248, solve_part1_following_connections(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!("co,de,ka,ta", solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!("aa,cf,cj,cv,dr,gj,iu,jh,oy,qr,xr,xy,zb", solve_part2(INPUT));
    }

    #[test]
    fn test_part2_andre_optimized() {
        assert_eq!(
            "aa,cf,cj,cv,dr,gj,iu,jh,oy,qr,xr,xy,zb",
            solve_part2_andre_optimized(INPUT)
        );
    }

    #[test]
    fn test_part2_andre_orig() {
        assert_eq!(
            "aa,cf,cj,cv,dr,gj,iu,jh,oy,qr,xr,xy,zb",
            solve_part2_andre_orig(INPUT)
        );
    }
}
