const INPUT: &str = include_str!("../input/day03.txt");

pub(crate) fn day03_part1() -> usize {
    let numbers = parse(INPUT);
    gamma_times_epsilon(numbers)
}

pub(crate) fn day03_part2() -> u32 {
    let numbers = parse(INPUT);
    reduce_numbers(numbers)
}

fn gamma_times_epsilon(numbers: Vec<Vec<u32>>) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..numbers[0].len() {
        let (ones, zeroes) = count_ones_and_zeroes_at_index(&numbers, i);
        gamma <<= 1;
        epsilon <<= 1;
        if ones >= zeroes {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

fn count_ones_and_zeroes_at_index(numbers: &[Vec<u32>], i: usize) -> (usize, usize) {
    let mut ones = 0;
    let mut zeroes = 0;
    for number in numbers.iter() {
        if number[i] == 1 {
            ones += 1;
        } else {
            zeroes += 1;
        }
    }
    (ones, zeroes)
}

fn reduce_numbers(numbers: Vec<Vec<u32>>) -> u32 {
    let og_rating = reduce(numbers.clone(), |o, z| if o >= z { 1 } else { 0 });
    let cs_rating = reduce(numbers, |o, z| if o >= z { 0 } else { 1 });
    og_rating * cs_rating
}

fn reduce(mut numbers: Vec<Vec<u32>>, wanted_filter: fn(usize, usize) -> u32) -> u32 {
    let mut i = 0;
    while numbers.len() > 1 {
        let (ones, zeroes) = count_ones_and_zeroes_at_index(&numbers, i);
        numbers = numbers
            .into_iter()
            .filter(|n| n[i] == wanted_filter(ones, zeroes))
            .collect();
        i += 1;
    }
    numbers[0].iter().fold(0, |a, &i| (a << 1) + i)
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|s| s.chars().filter_map(|c| c.to_digit(2)).collect::<Vec<_>>())
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
