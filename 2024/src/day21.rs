use crate::vec_2d::Vec2D;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter;
use DirPadButton::*;
use NumPadButton::*;

const INPUT: &str = include_str!("../../2024/input/day21.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let chain = Chain::new();
    parse_codes_from(input)
        .map(|code| code.numeric_value() * chain.shortest_sequence_len_for(&code, 2))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let chain = Chain::new();
    parse_codes_from(input)
        .map(|code| code.numeric_value() * chain.shortest_sequence_len_for(&code, 25))
        .sum()
}

#[derive(Debug)]
struct Code(Vec<NumPadButton>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum DirPadButton {
    Up = 0,
    DirPadA = 1,
    Left = 2,
    Down = 3,
    Right = 4,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct ButtonSequence(Vec<DirPadButton>);

#[derive(Debug, Clone, Default, PartialEq)]
struct ButtonSequences(Vec<ButtonSequence>);

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
struct NumericKeypad {
    sequences_by_target_by_source: Vec<Vec<ButtonSequences>>,
}
impl NumericKeypad {
    const FORBIDDEN_POS: Vec2D = Vec2D { x: 0, y: 3 };
    const POS: [Vec2D; 11] = [
        Vec2D { x: 1, y: 3 }, // 0
        Vec2D { x: 0, y: 2 }, // 1
        Vec2D { x: 1, y: 2 }, // 2
        Vec2D { x: 2, y: 2 }, // 3
        Vec2D { x: 0, y: 1 }, // 4
        Vec2D { x: 1, y: 1 }, // 5
        Vec2D { x: 2, y: 1 }, // 6
        Vec2D { x: 0, y: 0 }, // 7
        Vec2D { x: 1, y: 0 }, // 8
        Vec2D { x: 2, y: 0 }, // 9
        Vec2D { x: 2, y: 3 }, // A
    ];
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum NumPadButton {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    NumPadA = 10,
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
struct DirectionalKeypad {
    sequences_by_target_by_source: Vec<Vec<ButtonSequences>>,
}
impl DirectionalKeypad {
    const POS: [Vec2D; 5] = [
        Vec2D { x: 1, y: 0 }, // ^
        Vec2D { x: 2, y: 0 }, // A
        Vec2D { x: 0, y: 1 }, // <
        Vec2D { x: 1, y: 1 }, // v
        Vec2D { x: 2, y: 1 }, // >
    ];
    const FORBIDDEN_POS: Vec2D = Vec2D { x: 0, y: 0 };
}

struct Chain {
    num_pad: NumericKeypad,
    dir_pad: DirectionalKeypad,
}
impl Chain {
    fn new() -> Self {
        Chain {
            num_pad: NumericKeypad::new(),
            dir_pad: DirectionalKeypad::new(),
        }
    }
    fn shortest_sequence_len_for(&self, code: &Code, chain_len: usize) -> usize {
        self.num_pad
            .shortest_dir_pad_sequences_for(code)
            .0
            .into_iter()
            .map(|sequence| {
                self.dir_pad
                    .min_sequence_len(chain_len, &sequence, &mut HashMap::new())
            })
            .min()
            .unwrap()
    }
}

impl NumericKeypad {
    fn shortest_dir_pad_sequences_for(&self, code: &Code) -> ButtonSequences {
        let mut sequences = ButtonSequences(vec![ButtonSequence::default()]);
        let mut from_digit = NumPadA;
        for to_digit in &code.0 {
            let sub_sequences =
                &self.sequences_by_target_by_source[from_digit as usize][*to_digit as usize];
            sequences = ButtonSequences(
                sequences
                    .0
                    .into_iter()
                    .flat_map(|prev_sequence| {
                        sub_sequences.0.iter().map(move |sub_sequence| {
                            ButtonSequence(
                                prev_sequence
                                    .0
                                    .iter()
                                    .chain(sub_sequence.0.iter())
                                    .cloned()
                                    .chain(iter::once(DirPadA))
                                    .collect(),
                            )
                        })
                    })
                    .collect(),
            );
            from_digit = *to_digit;
        }
        sequences
    }
    fn new() -> Self {
        NumericKeypad {
            sequences_by_target_by_source:
                NumericKeypad::precompute_dir_pad_button_sequences_for_num_pad(),
        }
    }
    /// Precomputes all possible DirPadButton sequences to get
    /// from any NumPadButton to any other NumPadButton
    fn precompute_dir_pad_button_sequences_for_num_pad() -> Vec<Vec<ButtonSequences>> {
        // The first index is for the source, the second index is for the target
        let mut sequences_by_target_by_source: Vec<Vec<ButtonSequences>> =
            vec![vec![ButtonSequences(vec![]); 11]; 11];

        for source in NumPadButton::all() {
            for target in NumPadButton::all() {
                let from_pos = NumericKeypad::POS[source as usize];
                let to_pos = NumericKeypad::POS[target as usize];
                let diff = to_pos - from_pos;
                sequences_by_target_by_source[source as usize][target as usize]
                    .0
                    .extend(
                        position_change_sequences_for(diff)
                            .into_iter()
                            .filter(|path| !path.contains(&NumericKeypad::FORBIDDEN_POS, &from_pos))
                            .map(|path| path.into_iter().map(DirPadButton::from).collect())
                            .map(ButtonSequence),
                    );
            }
        }
        sequences_by_target_by_source
    }
}

impl DirectionalKeypad {
    fn min_sequence_len(
        &self,
        rem_depth: usize,
        sequence: &ButtonSequence,
        cache: &mut HashMap<(usize, ButtonSequence), usize>,
    ) -> usize {
        if rem_depth == 0 {
            sequence.0.len()
        } else if let Some(len) = cache.get(&(rem_depth, sequence.clone())) {
            *len
        } else {
            let mut from_button = DirPadA;
            let mut total_len = 0;
            for to_button in &sequence.0 {
                // enumerate all ways to get from the from_button to the to_button
                let sub_len = self.sequences_by_target_by_source[from_button as usize]
                    [*to_button as usize]
                    .0
                    .iter()
                    .map(|sub_seq| {
                        ButtonSequence(
                            sub_seq
                                .0
                                .iter()
                                .chain(iter::once(&DirPadA))
                                .cloned()
                                .collect(),
                        )
                    })
                    .map(|sub_seq| self.min_sequence_len(rem_depth - 1, &sub_seq, cache))
                    .min()
                    .unwrap();
                total_len += sub_len;
                from_button = *to_button;
            }
            cache.insert((rem_depth, sequence.clone()), total_len);
            total_len
        }
    }
    fn new() -> Self {
        DirectionalKeypad {
            sequences_by_target_by_source:
                DirectionalKeypad::precompute_dir_pad_button_sequences_for_dir_pad(),
        }
    }
    /// Precomputes all possible DirPadButton sequences to get
    /// from any DirPadButton to any other DirPadButton
    fn precompute_dir_pad_button_sequences_for_dir_pad() -> Vec<Vec<ButtonSequences>> {
        // The first index is for the source, the second index is for the target
        let mut sequences_by_target_by_source: Vec<Vec<ButtonSequences>> =
            vec![vec![ButtonSequences(vec![]); 5]; 5];

        for source in DirPadButton::all() {
            for target in DirPadButton::all() {
                let from_pos = DirectionalKeypad::POS[source as usize];
                let to_pos = DirectionalKeypad::POS[target as usize];
                let diff = to_pos - from_pos;
                sequences_by_target_by_source[source as usize][target as usize]
                    .0
                    .extend(
                        position_change_sequences_for(diff)
                            .into_iter()
                            .filter(|path| {
                                !path.contains(&DirectionalKeypad::FORBIDDEN_POS, &from_pos)
                            })
                            .map(|path| path.into_iter().map(DirPadButton::from).collect())
                            .map(ButtonSequence),
                    );
            }
        }
        sequences_by_target_by_source
    }
}

/// Returns all possible paths (as single-step moves) for a given `diff`.
/// For example, a `diff` of `Vec2D{ x: 2, y: -1 }` returns
/// ```rust
/// vec![
///     vec![Right, Right, Up],
///     vec![Right, Up, Right],
///     vec![Up, Right, Right]
/// ]
/// ```
fn position_change_sequences_for(diff: Vec2D) -> Vec<Vec<Vec2D>> {
    let (x_parts, y_parts) = diff.x_and_y_increments();
    combine(&x_parts, &y_parts, vec![])
        .into_iter()
        .map(|moves| moves.into_iter().cloned().collect())
        .collect()
}
/// Returns all possible in-order-combinations from the elements of the left and right list
/// Fox example, with `left = &[Right, Right]` and `right = &[Up]` it returns
/// ```rust
/// vec![
///     vec![Right, Right, Up],
///     vec![Right, Up, Right],
///     vec![Up, Right, Right]
/// ]
/// ```
// There must be an easier way to do this!?
fn combine<'a, T>(left: &'a [T], right: &'a [T], mut combined: Vec<&'a T>) -> Vec<Vec<&'a T>> {
    let mut combinations = Vec::new();
    if left.is_empty() && right.is_empty() {
        combinations.push(combined);
    } else {
        if !left.is_empty() {
            let mut clone = combined.clone();
            clone.push(&left[0]);
            combinations.extend(combine(&left[1..], right, clone));
        }
        if !right.is_empty() {
            combined.push(&right[0]);
            combinations.extend(combine(left, &right[1..], combined));
        }
    }
    combinations
}

// Just to make the filter above a bit more readable
trait ContainsForbiddenPosition {
    fn contains(&self, forbidden_pos: &Vec2D, start: &Vec2D) -> bool;
}
impl ContainsForbiddenPosition for &Vec<Vec2D> {
    fn contains(&self, forbidden_pos: &Vec2D, start: &Vec2D) -> bool {
        let mut intermediate = *start;
        for step in self.iter() {
            intermediate += *step;
            if &intermediate == forbidden_pos {
                return true;
            }
        }
        false
    }
}

fn parse_codes_from(input: &str) -> impl Iterator<Item = Code> + use<'_> {
    input.trim().lines().map(Code::from)
}

impl From<&str> for Code {
    fn from(line: &str) -> Self {
        Code(line.chars().map(NumPadButton::from).collect())
    }
}
impl Code {
    fn numeric_value(&self) -> usize {
        self.0
            .iter()
            .filter_map(|button| button.numeric_value())
            .reduce(|acc, num| acc * 10usize + num)
            .unwrap()
    }
}

impl From<Vec2D> for DirPadButton {
    fn from(dir: Vec2D) -> Self {
        match dir {
            Vec2D::NORTH => Up,
            Vec2D::SOUTH => Down,
            Vec2D::EAST => Right,
            Vec2D::WEST => Left,
            _ => unreachable!(),
        }
    }
}
impl Display for DirPadButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Up => '^',
                DirPadA => 'A',
                Left => '<',
                Down => 'v',
                Right => '>',
            }
        )
    }
}
impl DirPadButton {
    fn all() -> Vec<DirPadButton> {
        vec![Left, Right, Up, Down, DirPadA]
    }
}

