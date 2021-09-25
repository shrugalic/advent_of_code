use line_reader::read_file_to_lines;

pub(crate) fn day03_part1() -> usize {
    count_possible_triangle_rows(read_file_to_lines("input/day03.txt"))
}

pub(crate) fn day03_part2() -> usize {
    count_possible_triangle_columns(read_file_to_lines("input/day03.txt"))
}

fn count_possible_triangle_rows(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|s| is_triangle(s[0], s[1], s[2]))
        .count()
}

fn is_triangle(a: usize, b: usize, c: usize) -> bool {
    let mut sides = [a, b, c];
    sides.sort_unstable();
    sides[0] + sides[1] > sides[2]
}

fn count_possible_triangle_columns(input: Vec<String>) -> usize {
    let input: Vec<Vec<usize>> = input
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    input
        .windows(3)
        .step_by(3)
        .map(|n| {
            // println!("{:?}", n);
            (0..3)
                .into_iter()
                .filter(move |col| is_triangle(n[0][*col], n[1][*col], n[2][*col]))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    #[test]
    fn part1() {
        assert_eq!(1050, day03_part1());
    }

    const EXAMPLE: &str = "\
101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";

    #[test]
    fn part2_example() {
        assert_eq!(
            6,
            count_possible_triangle_columns(read_str_to_lines(EXAMPLE))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(1921, day03_part2());
    }
}
