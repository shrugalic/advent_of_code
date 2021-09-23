use line_reader::read_file_to_lines;

pub(crate) fn day09_part1() -> usize {
    shortest_route_length(read_file_to_lines("input/day09.txt"))
}

pub(crate) fn day09_part2() -> usize {
    longest_route_length(read_file_to_lines("input/day09.txt"))
}

fn shortest_route_length(input: Vec<String>) -> usize {
    route_lengths(input).into_iter().min().unwrap()
}
fn longest_route_length(input: Vec<String>) -> usize {
    route_lengths(input).into_iter().max().unwrap()
}
fn route_lengths(input: Vec<String>) -> Vec<usize> {
    let distances = parse_distances_from(input);
    generate_permutations_of_n_indices(distances.len())
        .into_iter()
        .map(|order| {
            order
                .windows(2)
                .into_iter()
                .map(|d| distances[d[0]][d[1]])
                .sum()
        })
        .collect()
}

fn generate_permutations_of_n_indices(size: usize) -> Vec<Vec<usize>> {
    let mut initial: Vec<_> = (0..size).into_iter().collect();
    let mut permutations: Vec<Vec<usize>> = vec![];
    generate_permutations(&mut permutations, initial.len(), &mut initial);
    permutations
}

// Heap's recursive permutation algorithm from
// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn generate_permutations(permutations: &mut Vec<Vec<usize>>, k: usize, a: &mut [usize]) {
    if k == 1 {
        permutations.push(a.to_vec());
    } else {
        // Generate permutations with kth unaltered
        // Initially k == length(A)
        generate_permutations(permutations, k - 1, a);

        // Generate permutations for kth swapped with each k-1 initial
        for i in 0..k - 1 {
            // Swap choice dependent on parity of k (even or odd)
            if k % 2 == 0 {
                a.swap(i, k - 1); // zero-indexed, the kth is at k-1
            } else {
                a.swap(0, k - 1);
            }
            generate_permutations(permutations, k - 1, a);
        }
    }
}

fn parse_distances_from(input: Vec<String>) -> Vec<Vec<usize>> {
    // Vector of location names. This is only needed to get a unique index for each location
    let mut locations: Vec<String> = vec![];
    // Distances from each location to all other locations (by index)
    let mut distances: Vec<Vec<usize>> = vec![];

    input.iter().for_each(|line| {
        let mut get_index_for_location = |loc| {
            if let Some(idx) = locations.iter().position(|l| l == &loc) {
                idx
            } else {
                locations.push(loc);
                distances.iter_mut().for_each(|d| d.push(usize::MAX));
                distances.push(vec![0; locations.len()]);
                locations.len() - 1
            }
        };
        // Example: London to Dublin = 464
        let split: Vec<_> = line.split_ascii_whitespace().collect();
        let (loc1, loc2, distance) = (split[0], split[2], split[4].parse().unwrap());
        let i1 = get_index_for_location(loc1.to_string());
        let i2 = get_index_for_location(loc2.to_string());
        distances[i1][i2] = distance;
        distances[i2][i1] = distance;
    });
    distances
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn part1_example() {
        assert_eq!(605, shortest_route_length(read_str_to_lines(EXAMPLE)));
    }

    #[test]
    fn part1() {
        assert_eq!(141, day09_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(982, longest_route_length(read_str_to_lines(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(736, day09_part2());
    }
}
