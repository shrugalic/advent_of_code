use self::Result::*;
use Shape::*;

const INPUT: &str = include_str!("../input/day02.txt");

pub(crate) fn day02_part1() -> usize {
    let strategy = parse(INPUT);
    follow_part1_rounds(strategy)
}

pub(crate) fn day02_part2() -> usize {
    let rounds = parse(INPUT);
    follow_part2_rounds(rounds)
}

fn follow_part1_rounds(strategy: Vec<Instructions>) -> usize {
    let mut score = 0;
    for round in strategy {
        // Interpret the unknown instruction as own shape
        let own_shape = Shape::from(round.unknown_instruction);
        score += score_shape(&own_shape) + score_outcome(&round.opponents_shape, &own_shape)
    }
    score
}

fn follow_part2_rounds(strategy: Vec<Instructions>) -> usize {
    let mut score = 0;
    for round in strategy {
        // Interpret the unknown instruction as desired result,
        // and choose own shape accordingly
        let result = Result::from(round.unknown_instruction);
        let own_shape = determine_own_shape(result, &round.opponents_shape);
        score += score_shape(&own_shape) + score_outcome(&round.opponents_shape, &own_shape)
    }
    score
}

fn score_shape(own_shape: &Shape) -> usize {
    match own_shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}
fn score_outcome(other_shape: &Shape, own_shape: &Shape) -> usize {
    match (other_shape, own_shape) {
        (Rock, Rock) => 3,
        (Rock, Paper) => 6,
        (Rock, Scissors) => 0,
        (Paper, Rock) => 0,
        (Paper, Paper) => 3,
        (Paper, Scissors) => 6,
        (Scissors, Rock) => 6,
        (Scissors, Paper) => 0,
        (Scissors, Scissors) => 3,
    }
}

fn determine_own_shape(result: Result, opponents_shape: &Shape) -> Shape {
    match (opponents_shape, result) {
        (Rock, Loss) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,
        (Paper, Loss) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Loss) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, Win) => Rock,
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}
impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => unreachable!(),
        }
    }
}

struct Instructions {
    opponents_shape: Shape,
    unknown_instruction: char,
}
impl From<&str> for Instructions {
    fn from(line: &str) -> Self {
        let line: Vec<char> = line.chars().collect();
        Instructions {
            opponents_shape: Shape::from(line[0]),
            unknown_instruction: line[2],
        }
    }
}

enum Result {
    Loss,
    Draw,
    Win,
}
impl From<char> for Result {
    fn from(c: char) -> Self {
        match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Instructions> {
    input.trim().lines().map(Instructions::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
A Y
B X
C Z";

    #[test]
    fn example1() {
        let rounds = parse(EXAMPLE);
        assert_eq!(15, follow_part1_rounds(rounds));
    }

    #[test]
    fn example2() {
        let rounds = parse(EXAMPLE);
        assert_eq!(12, follow_part2_rounds(rounds));
    }

    #[test]
    fn part1() {
        assert_eq!(day02_part1(), 12_276);
    }

    #[test]
    fn part2() {
        assert_eq!(day02_part2(), 9_975);
    }
}
