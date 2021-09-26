use crate::permutation::generate_permutations_of_n_indices;
use line_reader::read_file_to_lines;

pub(crate) fn day13_part1() -> isize {
    let input = read_file_to_lines("input/day13.txt");
    let happiness_table = parse_family(input);
    find_optimal_happiness(happiness_table)
}

pub(crate) fn day13_part2() -> isize {
    let input = read_file_to_lines("input/day13.txt");
    let mut happiness_table = parse_family(input);
    happiness_table.push(vec![0; happiness_table.len()]);
    happiness_table.iter_mut().for_each(|h| h.push(0));
    find_optimal_happiness(happiness_table)
}

fn find_optimal_happiness(happiness: Vec<Vec<isize>>) -> isize {
    let len = happiness.len();
    generate_permutations_of_n_indices(len)
        .into_iter()
        .filter(|order| order[0] == 0)
        .map(|order| {
            (0..len)
                .into_iter()
                .map(|i| {
                    let center = order[i];
                    let left = order[(len + i - 1) % len];
                    let right = order[(i + 1) % len];
                    happiness[center][left] + happiness[center][right]
                })
                .sum::<isize>()
        })
        .max()
        .unwrap()
}

fn parse_family(input: Vec<String>) -> Vec<Vec<isize>> {
    // Vector of family members. This is only needed to get a unique index for each location
    let mut family: Vec<_> = vec![];
    // Happiness from each family member to all other family members (by index)
    let mut happiness: Vec<Vec<isize>> = vec![];

    for line in input {
        let parts: Vec<_> = line.split(|c| c == ' ' || c == '.').collect();

        // Example: Alice would gain 54 happiness units by sitting next to Bob.
        let center = parts[0].to_string();
        let sign = parts[3].parse::<isize>().unwrap() * if parts[2] == "gain" { 1 } else { -1 };
        let neighbor = parts[10].to_string();

        let mut get_family_mumber_id = |member| {
            if let Some(idx) = family.iter().position(|m| m == &member) {
                idx
            } else {
                family.push(member);
                happiness.iter_mut().for_each(|h| h.push(0));
                happiness.push(vec![0; family.len()]);
                family.len() - 1
            }
        };

        let center_id = get_family_mumber_id(center);
        let neighbor_id = get_family_mumber_id(neighbor);
        happiness[center_id][neighbor_id] = sign;
    }
    // println!("Family: {:?}", family);
    // println!("happiness: {:?}", happiness);
    happiness
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn part1_example() {
        let input = read_str_to_lines(EXAMPLE);
        let happiness_table = parse_family(input);
        assert_eq!(330, find_optimal_happiness(happiness_table));
    }

    #[test]
    fn part1() {
        assert_eq!(733, day13_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(725, day13_part2());
    }
}
