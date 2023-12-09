const INPUT: &str = include_str!("../input/day09.txt");

type Number = isize;
type History = Vec<Number>;

pub(crate) fn part1() -> Number {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> Number {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> Number {
    parse_histories(input).map(predict_next_value).sum()
}

fn solve_part2(input: &str) -> Number {
    parse_histories(input).map(predict_previous_value).sum()
}

fn parse_histories(input: &str) -> impl Iterator<Item = History> + '_ {
    input.trim().lines().map(parse_history)
}

fn parse_history(line: &str) -> History {
    line.split_ascii_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn predict_next_value(history: History) -> Number {
    let (_, last_numbers) = calculate_deltas_for(history);
    last_numbers.iter().sum()
}

fn predict_previous_value(history: History) -> Number {
    let (first_numbers, _) = calculate_deltas_for(history);
    first_numbers.iter().rev().fold(
        /* initial prediction */ 0,
        |prediction, first| first - prediction,
    )
}

fn calculate_deltas_for(history: History) -> (Vec<Number>, Vec<Number>) {
    let mut deltas = history;
    let (mut firsts, mut lasts) = (vec![], vec![]);
    while !deltas.iter().all(|&n| n == 0) {
        firsts.push(*deltas.first().unwrap());
        lasts.push(*deltas.last().unwrap());
        deltas = deltas.windows(2).map(|pair| pair[1] - pair[0]).collect();
    }
    (firsts, lasts)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part1_example() {
        assert_eq!(114, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1_938_731_307, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(948, solve_part2(INPUT));
    }
}
