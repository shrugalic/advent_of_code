const INPUT: &str = include_str!("../../2024/input/day07.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    parse(input)
        .filter(Equation::has_solution_without_concatenation)
        .map(Equation::left_side)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    parse(input)
        .filter(Equation::has_solution_with_concatenation)
        .map(Equation::left_side)
        .sum()
}

#[derive(Debug)]
struct Equation {
    left_side: usize,
    right_side: Vec<usize>,
}
impl Equation {
    fn left_side(self) -> usize {
        self.left_side
    }
    fn has_solution_without_concatenation(&self) -> bool {
        Equation::has_solution(self.left_side, &self.right_side, false)
    }
    fn has_solution_with_concatenation(&self) -> bool {
        Equation::has_solution(self.left_side, &self.right_side, true)
    }
    fn has_solution(left_side: usize, right_side: &[usize], allow_concat: bool) -> bool {
        let last_idx = right_side.len() - 1;
        let last_num = right_side[last_idx];
        if right_side.len() == 1 {
            return left_side == last_num;
        }
        let front_nums = &right_side[..last_idx];

        let last_operation_is_addition = || {
            left_side > last_num
                && Equation::has_solution(left_side - last_num, front_nums, allow_concat)
        };
        let last_op_is_multiplication = || {
            left_side % last_num == 0
                && Equation::has_solution(left_side / last_num, front_nums, allow_concat)
        };
        let last_op_is_concatenation = || {
            let last_num_string = last_num.to_string();
            let concat_divisor = 10usize.pow(last_num_string.len() as u32);
            left_side.to_string().ends_with(&last_num_string)
                && Equation::has_solution(left_side / concat_divisor, front_nums, allow_concat)
        };

        last_operation_is_addition()
            || last_op_is_multiplication()
            || allow_concat && last_op_is_concatenation()
    }
}

fn parse(input: &str) -> impl Iterator<Item = Equation> + use<'_> {
    input.trim().lines().map(|line| {
        let (left, right) = line.split_once(": ").unwrap();
        let left = left.parse().unwrap();
        let right = right
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Equation {
            left_side: left,
            right_side: right,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1_example() {
        assert_eq!(190 + 3267 + 292, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(42_283_209_483_350, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(190 + 3267 + 156 + 7290 + 192 + 292, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1_026_766_857_276_279, solve_part2(INPUT));
    }
}
