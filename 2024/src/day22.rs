const INPUT: &str = include_str!("../../2024/input/day22.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

pub fn solve_part1(input: &str) -> usize {
    parse(input)
        .filter_map(|num| next_numbers(num).nth(2000))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let secret_numbers: Vec<_> = parse(input).collect();
    let mut last_seller_id_for_sequence_id: Vec<usize> = vec![secret_numbers.len(); POW4];
    let mut total_price_for_sequence_id: Vec<usize> = vec![0; POW4];
    for (seller_id, seed) in secret_numbers.into_iter().enumerate() {
        let secret_numbers: Vec<_> = next_numbers_with_price_diff(seed).take(2000).collect();

        for w in secret_numbers.windows(4) {
            let seq_id = id_from(w[0].1, w[1].1, w[2].1, w[3].1);
            let price = (w[3].0 % 10) as u8;

            if last_seller_id_for_sequence_id[seq_id] != seller_id {
                last_seller_id_for_sequence_id[seq_id] = seller_id;
                total_price_for_sequence_id[seq_id] += price as usize;
            }
        }
    }
    *total_price_for_sequence_id.iter().max().unwrap()
}

const POW4: usize = 19 * 19 * 19 * 19;
const POW3: usize = 19 * 19 * 19;
const POW2: usize = 19 * 19;
const POW1: usize = 19;
/// Convert a sequence of 4 digits ranging from -9 to +9 into an unsigned base-19 integer
fn id_from(a: isize, b: isize, c: isize, d: isize) -> usize {
    POW3 * (a + 9) as usize + POW2 * (b + 9) as usize + POW1 * (c + 9) as usize + (d + 9) as usize
}

fn parse(input: &str) -> impl Iterator<Item = usize> + use<'_> {
    input.trim().lines().filter_map(|line| line.parse().ok())
}

fn next_numbers(seed: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(seed), |curr_num| Some(next_number(*curr_num)))
}

fn next_numbers_with_price_diff(seed: usize) -> impl Iterator<Item = (usize, isize)> {
    let price_of = |num| (num % 10) as isize;

    let first = next_number(seed);
    let diff = price_of(first) - price_of(seed);

    std::iter::successors(Some((first, diff)), move |(curr_num, _)| {
        let next_num = next_number(*curr_num);
        let next_diff = price_of(next_num) - price_of(*curr_num);
        Some((next_num, next_diff))
    })
}

fn next_number(mut num: usize) -> usize {
    num ^= num << 6; // * 64
    num %= 16_777_216;
    num ^= num >> 5; // / 32 ; no need to prune after this
    num ^= num << 11; // * 2028
    num % 16_777_216
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
1
10
100
2024
";

    const EXAMPLE_2: &str = "\
1
2
3
2024
";

    #[test]
    fn test_next_number() {
        assert_eq!(15_887_950, next_number(123));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(37_327_623, solve_part1(EXAMPLE_1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(19_822_877_190, solve_part1(INPUT));
    }

    #[test]
    fn test_next_numbers_and_diff() {
        let numbers: Vec<_> = next_numbers_with_price_diff(123).take(9).collect();
        assert_eq!((15_887_950, -3), numbers[0]);
        assert_eq!((16_495_136, 6), numbers[1]);
        assert_eq!((527_345, -1), numbers[2]);
        assert_eq!((704_524, -1), numbers[3]);
        assert_eq!((1_553_684, 0), numbers[4]);
        assert_eq!((12_683_156, 2), numbers[5]);
        assert_eq!((11_100_544, -2), numbers[6]);
        assert_eq!((12_249_484, 0), numbers[7]);
        assert_eq!((7_753_432, -2), numbers[8]);
    }

    #[test]
    fn test_id_from() {
        assert_eq!(0, id_from(-9, -9, -9, -9));
        assert_eq!(POW3 * 9 + POW2 * 9 + POW1 * 9 + 9, id_from(0, 0, 0, 0));
        assert_eq!(POW4 - 1, id_from(9, 9, 9, 9));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(23, solve_part2(EXAMPLE_2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2_277, solve_part2(INPUT));
    }
}
