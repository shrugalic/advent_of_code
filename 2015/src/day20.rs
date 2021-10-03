use std::collections::HashSet;

const PUZZLE_INPUT: usize = 29_000_000;
const PART1_MULTIPLIER: usize = 10;
const PART2_MULTIPLIER: usize = 11;

pub(crate) fn day20_part1() -> usize {
    lowest_house_number_to_get_x_presents_part1(PUZZLE_INPUT)
}

pub(crate) fn day20_part2() -> usize {
    lowest_house_number_to_get_x_presents_part2(PUZZLE_INPUT)
}

fn lowest_house_number_to_get_x_presents_part1(count: usize) -> usize {
    lowest_house_number_to_get_x_presents(count, true)
}

fn lowest_house_number_to_get_x_presents_part2(count: usize) -> usize {
    lowest_house_number_to_get_x_presents(count, false)
}

fn lowest_house_number_to_get_x_presents(mut count: usize, is_part_1: bool) -> usize {
    count /= if is_part_1 {
        PART1_MULTIPLIER
    } else {
        PART2_MULTIPLIER
    };

    for num in 1..4 {
        if count_at_house(num, is_part_1) >= count {
            return num;
        }
    }
    // Optimization: From house number 4 the even houses
    // have larger numbers than odd houses further to the right
    let mut num = 4;
    while count_at_house(num, is_part_1) < count {
        num += 2;
    }
    num
}

fn count_at_house(num: usize, is_part_1: bool) -> usize {
    let naive = true;
    // Part 1 takes ~5s with the naive version,
    // and ~52s with the sum_of_divisors method
    if naive {
        let mut sum = 0;
        let limit = (num as f64).sqrt() as usize;
        for i in 1..=limit {
            if num % i == 0 && (is_part_1 || i <= 50) {
                sum += i + num / i;
            }
        }
        sum
    } else {
        sum_of_divisors_of(num)
    }
}

// Sum of divisors according to
// https://www2.math.upenn.edu/~deturck/m170/wk3/lecture/sumdiv.html
fn sum_of_divisors_of(n: usize) -> usize {
    let factors = prime_factors_of(n);
    let unique_factors: HashSet<usize> = factors.iter().cloned().collect();
    // println!("factors {:?}, unique {:?}", factors, unique_factors);

    let mut sum = 1;
    for factor in unique_factors {
        let count = factors.iter().filter(|&i| i == &factor).count();
        let mut factor_sum = 0;
        for i in 0..=count {
            factor_sum += factor.pow(i as u32);
        }
        sum *= factor_sum;
    }

    sum
}

fn prime_factors_of(n: usize) -> Vec<usize> {
    let mut factors = vec![];
    let mut rem = n;
    let mut factor = 2;
    while rem > 1 {
        while rem % factor == 0 {
            factors.push(factor);
            rem /= factor;
        }
        factor += 1;
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(1, lowest_house_number_to_get_x_presents_part1(10));
        assert_eq!(2, lowest_house_number_to_get_x_presents_part1(30));
        assert_eq!(3, lowest_house_number_to_get_x_presents_part1(40));
        assert_eq!(4, lowest_house_number_to_get_x_presents_part1(70));
        assert_eq!(6, lowest_house_number_to_get_x_presents_part1(120));
        assert_eq!(8, lowest_house_number_to_get_x_presents_part1(150));
        assert_eq!(10, lowest_house_number_to_get_x_presents_part1(180));
        assert_eq!(12, lowest_house_number_to_get_x_presents_part1(280));
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(
            vec![2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 29],
            prime_factors_of(PUZZLE_INPUT / PART1_MULTIPLIER)
        );
        assert_eq!(
            vec![2, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 5, 29],
            prime_factors_of(PUZZLE_INPUT)
        );
        assert_eq!(
            vec![2, 2, 2, 2, 2, 2, 3, 3, 3, 5, 7, 11],
            prime_factors_of(665_280)
        );
    }

    #[test]
    fn test_sum_of_divisors() {
        assert_eq!(1, sum_of_divisors_of(1));
        assert_eq!(7, sum_of_divisors_of(4));
        assert_eq!(12, sum_of_divisors_of(6));
        assert_eq!(15, sum_of_divisors_of(8));
        assert_eq!(18, sum_of_divisors_of(10));

        assert_eq!(2_926_080, sum_of_divisors_of(665_280));
        assert!(sum_of_divisors_of(665_280) * PART1_MULTIPLIER >= PUZZLE_INPUT);
    }

    #[test]
    fn part1() {
        assert_eq!(665_280, day20_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(705_600, day20_part2());
    }
}
