use itertools::Itertools;
use line_reader::read_file_to_lines;

pub(crate) fn day24_part1() -> usize {
    let weights = parse_weights(read_file_to_lines("input/day24.txt"));
    smallest_groups(&weights, 3).unwrap()
}

pub(crate) fn day24_part2() -> usize {
    let weights = parse_weights(read_file_to_lines("input/day24.txt"));
    smallest_groups(&weights, 4).unwrap()
}

trait MinimumQuantumEntanglement {
    fn min_quantum_entanglement(&self) -> usize;
}
impl MinimumQuantumEntanglement for Vec<Vec<Weight>> {
    fn min_quantum_entanglement(&self) -> usize {
        self.iter()
            .map(|group| group.iter().product())
            .min()
            .unwrap()
    }
}

type Weight = usize;
fn parse_weights(input: Vec<String>) -> Vec<Weight> {
    input.into_iter().map(|l| l.parse().unwrap()).collect()
}

fn smallest_groups(weights: &[Weight], n: usize) -> Option<usize> {
    let target_weight = weights.iter().sum::<Weight>() / n;

    for len in 1..=(weights.len() / n) {
        if let Some(qe) = weights
            .iter()
            .combinations(len)
            .filter(|combo| combo.iter().cloned().sum::<usize>() == target_weight)
            // This filter checks that the rest can be further divided into groups of the same weight.
            // This takes 65s for part one (about 64s extra!) or 35s for part 2(34s extra!),
            // and yields the same result as without this extra check
            /*
            .filter(|combo| {
                n == 2 // if there are only 2 groups left, the remainder is already a valid group
                    || smallest_groups(
                        // else try to split the remaining weights into valid groups
                        &weights
                            .iter()
                            .filter(|w| !combo.contains(w))
                            .cloned()
                            .collect::<Vec<_>>(),
                        n - 1,
                    )
                    .is_some()
            })
             */
            .map(|combo| combo.iter().cloned().product::<usize>())
            .min()
        {
            return Some(qe);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
1
2
3
4
5
7
8
9
10
11";

    #[test]
    fn parse_part1_example() {
        let weights = parse_weights(read_str_to_lines(EXAMPLE));
        assert_eq!(vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11], weights);
    }

    #[test]
    fn part1_example1() {
        let weights = parse_weights(read_str_to_lines(EXAMPLE));
        assert_eq!(99, smallest_groups(&weights, 3).unwrap());
    }
    #[test]
    fn part1() {
        assert_eq!(11_846_773_891, day24_part1());
    }

    #[test]
    fn part2_example1() {
        let weights = parse_weights(read_str_to_lines(EXAMPLE));
        assert_eq!(44, smallest_groups(&weights, 4).unwrap());
    }

    #[test]
    fn part2() {
        assert_eq!(80_393_059, day24_part2());
    }
}
