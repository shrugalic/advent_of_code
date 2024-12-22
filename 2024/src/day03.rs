const INPUT: &str = include_str!("../../2024/input/day03.txt");

pub fn part1() -> u32 {
    solve_part1(INPUT)
}

pub fn part2() -> u32 {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> u32 {
    sum_of_multiplications(input, true)
}

fn solve_part2(input: &str) -> u32 {
    sum_of_multiplications(input, false)
}

fn sum_of_multiplications(input: &str, unconditional_sum: bool) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    let chars: Vec<char> = input.trim().chars().collect();
    let mut is_addition_enabled = true;

    while i < chars.len() {
        if input[i..].starts_with("mul(") {
            i += "mul(".len();
        } else if input[i..].starts_with("do()") {
            i += "do()".len();
            is_addition_enabled = true;
        } else if input[i..].starts_with("don't()") {
            i += "don't()".len();
            is_addition_enabled = false;
        } else {
            i += 1;
            continue;
        }

        let mut find_factor_before = |terminator: char| -> Option<u32> {
            let mut factor = 0;
            while let Some(&c) = chars.get(i) {
                if c.is_ascii_digit() {
                    i += 1;
                    factor = 10 * factor + c.to_digit(10).unwrap();
                } else if c == terminator {
                    i += 1;
                    return Some(factor);
                } else {
                    return None;
                }
            }
            None
        };

        if let Some(multiplier) = find_factor_before(',') {
            if let Some(factor) = find_factor_before(')') {
                if unconditional_sum || is_addition_enabled {
                    sum += multiplier * factor;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1_example() {
        assert_eq!(2 * 4 + 5 * 5 + 11 * 8 + 8 * 5, solve_part1(EXAMPLE1));
    }

    #[test]
    fn test_part1() {
        assert_eq!(179_571_322, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2 * 4 + 8 * 5, solve_part2(EXAMPLE2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(103_811_193, solve_part2(INPUT));
    }
}
