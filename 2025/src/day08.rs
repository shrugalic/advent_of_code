use crate::vec_3d::Vec3D;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../2025/input/day08.txt");

pub fn part1() -> usize {
    solve_part1(INPUT, 1000)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str, pair_count: usize) -> usize {
    let positions = parse_junction_box_positions(input);
    let (mut circuits, _) = solve(&positions, pair_count);

    circuits.sort_unstable_by_key(|circuit| -(circuit.len() as isize)); // Largest  first
    circuits
        .into_iter()
        .take(3)
        .map(|circuit| circuit.len())
        .product()
}

fn solve_part2(input: &str) -> usize {
    let positions = parse_junction_box_positions(input);
    let len = positions.len();
    let pair_count = len * (len - 1) / 2; // Number of *all* distinct pairs

    let (_, final_connection) = solve(&positions, pair_count);
    let (a, b) = final_connection.unwrap();
    (positions[a].x * positions[b].x) as usize
}

fn parse_junction_box_positions(input: &str) -> Vec<Vec3D> {
    input
        .trim()
        .lines()
        .map(|line| {
            let n: Vec<_> = line.split(',').map(|n| n.parse().unwrap()).collect();
            Vec3D::new(n[0], n[1], n[2])
        })
        .collect()
}

fn solve(positions: &[Vec3D], pair_count: usize) -> (Vec<HashSet<usize>>, Option<(usize, usize)>) {
    let ordered_position_index_pairs = order_into_pairs_by_ascending_distance(positions);

    // The first index is the circuit index, starting from 0.
    // The inner vec contains indices of positions.
    let mut circuits: Vec<HashSet<usize>> = vec![];

    let mut final_connection = None;
    for (a, b) in ordered_position_index_pairs.iter().take(pair_count) {
        let a_circuit_id = circuits
            .iter()
            .enumerate()
            .find_map(|(cid, circuit)| circuit.contains(a).then_some(cid));
        let b_circuit_cid = circuits
            .iter()
            .enumerate()
            .find_map(|(cid, circuit)| circuit.contains(b).then_some(cid));
        match (a_circuit_id, b_circuit_cid) {
            (None, None) => circuits.push(HashSet::from([*a, *b])), // Create a new circuit of two
            (Some(a_cid), None) => { circuits[a_cid].insert(*b); } // Add `b` to the circuit of a
            (None, Some(b_cid)) => { circuits[b_cid].insert(*a); } // Add `a` to the circuit of b
            (Some(a_cid), Some(b_cid)) if a_cid == b_cid => {} // Noop, they're already in the same circuit
            (Some(a_cid), Some(b_cid)) /* a_cid != b_cid */ => {
                // Merge circuit by moving elements in the circuit of `b` into the circuit of `a` …
                circuits[b_cid].clone().into_iter().for_each(|n| { circuits[a_cid].insert(n); });
                // … and then removing the now empty circuit
                circuits.remove(b_cid);
            }
        }
        if final_connection.is_none() && circuits[0].len() == positions.len() {
            final_connection = Some((*a, *b));
        }
    }
    (circuits, final_connection)
}

fn order_into_pairs_by_ascending_distance(positions: &[Vec3D]) -> Vec<(usize, usize)> {
    let len = positions.len();
    let mut distances: Vec<((usize, usize), usize)> = Vec::with_capacity(len * (len - 1));
    for a in 0..len - 1 {
        for b in a + 1..len {
            let pair = if a < b { (a, b) } else { (b, a) };
            let diff = positions[a] - positions[b];
            let distance_squared = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as usize;
            distances.push((pair, distance_squared));
        }
    }
    distances.sort_unstable_by_key(|(_, distance_squared)| *distance_squared);
    distances.into_iter().map(|(pair, _)| pair).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part1_example() {
        assert_eq!(5 * 4 * 2 /* = 40 */, solve_part1(EXAMPLE, 10));
    }

    #[test]
    fn test_part1() {
        // 733824 and 1072526 are too high
        assert_eq!(84968, solve_part1(INPUT, 1000));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(216 * 117 /* = 25272 */, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8663467782, solve_part2(INPUT));
    }
}
