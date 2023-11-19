const INPUT: &str = include_str!("../input/day10.txt");

pub(crate) fn day10_part1() -> usize {
    let lines = parse(INPUT);
    calculate_score(lines, ChunkType::Illegal)
}

pub(crate) fn day10_part2() -> usize {
    let lines = parse(INPUT);
    calculate_score(lines, ChunkType::Incomplete)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

use ChunkType::*;
enum ChunkType {
    Illegal,
    Incomplete,
}

fn calculate_score(lines: Vec<Vec<char>>, chunk_type: ChunkType) -> usize {
    let mut illegal: Vec<char> = vec![];
    let mut incomplete: Vec<usize> = vec![];
    'lines_loop: for line in lines {
        let mut expected: Vec<char> = vec![];
        for c in line {
            match c {
                '(' => expected.push(')'),
                '[' => expected.push(']'),
                '{' => expected.push('}'),
                '<' => expected.push('>'),
                _ => {
                    if c != expected.pop().expect("No more open chunks") {
                        // Stop on and remember the first illegal closing character
                        illegal.push(c);
                        continue 'lines_loop;
                    }
                }
            }
        }
        // Score the unclosed chunks
        incomplete.push(expected.score_incomplete());
    }
    match chunk_type {
        Illegal => illegal.score_illegal(),
        Incomplete => incomplete.score_incomplete(),
    }
}
trait CalculateScore {
    fn score_incomplete(self) -> usize;
    fn score_illegal(self) -> usize;
}
impl CalculateScore for char {
    fn score_incomplete(self) -> usize {
        match self {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        }
    }
    fn score_illegal(self) -> usize {
        match self {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    }
}
impl CalculateScore for Vec<char> {
    fn score_incomplete(self) -> usize {
        self.into_iter()
            .rev()
            .fold(0_usize, |acc, c| 5 * acc + c.score_incomplete())
    }

    fn score_illegal(self) -> usize {
        self.into_iter().map(|c| c.score_illegal()).sum()
    }
}
impl CalculateScore for Vec<usize> {
    fn score_incomplete(mut self) -> usize {
        self.sort_unstable();
        self[self.len() / 2]
    }
    fn score_illegal(self) -> usize {
        unreachable!("Not needed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_example() {
        let lines = parse(EXAMPLE);
        assert_eq!(26_397, calculate_score(lines, ChunkType::Illegal));
    }

    #[test]
    fn part1() {
        assert_eq!(319_329, day10_part1());
    }

    #[test]
    fn part2_example() {
        let lines = parse(EXAMPLE);
        assert_eq!(288_957, calculate_score(lines, ChunkType::Incomplete));
    }

    #[test]
    fn part2() {
        assert_eq!(3_515_583_998, day10_part2());
    }
}
