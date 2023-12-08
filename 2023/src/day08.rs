use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day08.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    solve(input, "AAA", "ZZZ")
}

fn solve_part2(input: &str) -> usize {
    solve(input, "A", "Z")
}

fn solve(input: &str, start_suffix: &str, end_suffix: &str) -> usize {
    let (instructions, node_map) = parse(input);
    // start from all matching nodes in parallel
    let mut current_nodes: Vec<&Node> = node_map
        .keys()
        .filter(|n| n.ends_with(start_suffix))
        .collect();
    let mut periods = vec![0usize; current_nodes.len()];
    let mut instructions = instructions.chars().cycle();
    for step_count in 1usize.. {
        let instruction = instructions.next().unwrap();
        for (i, current) in current_nodes.iter_mut().enumerate() {
            let (left, right) = node_map.get(*current).expect("node in nodes");
            let next = match instruction {
                'L' => left,
                'R' => right,
                _ => unreachable!("Illegal instruction {}", instruction),
            };
            *current = next;
            // update period whenever a node reached an ending position
            if current.ends_with(end_suffix) {
                periods[i] = step_count - periods[i];
            }
        }
        // when all periods are known, calculate the step count where they meet
        if periods.iter().all(|cycle_count| cycle_count > &0) {
            return least_common_multiple(periods);
        }
    }
    unreachable!()
}

type Instructions<'a> = &'a str;
type Node<'a> = &'a str;
fn parse(input: &str) -> (Instructions, HashMap<Node, (Node, Node)>) {
    let (instructions, nodes) = input
        .trim()
        .split_once("\n\n")
        .expect("an empty line somewhere");
    let node_map: HashMap<_, (_, _)> = nodes
        .lines()
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect();
    (instructions, node_map)
}

fn least_common_multiple(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let unique_prime_factors: HashSet<usize> =
        nums.iter().flat_map(|n| prime_factors(*n)).collect();
    // the product of the unique prime factors is a reasonable increment
    // when increasing the number to be tested as least common multiple
    let upf_product: usize = unique_prime_factors.into_iter().product();
    let mut lcm = upf_product;
    while !nums.iter().all(|num| upf_product % num == 0) {
        lcm += upf_product;
    }
    lcm
}

fn prime_factors(number: usize) -> Vec<usize> {
    let mut rem = number;
    let mut div = 2;
    let mut factors = vec![];
    while div < rem {
        while rem % div == 0 {
            factors.push(div);
            rem /= div;
        }
        if div == 2 {
            div += 1;
        } else {
            div += 2;
        }
    }
    factors.push(rem);
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE_1_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE_2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part1_example1() {
        assert_eq!(2, solve_part1(EXAMPLE_1_1));
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(6, solve_part1(EXAMPLE_1_2));
    }

    #[test]
    fn test_part1() {
        assert_eq!(16_531, solve_part1(INPUT));
    }

    #[test]
    fn test_least_common_multiple() {
        assert_eq!(
            least_common_multiple(vec![19241, 16531, 21409, 18157, 14363, 19783]),
            24_035_773_251_517
        );
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(2 * 3 * 5 * 7), vec![2, 3, 5, 7]);
        assert_eq!(prime_factors(71 * 271), vec![71, 271]);
        assert_eq!(prime_factors(61 * 271), vec![61, 271]);
        assert_eq!(prime_factors(79 * 271), vec![79, 271]);
        assert_eq!(prime_factors(67 * 271), vec![67, 271]);
        assert_eq!(prime_factors(53 * 271), vec![53, 271]);
        assert_eq!(prime_factors(73 * 271), vec![73, 271]);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(6, solve_part2(EXAMPLE_2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(24_035_773_251_517, solve_part2(INPUT));
    }
}
