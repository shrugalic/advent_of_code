use crate::parse;

const INPUT: &str = include_str!("../input/day25.txt");

pub(crate) fn day25_part1() -> usize {
    let (row, column) = parse_input();
    calculate_code_for_row_and_column(row, column)
}

fn parse_input() -> (usize, usize) {
    let line = &parse(INPUT)[0];
    let parts: Vec<_> = line.split(|c| [' ', '.', ','].contains(&c)).collect();
    (parts[18].parse().unwrap(), parts[21].parse().unwrap())
}

const FIRST_CODE: usize = 20_151_125;
const MULTIPLIER: usize = 252_533;
const DIVISOR: usize = 33_554_393;
fn generate_next_code(current_code: usize) -> usize {
    (current_code * MULTIPLIER) % DIVISOR
}

fn generate_nth_code(n: usize) -> usize {
    let mut code = FIRST_CODE;
    for _ in 1..n {
        code = generate_next_code(code);
    }
    code
}

fn calculate_n_for_row_and_column(row: usize, col: usize) -> usize {
    // y\x | 1 | 2 | 3 | 4 | 5 | 6 | 7 | …
    // ----+---+---+---+---+---+---+---+
    //   1 |  1   3   6  10  15  21
    //   2 |  2   5   9  14  20
    //   3 |  4   8  13  19
    //   4 |  7  12  18
    //   5 | 11  17
    //   6 | 16
    //   7 | …
    // The x-th diagonal is equal to row + col - 1. The 1st diagonal is box (1, 1)
    // The 2nd diagonal consists of boxes (2, 1) and (1, 2)
    // The 3rd diagonal consists of boxes (3, 1), (2, 2) and (1, 3)
    let x = row + col - 1;
    // The value r0 at (1, x) is the sum of 1 + 2 + 3 + 4 + … + x = x * (x + 1) / 2
    let r0 = x * (x + 1) / 2;
    // The value at (y, x) is (y - 1) less than the value at (1, x)
    r0 - (row - 1)
}

fn calculate_code_for_row_and_column(row: usize, col: usize) -> usize {
    generate_nth_code(calculate_n_for_row_and_column(row, col))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_next_code() {
        assert_eq!(31_916_031, generate_next_code(FIRST_CODE));
        assert_eq!(18_749_137, generate_next_code(31_916_031));
        assert_eq!(16_080_970, generate_next_code(18_749_137));
        assert_eq!(21_629_792, generate_next_code(16_080_970));
        assert_eq!(17_289_845, generate_next_code(21_629_792));
    }

    #[test]
    fn test_generate_nth_code() {
        assert_eq!(FIRST_CODE, generate_nth_code(1));
        assert_eq!(31_916_031, generate_nth_code(2));
        assert_eq!(18_749_137, generate_nth_code(3));
        assert_eq!(16_080_970, generate_nth_code(4));
        assert_eq!(21_629_792, generate_nth_code(5));
        assert_eq!(17_289_845, generate_nth_code(6));
    }

    #[test]
    fn test_calculate_n_for_row_and_column() {
        assert_eq!(1, calculate_n_for_row_and_column(1, 1));
        assert_eq!(2, calculate_n_for_row_and_column(2, 1));
        assert_eq!(3, calculate_n_for_row_and_column(1, 2));
        assert_eq!(4, calculate_n_for_row_and_column(3, 1));
        assert_eq!(5, calculate_n_for_row_and_column(2, 2));
        assert_eq!(6, calculate_n_for_row_and_column(1, 3));
    }

    #[test]
    fn test_calculate_code_for_row_and_column() {
        assert_eq!(FIRST_CODE, calculate_code_for_row_and_column(1, 1));
        assert_eq!(31_916_031, calculate_code_for_row_and_column(2, 1));
        assert_eq!(18_749_137, calculate_code_for_row_and_column(1, 2));
        assert_eq!(16_080_970, calculate_code_for_row_and_column(3, 1));
        assert_eq!(21_629_792, calculate_code_for_row_and_column(2, 2));
        assert_eq!(17_289_845, calculate_code_for_row_and_column(1, 3));
    }

    #[test]
    fn part1() {
        assert_eq!(19_980_801, day25_part1());
    }
}
