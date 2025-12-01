const INPUT: &str = include_str!("../../2025/input/day01.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

const NAIVE: bool = false;
const START: isize = 50;
const SIZE: isize = 100;

fn solve_part1(input: &str) -> usize {
    let mut curr = START;
    let mut zero_count: usize = 0;
    for dir in parse(input) {
        match dir {
            Direction::Left(steps) => {
                curr = (curr - steps).rem_euclid(SIZE);
            }
            Direction::Right(steps) => {
                curr = (curr + steps) % SIZE;
            }
        }
        if curr == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn solve_part2(input: &str) -> usize {
    let mut curr = START;
    let mut zero_count: usize = 0;
    for dir in parse(input) {
        match dir {
            Direction::Left(steps) => {
                if NAIVE {
                    let mut remaining = steps;
                    while remaining > 0 {
                        curr = (curr - 1).rem_euclid(SIZE);
                        if curr == 0 {
                            zero_count += 1;
                        }
                        remaining -= 1;
                    }
                } else {
                    let full_turns = ((-curr).rem_euclid(SIZE) + steps) / SIZE;
                    zero_count += full_turns as usize;
                    curr = (curr - steps).rem_euclid(SIZE);
                }
            }
            Direction::Right(steps) => {
                curr += steps;
                let full_turns = curr / SIZE;
                zero_count += full_turns as usize;
                curr = curr % SIZE;
            }
        }
    }
    zero_count
}

#[derive(Debug)]
enum Direction {
    Left(isize),
    Right(isize),
}

fn parse(input: &str) -> impl Iterator<Item = Direction> {
    input.trim().lines().map(|line| {
        let (l, r) = line.split_at(1);
        match l {
            "L" => Direction::Left(r.parse().unwrap()),
            "R" => Direction::Right(r.parse().unwrap()),
            _ => unreachable!(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1_example() {
        assert_eq!(3, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1_031, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(6, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        // 2_310, 2_188, 5_643 are too low
        assert_eq!(5_831, solve_part2(INPUT));
    }

    use rstest::rstest;
    #[rstest]
    #[case("R50", 1)]
    #[case("R100", 1)]
    #[case("R150", 2)]
    #[case("R200", 2)]
    #[case("L50", 1)]
    #[case("L100", 1)]
    #[case("L150", 2)]
    #[case("L200", 2)]
    fn run_part2_test(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, solve_part2(input));
    }
}
