use std::collections::HashSet;

const INPUT: &str = include_str!("../../2024/input/day23.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> String {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let pairs = parse(input);
    let triples = find_larger_tuples(&pairs, pairs.clone());
    triples
        .iter()
        .filter(|triple| triple.iter().any(|c| c.starts_with('t')))
        .count()
}

fn solve_part2(input: &str) -> String {
    let pairs = parse(input);
    let triples = find_larger_tuples(&pairs, pairs.clone());
    let mut current = triples;
    let mut next = find_larger_tuples(&pairs, current.clone());
    while !next.is_empty() {
        current = next;
        next = find_larger_tuples(&pairs, current.clone());
    }

    let mut nodes = current
        .into_iter()
        .flat_map(|s| s.into_iter())
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect::<Vec<&str>>();
    nodes.sort_unstable();
    nodes.join(",")
}

fn find_larger_tuples<'a>(
    pairs: &'a Vec<Vec<&'a str>>,
    current_tuples: Vec<Vec<&'a str>>,
) -> Vec<Vec<&'a str>> {
    let current_len = current_tuples.first().unwrap().len();
    let larger_tuples = current_tuples
        .into_iter()
        .flat_map(|current_tuple| {
            pairs.iter().filter_map(move |pair| {
                // The other part of this pair is a candidate that could enlarge this tuple by 1,
                // if there are pairs for it and the other parts of the current tuple
                let candidate = pair[1];
                (current_tuple[0] == pair[0]
                    && !current_tuple[1..].iter().any(|&other| other == candidate)
                    && can_extend_tuple_with(candidate, &current_tuple, pairs))
                .then_some({
                    let mut next = current_tuple.clone();
                    next.push(candidate);
                    next.sort_unstable();
                    next
                })
            })
        })
        .collect::<HashSet<Vec<&str>>>()
        .into_iter()
        .collect::<Vec<Vec<&str>>>();
    // println!(
    //     "{} tuples of length {}",
    //     larger_tuples.len(),
    //     current_len + 1
    // );
    // part 2:
    // 11011 tuples of length 3
    // 26455 tuples of length 4
    // 45045 tuples of length 5
    // 55770 tuples of length 6
    // 50622 tuples of length 7
    // 33462 tuples of length 8
    // 15730 tuples of length 9
    // 5005 tuples of length 10
    // 975 tuples of length 11
    // 91 tuples of length 12
    // 1 tuples of length 13
    // 0 tuples of length 14
    larger_tuples
}

fn can_extend_tuple_with(candidate: &str, tuple: &[&str], pairs: &[Vec<&str>]) -> bool {
    tuple
        .iter()
        .map(|&existing| {
            let mut pair = vec![candidate, existing];
            pair.sort_unstable();
            pair
        })
        .all(|pair| pairs.contains(&pair))
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut pair = line.split("-").collect::<Vec<&str>>();
            pair.sort_unstable();
            pair
        })
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
    fn test_part2_example() {
        assert_eq!("co,de,ka,ta", solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!("aa,cf,cj,cv,dr,gj,iu,jh,oy,qr,xr,xy,zb", solve_part2(INPUT));
    }
}
