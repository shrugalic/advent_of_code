const INPUT: &str = include_str!("../../2024/input/day04.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

type Grid = Vec<Vec<char>>;

#[expect(clippy::identity_op)] // I consider the + 0 helpful for readability
fn solve_part1(input: &str) -> usize {
    let grid = parse(input);

    let mut horizontal_count = 0;
    for line in grid.iter() {
        for w in line.windows(4) {
            if w == ['X', 'M', 'A', 'S'] || w == ['S', 'A', 'M', 'X'] {
                horizontal_count += 1;
            }
        }
    }

    let contains_xmas = |c1: &char, c2: &char, c3: &char, c4: &char| -> bool {
        c1 == &'X' && c2 == &'M' && c3 == &'A' && c4 == &'S'
            || c1 == &'S' && c2 == &'A' && c3 == &'M' && c4 == &'X'
    };
    let mut vertical_count = 0;
    for x in 0..grid[0].len() {
        for y in 0..grid.len() - 3 {
            if contains_xmas(
                &grid[y + 0][x],
                &grid[y + 1][x],
                &grid[y + 2][x],
                &grid[y + 3][x],
            ) {
                vertical_count += 1;
            }
        }
    }

    let mut diagonal1_count = 0;
    let mut diagonal2_count = 0;
    for x in 0..grid[0].len() - 3 {
        for y in 0..grid.len() - 3 {
            if contains_xmas(
                &grid[y + 0][x + 0],
                &grid[y + 1][x + 1],
                &grid[y + 2][x + 2],
                &grid[y + 3][x + 3],
            ) {
                diagonal1_count += 1;
            }
            if contains_xmas(
                &grid[y + 3][x + 0],
                &grid[y + 2][x + 1],
                &grid[y + 1][x + 2],
                &grid[y + 0][x + 3],
            ) {
                diagonal2_count += 1;
            }
        }
    }
    horizontal_count + vertical_count + diagonal1_count + diagonal2_count
}

#[expect(clippy::identity_op)] // I consider the + 0 helpful for readability
fn solve_part2(input: &str) -> usize {
    let grid = parse(input);
    let mut count = 0;
    let contains_mas = |c1: &char, c2: &char, c3: &char| -> bool {
        c1 == &'M' && c2 == &'A' && c3 == &'S' || c1 == &'S' && c2 == &'A' && c3 == &'M'
    };
    for x in 0..grid[0].len() - 2 {
        for y in 0..grid.len() - 2 {
            if contains_mas(
                &grid[y + 0][x + 0],
                &grid[y + 1][x + 1],
                &grid[y + 2][x + 2],
            ) && contains_mas(
                &grid[y + 2][x + 0],
                &grid[y + 1][x + 1],
                &grid[y + 0][x + 2],
            ) {
                count += 1;
            }
        }
    }
    count
}

fn parse(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1_example() {
        assert_eq!(18, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(2507, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(9, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        // not 1983, that's when also including a straight cross shape 🤦‍♂️
        assert_eq!(1969, solve_part2(INPUT));
    }
}
