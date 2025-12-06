const INPUT: &str = include_str!("../../2025/input/day06.txt");

pub fn part1() -> usize {
    solve_part1(INPUT)
}

pub fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let mut lines: Vec<_> = input.trim().lines().collect();
    let operator_line: Vec<_> = lines.remove(lines.len() - 1).split_whitespace().collect();
    let number_grid: Vec<Vec<usize>> = lines
        .into_iter()
        .map(|line| line.split_whitespace().flat_map(|n| n.parse()).collect())
        .collect();

    operator_line
        .iter()
        .enumerate()
        .map(|(col_idx, operator)| match *operator {
            "*" => number_grid.iter().map(|nums| nums[col_idx]).product(),
            "+" => number_grid.iter().map(|nums| nums[col_idx]).sum::<usize>(),
            _ => unreachable!(),
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let char_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    let mut problem_numbers = vec![];
    for (col_idx, symbol) in char_grid[char_grid.len() - 1].iter().enumerate().rev() {
        let col_str = char_grid
            .iter()
            .map(|line| line[col_idx])
            .take(char_grid.len() - 1) // skip the operator at the bottom end
            .collect::<String>();
        if let Ok(number) = col_str.trim().parse() {
            problem_numbers.push(number);
        } else {
            problem_numbers.clear();
        }
        if symbol == &'*' {
            sum += problem_numbers.iter().product::<usize>();
        } else if symbol == &'+' {
            sum += problem_numbers.iter().sum::<usize>();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    // Workaround to preserve leading and trailing whitespace:
    const EXAMPLE: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  \n"
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(
            33210 + 490 + 4243455 + 401, // = 4277556
            solve_part1(EXAMPLE)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(7098065460541, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            1058 + 3253600 + 625 + 8544, // = 3263827
            solve_part2(EXAMPLE)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(13807151830618, solve_part2(INPUT));
    }
}
