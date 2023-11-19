use std::fmt::{Display, Formatter};

pub(crate) fn evaluate_day18_part1(lines: &[String]) -> usize {
    lines.iter().map(|line| evaluate_line_part1(line)).sum()
}

pub(crate) fn evaluate_day18_part2(lines: &[String]) -> usize {
    lines.iter().map(|line| evaluate_line_part2(line)).sum()
}

#[derive(Debug, PartialEq)]
enum Op {
    Addition,
    Multiplication,
}
impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Addition => write!(f, " + "),
            Op::Multiplication => write!(f, " * "),
        }
    }
}
impl Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Op::Addition,
            "*" => Op::Multiplication,
            _ => panic!("Illegal op \"{}\"", s),
        }
    }
}

#[derive(Debug)]
struct Left {
    left: Option<usize>,
    op: Option<Op>,
}
impl Default for Left {
    fn default() -> Self {
        Left {
            left: None,
            op: None,
        }
    }
}
impl Left {
    fn value(&mut self, right: usize) {
        match self.left {
            // init
            None => {
                assert!(self.op.is_none());
                self.left = Some(right);
            }
            // apply value
            Some(left) => {
                self.left = match self.op {
                    Some(Op::Addition) => Some(left + right),
                    Some(Op::Multiplication) => Some(left * right),
                    None => panic!("Operation missing!"),
                };
                self.op = None;
            }
        }
    }
    fn set_op(&mut self, op: Op) {
        assert!(self.left.is_some());
        self.op = Some(op);
    }
}

fn evaluate_line_part1(line: &dyn AsRef<str>) -> usize {
    let mut result: Left = Left::default();
    let mut intermediates: Vec<Left> = vec![];
    for s in line
        .as_ref()
        .replace("(", "( ")
        .replace(")", " )")
        .split_ascii_whitespace()
    // .inspect(|s| println!("{}", s))
    {
        if let Ok(value) = s.parse() {
            if let Some(intermediate) = intermediates.last_mut() {
                intermediate.value(value);
            } else {
                result.value(value);
            }
        } else {
            match s {
                "(" => intermediates.push(Left::default()),
                ")" => match intermediates.pop() {
                    Some(last_intermediate) => {
                        if let Some(prev_intermediate) = intermediates.last_mut() {
                            prev_intermediate.value(last_intermediate.left.unwrap());
                        } else {
                            result.value(last_intermediate.left.unwrap());
                        }
                    }
                    expected => panic!("Expected match for {:?}", expected),
                },
                op @ "+" | op @ "*" => {
                    if let Some(intermediate) = intermediates.last_mut() {
                        intermediate.set_op(Op::from(op));
                    } else {
                        result.set_op(Op::from(op));
                    }
                }
                s => {
                    panic!("Invalid string \"{}\"", s);
                }
            };
        }
    }
    if let Some(left) = result.left {
        left
    } else {
        panic!("No result")
    }
}

#[derive(Debug)]
struct LeftTwo {
    left: Option<usize>,
    middle: Option<usize>,
    op1: Option<Op>,
    op2: Option<Op>,
}
impl Default for LeftTwo {
    fn default() -> Self {
        LeftTwo {
            left: None,
            middle: None,
            op1: None,
            op2: None,
        }
    }
}
impl LeftTwo {
    fn add_value(&mut self, right: usize) {
        match (self.left, self.middle) {
            // Init
            (None, None) => {
                assert!(self.op1.is_none());
                self.left = Some(right);
            }
            (Some(left), None) => match self.op1 {
                None => panic!("op1 not set when adding second value"),
                Some(Op::Addition) => {
                    // a + b -> Simplify to left: (a + b)
                    self.left = Some(left + right);
                    self.op1 = None;
                }
                // Next op could be an addition, which should take precedence
                Some(Op::Multiplication) => self.middle = Some(right),
            },
            (Some(left), Some(middle)) => {
                assert_eq!(self.op1, Some(Op::Multiplication));
                match self.op2 {
                    None => panic!("op2 not set when adding third value"),
                    Some(Op::Addition) => {
                        // a * b + c -> Simplify to middle: a * (b + c)
                        self.middle = Some(middle + right);
                        self.op2 = None;
                    }
                    Some(Op::Multiplication) => {
                        // a * b * c -> Simplify to middle: (a * b) * c
                        self.left = Some(left * middle);
                        self.middle = Some(right);
                        self.op2 = None;
                    }
                }
            }
            (None, Some(middle)) => panic!("Illegal state (None, {})", middle),
        }
    }
    fn set_op(&mut self, op: Op) {
        assert!(self.left.is_some());
        match (&self.op1, &self.op2) {
            // Init
            (None, None) => self.op1 = Some(op),
            (Some(_), None) => self.op2 = Some(op),
            (None, Some(op2)) => panic!("Illegal state (None, {})", op2),
            (Some(op1), Some(op2)) => panic!("Illegal state ({}, {})", op1, op2),
        }
    }
    fn get_value(&self) -> usize {
        match (&self.op1, &self.op2) {
            (None, None) => {
                assert!(self.middle.is_none());
                match self.left {
                    None => panic!("Missing left on empty op1 & op2"),
                    Some(left) => left,
                }
            }
            (Some(op1), None) => match (self.left, self.middle) {
                (None, None) => panic!("Illegal state (None, None) with op1 {}", op1),
                (Some(left), None) => panic!("Illegal state ({}, None) with op1 {}", left, op1),
                (Some(left), Some(middle)) => match op1 {
                    Op::Addition => left + middle,
                    Op::Multiplication => left * middle,
                },
                (None, Some(middle)) => panic!("Illegal state (None, {}) with op1 {}", middle, op1),
            },
            (Some(op1), Some(op2)) => panic!("Illegal state with ops ({}, {})", op1, op2),
            (None, Some(op2)) => panic!("Illegal state (None, {})", op2),
        }
    }
}