impl From<char> for NumPadButton {
    fn from(c: char) -> Self {
        match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'A' => NumPadA,
            _ => unreachable!(),
        }
    }
}
impl Display for NumPadButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.numeric_value()
                .map_or("A".to_string(), |v| v.to_string())
        )
    }
}
impl NumPadButton {
    fn numeric_value(&self) -> Option<usize> {
        match self {
            Zero => Some(0),
            One => Some(1),
            Two => Some(2),
            Three => Some(3),
            Four => Some(4),
            Five => Some(5),
            Six => Some(6),
            Seven => Some(7),
            Eight => Some(8),
            Nine => Some(9),
            NumPadA => None,
        }
    }
    fn all() -> Vec<NumPadButton> {
        vec![
            Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, NumPadA,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
029A
980A
179A
456A
379A
";

    #[test]
    fn test_part1_example() {
        assert_eq!(
            68 * 29 + 60 * 980 + 68 * 179 + 64 * 456 + 64 * 379,
            solve_part1(EXAMPLE)
        );
    }

    #[test]
    fn test_precompute_dir_pad_button_sequences_for_num_pad() {
        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        let sequences_by_target_by_source: Vec<Vec<ButtonSequences>> =
            NumericKeypad::precompute_dir_pad_button_sequences_for_num_pad();
        // single
        assert_eq!(
            sequences_by_target_by_source[Zero as usize][Two as usize],
            ButtonSequences(vec![ButtonSequence(vec![Up])]),
        );
        // double
        assert_eq!(
            sequences_by_target_by_source[Four as usize][Eight as usize],
            ButtonSequences(vec![
                ButtonSequence(vec![Right, Up]),
                ButtonSequence(vec![Up, Right])
            ])
        );
        // would-be-double if not for forbidden excluded
        assert_eq!(
            sequences_by_target_by_source[One as usize][Zero as usize],
            ButtonSequences(vec![ButtonSequence(vec![Right, Down])]),
        );
        // 3x2
        assert_eq!(
            6,
            sequences_by_target_by_source[One as usize][Nine as usize]
                .0
                .len()
        );
        // 3x3
        assert_eq!(
            9,
            sequences_by_target_by_source[Seven as usize][NumPadA as usize]
                .0
                .len()
        );
    }

    #[test]
    fn test_generate_dir_pad_sequences() {
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        let sequences_by_target_by_source: Vec<Vec<ButtonSequences>> =
            DirectionalKeypad::precompute_dir_pad_button_sequences_for_dir_pad();
        // single
        assert_eq!(
            sequences_by_target_by_source[DirPadA as usize][Up as usize],
            ButtonSequences(vec![ButtonSequence(vec![Left])]),
        );
        // double
        assert_eq!(
            sequences_by_target_by_source[Up as usize][Right as usize],
            ButtonSequences(vec![
                ButtonSequence(vec![Right, Down]),
                ButtonSequence(vec![Down, Right])
            ])
        );
        // would-be-double if not for forbidden excluded
        assert_eq!(
            sequences_by_target_by_source[Left as usize][Up as usize],
            ButtonSequences(vec![ButtonSequence(vec![Right, Up])]),
        );
    }

    #[test]
    fn test_part1_shortest_dir_pad_sequences_for_code() {
        let num_pad = NumericKeypad::new();
        let code = Code::from("029A");
        let sequences = num_pad.shortest_dir_pad_sequences_for(&code);
        assert_eq!(3, sequences.0.len());
        assert_eq!(
            vec![
                Left, DirPadA, // 0
                Up, DirPadA, // 2
                Right, Up, Up, DirPadA, // 9
                Down, Down, Down, DirPadA // A
            ],
            sequences.0[0].0
        );
        assert_eq!(
            vec![
                Left, DirPadA, // 0
                Up, DirPadA, // 2
                Up, Right, Up, DirPadA, // 9
                Down, Down, Down, DirPadA // A
            ],
            sequences.0[1].0
        );
        assert_eq!(
            vec![
                Left, DirPadA, // 0
                Up, DirPadA, // 2
                Up, Up, Right, DirPadA, // 9
                Down, Down, Down, DirPadA // A
            ],
            sequences.0[2].0
        );
    }

    #[test]
    fn test_combine() {
        let left = vec![Vec2D::WEST, Vec2D::WEST];
        let right = vec![Vec2D::NORTH, Vec2D::NORTH];
        let actual = combine(&left, &right, vec![]);

        let expected: Vec<Vec<&Vec2D>> = [
            vec![&Vec2D::WEST, &Vec2D::WEST, &Vec2D::NORTH, &Vec2D::NORTH],
            vec![&Vec2D::WEST, &Vec2D::NORTH, &Vec2D::WEST, &Vec2D::NORTH],
            vec![&Vec2D::WEST, &Vec2D::NORTH, &Vec2D::NORTH, &Vec2D::WEST],
            vec![&Vec2D::NORTH, &Vec2D::WEST, &Vec2D::WEST, &Vec2D::NORTH],
            vec![&Vec2D::NORTH, &Vec2D::WEST, &Vec2D::NORTH, &Vec2D::WEST],
            vec![&Vec2D::NORTH, &Vec2D::NORTH, &Vec2D::WEST, &Vec2D::WEST],
        ]
        .into_iter()
        .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_code_numeric_value() {
        assert_eq!(29, Code::from("029A").numeric_value());
    }

    #[test]
    fn test_part1() {
        assert_eq!(206_798, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_single_example() {
        let chain = Chain::new();
        let code = Code::from("029A");
        let min_len = chain.shortest_sequence_len_for(&code, 2);
        assert_eq!(68, min_len)
    }

    #[test]
    fn test_part2_rem_depth_1() {
        let dir_pad = DirectionalKeypad::new();
        let sequence = ButtonSequence(vec![Left, DirPadA]);
        let len = dir_pad.min_sequence_len(1, &sequence, &mut HashMap::new());
        assert_eq!(8, len)
    }

    #[test]
    fn test_part2_rem_depth_2() {
        let dir_pad = DirectionalKeypad::new();
        let sequence = ButtonSequence(vec![Left, DirPadA]);
        let len = dir_pad.min_sequence_len(2, &sequence, &mut HashMap::new());
        assert_eq!(18, len)
    }

    #[test]
    fn test_part2() {
        assert_eq!(251_508_572_750_680, solve_part2(INPUT));
    }
}
