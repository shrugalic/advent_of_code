use crate::parse;

const INPUT: &str = include_str!("../input/day21.txt");

pub(crate) fn day21_part1() -> String {
    let ops: Vec<Op> = parse_operations(parse(INPUT));
    scramble(&ops, "abcdefgh")
}

pub(crate) fn day21_part2() -> String {
    let ops: Vec<Op> = parse_operations(parse(INPUT));
    unscramble(&ops, "fbgdceah")
}

#[derive(Debug, Copy, Clone)]
enum Op {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnLetterPos(char),
    ReversePositions(usize, usize),
    MovePos(usize, usize),
}
impl Op {
    fn apply_to(&self, chars: &mut Vec<char>) {
        match self {
            Op::SwapPos(x, y) => chars.swap(*x, *y),
            Op::SwapLetter(x, y) => {
                let (pos_x, pos_y) = chars.find_pos_pair(x, y);
                chars.swap(pos_x, pos_y);
            }
            Op::RotateLeft(x) => chars.rotate_left(*x),
            Op::RotateRight(x) => chars.rotate_right(*x),
            Op::RotateBasedOnLetterPos(x) => {
                let pos = chars.find_pos(x);
                let mut rot = 1 + pos + if pos >= 4 { 1 } else { 0 };
                rot %= chars.len();
                chars.rotate_right(rot);
            }
            Op::ReversePositions(x, y) => chars[*x..=*y].reverse(),
            Op::MovePos(x, y) => {
                let c = chars.remove(*x);
                chars.insert(*y, c);
            }
        }
    }
    fn inverse(&self, chars: &[char]) -> Op {
        match self {
            Op::RotateLeft(x) => Op::RotateRight(*x),
            Op::RotateRight(x) => Op::RotateLeft(*x),
            Op::MovePos(x, y) => Op::MovePos(*y, *x),
            Op::RotateBasedOnLetterPos(x) => match chars.find_pos(x) {
                1 => Op::RotateLeft(1),
                3 => Op::RotateLeft(2),
                5 => Op::RotateLeft(3),
                7 => Op::RotateLeft(4),
                2 => Op::RotateLeft(6 % chars.len()),
                4 => Op::RotateLeft(7 % chars.len()),
                6 => Op::RotateLeft(0),
                0 => Op::RotateLeft(1),
                _ => unreachable!(),
            },
            op => *op, // These other ops are their own inverse
        }
    }
}
impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let p: Vec<_> = s.split_ascii_whitespace().collect();
        match (p[0], p[1]) {
            ("swap", "position") => Op::SwapPos(p[2].parse().unwrap(), p[5].parse().unwrap()),
            ("swap", "letter") => Op::SwapLetter(p[2].parse().unwrap(), p[5].parse().unwrap()),
            ("rotate", "left") => Op::RotateLeft(p[2].parse().unwrap()),
            ("rotate", "right") => Op::RotateRight(p[2].parse().unwrap()),
            ("rotate", "based") => Op::RotateBasedOnLetterPos(p[6].parse().unwrap()),
            ("reverse", "positions") => {
                Op::ReversePositions(p[2].parse().unwrap(), p[4].parse().unwrap())
            }
            ("move", "position") => Op::MovePos(p[2].parse().unwrap(), p[5].parse().unwrap()),
            _ => panic!("Invalid input {}", s),
        }
    }
}

fn parse_operations(input: Vec<&str>) -> Vec<Op> {
    input.into_iter().map(Op::from).collect()
}

trait FindPos {
    fn find_pos(&self, wanted: &char) -> usize;
    fn find_pos_pair(&self, wanted1: &char, wanted2: &char) -> (usize, usize) {
        (self.find_pos(wanted1), self.find_pos(wanted2))
    }
}
impl FindPos for [char] {
    fn find_pos(&self, wanted: &char) -> usize {
        self.iter().position(|ch| ch == wanted).unwrap()
    }
}

fn scramble(ops: &[Op], input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    apply_ops(ops, chars)
}
fn apply_ops(ops: &[Op], mut chars: Vec<char>) -> String {
    for op in ops {
        op.apply_to(&mut chars);
    }
    chars.into_iter().collect()
}

fn unscramble(ops: &[Op], input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    for op in ops.iter().rev() {
        op.inverse(&chars).apply_to(&mut chars);
    }
    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE_OPS: &str = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn part1_example() {
        let input = "abcde";
        let ops: Vec<Op> = parse_operations(parse(EXAMPLE_OPS));
        assert_eq!("decab", scramble(&ops, input));
    }

    #[test]
    fn part1() {
        assert_eq!("dgfaehcb", day21_part1());
    }

    #[test]
    fn part2_example() {
        let ops: Vec<Op> = parse_operations(parse(EXAMPLE_OPS));
        assert_eq!("abcde", unscramble(&ops, "decab"));
    }

    #[test]
    fn test_reverse_rotate_based_on_letter_pos() {
        let input = "abcdefgh";
        let ops = vec![Op::RotateBasedOnLetterPos('a')];
        assert_eq!(input, unscramble(&ops, &scramble(&ops, input)));

        let ops = vec![Op::RotateBasedOnLetterPos('b')];
        assert_eq!(input, unscramble(&ops, &scramble(&ops, input)));

        let ops = vec![Op::RotateBasedOnLetterPos('c')];
        assert_eq!(input, unscramble(&ops, &scramble(&ops, input)));

        let ops = vec![Op::RotateBasedOnLetterPos('d')];
        assert_eq!(input, unscramble(&ops, &scramble(&ops, input)));

        let ops = vec![Op::RotateBasedOnLetterPos('e')];
        assert_eq!(input, unscramble(&ops, &scramble(&ops, input)));

        let ops = vec![Op::RotateBasedOnLetterPos('f')];
        assert_eq!(input, unscramble(&ops, &scramble(&ops, input)));

        let ops = vec![Op::RotateBasedOnLetterPos('g')];
        assert_eq!(input, unscramble(&ops, &scramble(&ops, input)));
    }

    #[test]
    fn part2() {
        assert_eq!("fdhgacbe", day21_part2());
    }
}
