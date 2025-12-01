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
                    let previous = curr;
                    let full_turns = steps / SIZE;
                    zero_count += full_turns as usize;
                    let remaining_steps = steps % SIZE;
                    if remaining_steps >= curr && previous != 0 {
                        zero_count += 1;
                    }
                    curr = (curr - remaining_steps).rem_euclid(SIZE);
                }
            }
            Direction::Right(steps) => {
                curr += steps;
                let full_turns = curr / SIZE;
                curr = curr % SIZE;
                zero_count += full_turns as usize;
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
    fn test_part2_example_R50() {
        assert_eq!(1, solve_part2("R50"));
    }

    #[test]
    fn test_part2_example_R100() {
        assert_eq!(1, solve_part2("R100"));
    }

    #[test]
    fn test_part2_example_R150() {
        assert_eq!(2, solve_part2("R150"));
    }

    #[test]
    fn test_part2_example_R200() {
        assert_eq!(2, solve_part2("R200"));
    }

    #[test]
    fn test_part2_example_L50() {
        assert_eq!(1, solve_part2("L50"));
    }

    #[test]
    fn test_part2_example_L100() {
        assert_eq!(1, solve_part2("L100"));
    }
    #[test]
    fn test_part2_example_L150() {
        assert_eq!(2, solve_part2("L150"));
    }

    #[test]
    fn test_part2_example_L200() {
        assert_eq!(2, solve_part2("L200"));
    }

    #[test]
    fn test_part2() {
        // 2_310, 2_188, 5_643 are too low
        assert_eq!(5_831, solve_part2(INPUT));
    }
}
