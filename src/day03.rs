const INPUT: &str = include_str!("../input/day03.txt");

pub(crate) fn day03_part1() -> u32 {
    let numbers = parse(INPUT);
    gamma_times_epsilon(numbers)
}

pub(crate) fn day03_part2() -> u32 {
    let numbers = parse(INPUT);
    reduce_numbers(numbers)
}

fn gamma_times_epsilon(numbers: Vec<Vec<bool>>) -> u32 {
    let len = numbers[0].len();
    let mut gamma = Vec::with_capacity(len);
    for i in 0..len {
        let (ones, zeroes) = count_ones_and_zeroes_at_index(&numbers, i);
        gamma.push(ones >= zeroes);
    }
    let gamma = to_decimal(&gamma);
    let epsilon = (1 << len) - 1 - gamma; // complement of gamma
    gamma * epsilon
}

fn count_ones_and_zeroes_at_index(numbers: &[Vec<bool>], i: usize) -> (usize, usize) {
    let ones = numbers.iter().filter(|bits| bits[i]).count();
    let zeroes = numbers.len() - ones;
    (ones, zeroes)
}

fn reduce_numbers(numbers: Vec<Vec<bool>>) -> u32 {
    let og_rating = reduce(numbers.clone(), |ones, zeroes| ones >= zeroes);
    let cs_rating = reduce(numbers, |ones, zeroes| ones < zeroes);
    og_rating * cs_rating
}

type Filter = fn(usize, usize) -> bool;
fn reduce(mut numbers: Vec<Vec<bool>>, wanted: Filter) -> u32 {
    let mut i = 0;
    while numbers.len() > 1 {
        let (ones, zeroes) = count_ones_and_zeroes_at_index(&numbers, i);
        numbers.retain(|bits| bits[i] == wanted(ones, zeroes));
        i += 1;
    }
    to_decimal(&numbers[0])
}

fn to_decimal(bits: &[bool]) -> u32 {
    bits.iter()
        .map(|&is_one| if is_one { 1 } else { 0 })
        .fold(0, |a, i| (a << 1) + i)
}

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|s| s.chars().map(|c| c == '1').collect::<Vec<_>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn example1() {
        let numbers = parse(EXAMPLE);
        assert_eq!(22 * 9, gamma_times_epsilon(numbers));
    }

    #[test]
    fn example2() {
        let numbers = parse(EXAMPLE);
        assert_eq!(23 * 10, reduce_numbers(numbers));
    }

    #[test]
    fn part1() {
        assert_eq!(day03_part1(), 284 * 3811);
    }

    #[test]
    fn part2() {
        assert_eq!(day03_part2(), 486 * 2784);
    }
}
