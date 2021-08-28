const PUZZLE_DAY15_INPUT: [usize; 2] = [289, 629];
const FACTORS: [usize; 2] = [16807, 48271];
const DIVISOR: usize = 2147483647;
const PART1_ITERATIONS: usize = 40_000_000;
const PART2_ITERATIONS: usize = 5_000_000;
const GENERATOR_A: usize = 0;
const GENERATOR_B: usize = 1;
const BITMASK: usize = 65535;

pub(crate) fn day15_part1() -> usize {
    part1_generators(PUZZLE_DAY15_INPUT)
}

pub(crate) fn day15_part2() -> usize {
    part2_generators(PUZZLE_DAY15_INPUT)
}

fn part1_generators(initial: [usize; 2]) -> usize {
    let mut a = initial[GENERATOR_A];
    let mut b = initial[GENERATOR_B];

    let mut match_count = 0;
    for _ in 0..PART1_ITERATIONS {
        a = (a * FACTORS[GENERATOR_A]) % DIVISOR;
        b = (b * FACTORS[GENERATOR_B]) % DIVISOR;
        if (a & BITMASK) == (b & BITMASK) {
            match_count += 1;
        }
    }
    match_count
}

fn part2_generators(initial: [usize; 2]) -> usize {
    let mut a = initial[GENERATOR_A];
    let mut b = initial[GENERATOR_B];

    let mut match_count = 0;
    for _ in 0..PART2_ITERATIONS {
        loop {
            a = (a * FACTORS[GENERATOR_A]) % DIVISOR;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = (b * FACTORS[GENERATOR_B]) % DIVISOR;
            if b % 8 == 0 {
                break;
            }
        }
        if (a & BITMASK) == (b & BITMASK) {
            match_count += 1;
        }
    }
    match_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [usize; 2] = [65, 8921];

    #[test]
    fn part1_example() {
        assert_eq!(588, part1_generators(EXAMPLE));
    }

    #[test]
    fn part1_full() {
        assert_eq!(638, day15_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(309, part2_generators(EXAMPLE));
    }

    #[test]
    fn part2_full() {
        assert_eq!(343, day15_part2());
    }
}
