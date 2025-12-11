use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../2025/input/day11.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let map = parse(input);
    count_paths(&map, "you", "out", HashSet::new())
}

fn solve_part2(input: &str) -> usize {
    let successor_map: HashMap<_, _> = parse(input);
    let predecessor_map = build_predecessor_map(&successor_map);

    let hubs: Vec<_> = predecessor_map
        .iter()
        .filter_map(|(node, predecessors)| {
            (predecessors.len() > 1).then_some(format!("{node}: {}", predecessors.len()))
        })
        .collect();

    // only in keys: svr
    // only in values: out

    let svr = "svr";
    let fft = "fft";
    let dac = "dac";
    let out = "out";

    // Due to runtimes I assume that 'fft' is before 'dac' in the input as well
    // Thus the interesting paths are 'svr' -> 'fft' -> 'dac' -> 'out'

    // Let's count the paths from 'fft' *back* to the starting 'svr'
    // (Searching backwards avoids traversing the whole graph)
    let svr_to_fft_counter = count_paths(&predecessor_map, fft, svr, HashSet::from([fft]));
    dbg!(svr_to_fft_counter);

    // Counting the paths from 'dac' to 'out' is similarly straightforward
    let dac_to_out_counter = count_paths(&successor_map, dac, out, HashSet::from([fft]));
    dbg!(dac_to_out_counter);

    // The tricky bit is between 'fft' and 'dac'. When trying to go from one to the other,
    // it's easy to miss it and traversing the whole graph instead.
    // To stop this as early as possible, determine all the successors of 'dac' first,
    // then when traversing from 'fft' to 'dac', stop the search when encountering any of them.
    let successors_of_dac = list_nodes(&successor_map, dac, out);
    dbg!(successors_of_dac.len());

    // fft -> dac is very slow
    let fft_to_dac_counter = count_paths(&successor_map, fft, dac, successors_of_dac);
    dbg!(fft_to_dac_counter);
    svr_to_fft_counter * fft_to_dac_counter * dac_to_out_counter
}

fn build_predecessor_map<'a>(
    successor_map: &HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let successors: HashSet<_> = successor_map.values().flatten().cloned().collect();
    successors
        .into_iter()
        .map(|successor| (successor, direct_predecessors_of(successor, successor_map)))
        .collect()
}

fn direct_predecessors_of<'a>(
    node: &str,
    successor_map: &HashMap<&'a str, Vec<&str>>,
) -> Vec<&'a str> {
    successor_map
        .iter()
        .filter_map(|(source, targets)| targets.contains(&node).then_some(*source))
        .collect::<Vec<_>>()
}

// fn compress<'a>(mut map: HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {
//     let mut compressed = HashMap::new();
//     for (source, targets) in map {
//         if targets.len() == 1 {
//
//         } else {
//             compressed.insert(source, targets.clone());
//         }
//     }
//
//     compressed
// }

fn count_paths<'a>(
    connection_map: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str,
    skip: HashSet<&'a str>,
) -> usize {
    println!("Counting paths from '{start}' to '{end}'");
    let mut queue: Vec<_> = connection_map[start].clone();

    let mut end_reached_counter = 0;
    while let Some(curr) = queue.pop() {
        if let Some(nexts) = connection_map.get(curr) {
            for next in nexts {
                if skip.contains(next) {
                    continue;
                }
                if next == &end {
                    end_reached_counter += 1;
                } else if queue.contains(next) {
                    println!("Queue already contains '{next:?}'");
                } else {
                    queue.push(next);
                }
            }
        }
    }
    end_reached_counter
}

fn list_nodes<'a>(
    connection_map: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str,
) -> HashSet<&'a str> {
    let mut queue: Vec<_> = connection_map[start].clone();
    let mut visited = HashSet::new();

    while let Some(curr) = queue.pop() {
        if visited.contains(curr) {
            continue;
        }
        visited.insert(curr);
        if let Some(nexts) = connection_map.get(curr) {
            for next in nexts {
                if next == &end {
                    visited.insert(next);
                    continue;
                } else if queue.contains(next) {
                    println!("Queue already contains {next:?}");
                } else {
                    queue.push(next);
                }
            }
        }
    }
    visited
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (source, target) = line.split_once(": ").unwrap();
            let targets = target.split_whitespace().collect::<Vec<_>>();
            (source, targets)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const EXAMPLE2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_part1_example() {
        assert_eq!(5, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(607, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2, solve_part2(EXAMPLE2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(506264456238938, solve_part2(INPUT));
    }
}
