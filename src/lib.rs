use std::fmt::{Display, Formatter};

mod tests;

fn evaluate_part1(lines: &[String]) -> usize {
    lines.iter().map(|line| evaluate_line_part1(line)).sum()
}

#[derive(Debug)]
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

#[derive(PartialEq, Debug)]
enum Parens {
    Open,
    Close,
}
impl Display for Parens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Parens::Open => write!(f, "("),
            Parens::Close => write!(f, ")"),
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
            Some(left) => {
                self.left = match self.op {
                    Some(Op::Addition) => Some(left + right),
                    Some(Op::Multiplication) => Some(left * right),
                    None => panic!("Operation missing!"),
                };
                self.op = None;
            }
            None => {
                assert!(self.op.is_none());
                self.left = Some(right);
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
