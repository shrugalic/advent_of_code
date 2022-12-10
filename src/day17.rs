use crate::parse;

const INPUT: &str = include_str!("../input/day17.txt");

pub(crate) fn day17_part1() -> usize {
    let input = parse(INPUT);
    ways_to_fill_containers(input, TOTAL, Part::One)
}

pub(crate) fn day17_part2() -> usize {
    let input = parse(INPUT);
    ways_to_fill_containers(input, TOTAL, Part::Two)
}

const TOTAL: usize = 150;
enum Part {
    One,
    Two,
}
fn ways_to_fill_containers(input: Vec<&str>, total: usize, part: Part) -> usize {
    let mut containers = parse_containers(input);

    // Optimization: The containers are sorted by largest first, so that
    // aborting when sum > total happens with as few elements as possible.
    containers.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let combination_count = 2usize.pow(containers.len() as u32);
    let matches: Vec<_> = (0..combination_count)
        .into_iter()
        .filter_map(|combination_bitmask| {
            let mut sum = 0;
            let mut count = 0;
            for i in (0..containers.len())
                .into_iter()
                .filter(|i| (combination_bitmask >> i & 1) == 1)
            {
                count += 1;
                sum += containers[i];
                if sum > total {
                    return None;
                }
            }
            if sum == total {
                Some(count)
            } else {
                None
            }
        })
        .collect();

    match part {
        Part::One => matches.len(),
        Part::Two => {
            let min = matches.iter().min().unwrap();
            matches.iter().filter(|&v| v == min).count()
        }
    }
}

fn parse_containers(input: Vec<&str>) -> Vec<usize> {
    input
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "\
20
15
10
5
5";

    #[test]
    fn part1_example() {
        let input = parse(EXAMPLE);
        assert_eq!(4, ways_to_fill_containers(input, 25, Part::One));
    }
    #[test]
    fn part1() {
        assert_eq!(654, day17_part1());
    }

    #[test]
    fn part2_example() {
        let input = parse(EXAMPLE);
        assert_eq!(3, ways_to_fill_containers(input, 25, Part::Two));
    }

    #[test]
    fn part2() {
        assert_eq!(57, day17_part2());
    }
}
