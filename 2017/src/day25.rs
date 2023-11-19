use crate::parse;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input/day25.txt");

pub(crate) fn day25_part1() -> usize {
    diagnostic_checksum(parse(INPUT))
}

type State = char;

#[derive(Clone, PartialEq)]
enum Value {
    Zero,
    One,
}
impl From<char> for Value {
    fn from(c: char) -> Self {
        match c {
            '0' => Value::Zero,
            '1' => Value::One,
            _ => panic!("Unknown value {}", c),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Zero
    }
}

#[derive(Clone)]
enum Dir {
    Left,
    Right,
}
impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "left." => Dir::Left,
            "right." => Dir::Right,
            _ => panic!("Unknown dir {}", s),
        }
    }
}

type Transition = (Value, Dir, State);

struct TouringMachine {
    cursor: usize,
    tape: VecDeque<Value>,
    curr_state: State,
    program: Program,
    steps_until_checksum: usize,
}
impl From<Vec<&str>> for TouringMachine {
    fn from(input: Vec<&str>) -> Self {
        let curr_state = input[0]
            .trim_start_matches("Begin in state ")
            .chars()
            .next()
            .unwrap();
        let steps_until_checksum = input[1]
            .split_ascii_whitespace()
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        let program: Vec<_> = input.split(|line| line.is_empty()).skip(1).collect();
        TouringMachine {
            cursor: 0,
            tape: vec![Value::default()].iter().cloned().collect(),
            curr_state,
            program: Program::from(program),
            steps_until_checksum,
        }
    }
}
impl TouringMachine {
    fn check_sum(&mut self) -> usize {
        let mut steps = 0;
        while steps < self.steps_until_checksum {
            steps += 1;
            let value = &self.tape[self.cursor];
            let (value, dir, state) = self.program.transition(&self.curr_state, value);
            self.tape[self.cursor] = value;
            self.move_into(dir);
            self.curr_state = state;
        }
        self.tape.iter().filter(|&v| v == &Value::One).count()
    }

    fn move_into(&mut self, dir: Dir) {
        match dir {
            Dir::Left => {
                if self.cursor == 0 {
                    self.tape.push_front(Value::default());
                } else {
                    self.cursor -= 1;
                }
            }
            Dir::Right => {
                self.cursor += 1;
                if self.cursor == self.tape.len() {
                    self.tape.push_back(Value::default());
                }
            }
        };
    }
}

struct Program {
    program: Vec<[Transition; 2]>,
}
impl From<Vec<&[&str]>> for Program {
    fn from(states: Vec<&[&str]>) -> Self {
        Program {
            program: states
                .iter()
                .filter(|state| !state.is_empty())
                .map(|lines| {
                    [
                        Program::transition_from(&lines[2..=4]),
                        Program::transition_from(&lines[6..=8]),
                    ]
                })
                .collect(),
        }
    }
}
impl Program {
    fn transition(&self, curr_state: &State, curr_value: &Value) -> Transition {
        let idx = ((*curr_state as u8) - b'A') as usize;
        // println!("state {} idx {}", curr_state, idx);
        let value = match curr_value {
            Value::Zero => 0,
            Value::One => 1,
        };
        self.program[idx][value].clone()
    }
    fn transition_from(desc: &[&str]) -> Transition {
        let value = Value::from(
            desc[0]
                .trim_start_matches("    - Write the value ")
                .chars()
                .next()
                .unwrap(),
        );
        let dir = Dir::from(desc[1].trim_start_matches("    - Move one slot to the "));
        let state = desc[2]
            .trim_start_matches("    - Continue with state ")
            .chars()
            .next()
            .unwrap();
        (value, dir, state)
    }
}

fn diagnostic_checksum(input: Vec<&str>) -> usize {
    let mut touring_machine = TouringMachine::from(input);
    touring_machine.check_sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

    #[test]
    fn example_part1() {
        assert_eq!(3, diagnostic_checksum(parse(EXAMPLE)));
    }

    #[test]
    fn part1() {
        assert_eq!(2474, day25_part1());
    }
}