fn evaluate_line_part2(line: &dyn AsRef<str>) -> usize {
    let mut result = LeftTwo::default();
    let mut intermediates: Vec<LeftTwo> = vec![];
    for s in line
        .as_ref()
        .replace("(", "( ")
        .replace(")", " )")
        .split_ascii_whitespace()
    // .inspect(|s| println!("{}", s))
    {
        if let Ok(value) = s.parse() {
            if let Some(intermediate) = intermediates.last_mut() {
                intermediate.add_value(value);
            } else {
                result.add_value(value);
            }
        } else {
            match s {
                "(" => intermediates.push(LeftTwo::default()),
                ")" => match intermediates.pop() {
                    Some(last_intermediate) => {
                        if let Some(prev_intermediate) = intermediates.last_mut() {
                            prev_intermediate.add_value(last_intermediate.get_value());
                        } else {
                            result.add_value(last_intermediate.get_value());
                        }
                    }
                    expected => panic!("Expected match for {:?}", expected),
                },
                op @ "+" | op @ "*" => {
                    if let Some(intermediate) = intermediates.last_mut() {
                        intermediate.set_op(Op::from(op));
                    } else {
                        result.set_op(Op::from(op));
                    }
                }
                s => {
                    panic!("Invalid string \"{}\"", s);
                }
            };
        }
        // println!("{:?}", result);
    }
    result.get_value()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line_reader::read_file_to_lines;

    #[test]
    fn part1_example_1() {
        assert_eq!(evaluate_line_part1(&"1"), 1);
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(evaluate_line_part1(&"1 + 2 * 3"), 9);
    }
    #[test]
    fn part1_example_3() {
        assert_eq!(evaluate_line_part1(&"2 * 3 + 4"), 10);
    }
    #[test]
    fn part1_example_4() {
        assert_eq!(evaluate_line_part1(&"1 + 2 * 3 + 4 * 5 + 6"), 71);
    }
    #[test]
    fn part1_example_5() {
        assert_eq!(evaluate_line_part1(&"1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }
    #[test]
    fn part1_example_6() {
        assert_eq!(evaluate_line_part1(&"2 * 3 + (4 * 5)"), 26);
    }
    #[test]
    fn part1_example_7() {
        assert_eq!(evaluate_line_part1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    }
    #[test]
    fn part1_example_8() {
        assert_eq!(
            evaluate_line_part1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
    }
    #[test]
    fn part1_example_9() {
        assert_eq!(
            evaluate_line_part1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
    #[test]
    fn part1_example_10() {
        assert_eq!(evaluate_line_part1(&"(1 * 2)"), 2);
    }
    #[test]
    fn part1_example_11() {
        assert_eq!(evaluate_line_part1(&"((1 * 2))"), 2);
    }
    #[test]
    fn part1() {
        assert_eq!(
            evaluate_day18_part1(&read_file_to_lines("input/day18.txt")),
            3348222486398
        );
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(evaluate_line_part2(&"1"), 1);
    }
    #[test]
    fn part2_example_2() {
        assert_eq!(evaluate_line_part2(&"1 + 2 * 3"), 9);
    }
    #[test]
    fn part2_example_3() {
        assert_eq!(evaluate_line_part2(&"2 * 3 + 4"), 14);
    }
    #[test]
    fn part2_example_4() {
        assert_eq!(evaluate_line_part2(&"1 + 2 * 3 + 4 * 5 + 6"), 231);
    }
    #[test]
    fn part2_example_5() {
        assert_eq!(evaluate_line_part2(&"1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }
    #[test]
    fn part2_example_6() {
        assert_eq!(evaluate_line_part2(&"2 * 3 + (4 * 5)"), 46);
    }
    #[test]
    fn part2_example_7() {
        assert_eq!(evaluate_line_part2(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    }
    #[test]
    fn part2_example_8() {
        assert_eq!(
            evaluate_line_part2(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
    }
    #[test]
    fn part2_example_9() {
        assert_eq!(
            evaluate_line_part2(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
    #[test]
    fn part2_example_10() {
        assert_eq!(evaluate_line_part2(&"(1 * 2)"), 2);
    }
    #[test]
    fn part2_example_11() {
        assert_eq!(evaluate_line_part2(&"((1 * 2))"), 2);
    }
    #[test]
    fn part2() {
        assert_eq!(
            evaluate_day18_part2(&read_file_to_lines("input/day18.txt")),
            43423343619505
        );
    }
}
