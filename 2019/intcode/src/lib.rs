use std::collections::VecDeque;
use Mode::*;
use Op::*;

pub const TEST_MODE_INPUT: isize = 1;
const PRINT_INT_CODE_COMPUTER_OUTPUT: bool = false;

#[derive(Debug)]
pub enum State {
    Idle,
    ExpectingInput,
    WroteOutput(isize),
    Halted,
}

#[derive(Debug)]
pub struct IntCodeComputer {
    pub(crate) instr: Vec<isize>,        // program
    pub(crate) ptr: usize,               // instruction pointer
    base: isize,                         // relative base
    pub inputs: VecDeque<isize>,         // input queue to read from when needed
    pub(crate) outputs: VecDeque<isize>, // outputs it generated
}
impl IntCodeComputer {
    pub fn new(instr: Vec<isize>) -> Self {
        IntCodeComputer {
            instr,
            ptr: 0,
            base: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }
    pub fn outputs(&self) -> VecDeque<isize> {
        self.outputs.clone()
    }
    pub fn add_input(&mut self, i: isize) {
        self.inputs.push_back(i);
    }
    pub fn add_inputs(&mut self, iq: &[isize]) {
        self.inputs.append(&mut VecDeque::from(iq.to_vec()));
    }
    fn next_op_as_5_digit_string_padded_with_leading_zeroes(&self) -> String {
        let s = self.instr[self.ptr].to_string();
        "0".repeat(5 - s.len()) + s.as_ref()
    }
    pub fn get(&mut self, idx: usize) -> isize {
        self.grow_if_needed(idx);
        self.instr[idx]
    }
    pub fn set(&mut self, idx: usize, val: isize) {
        self.grow_if_needed(idx);
        self.instr[idx] = val;
    }
    fn grow_if_needed(&mut self, idx: usize) {
        if idx >= self.instr.len() {
            let diff = 1 + idx - self.instr.len();
            if diff > 10000 && PRINT_INT_CODE_COMPUTER_OUTPUT {
                println!(
                    "old size = {}, idx = {}, huge diff = {}",
                    self.instr.len(),
                    idx,
                    diff
                );
            }
            self.instr.extend(vec![0; diff]);
        }
    }
    fn value(&mut self, offset: usize, mode: &Mode) -> isize {
        let val = self.get(self.ptr + offset);
        match mode {
            Immediate => val,
            Position => self.get(val as usize),
            Relative => self.get((self.base + val) as usize),
        }
    }
    fn set_result(&mut self, offset: usize, mode: &Mode, res: isize) {
        let val = self.get(self.ptr + offset);
        if PRINT_INT_CODE_COMPUTER_OUTPUT {
            println!(" [{}] = {}", val, res);
        }
        match mode {
            Immediate => panic!("Output parameter in immediate mode!"),
            Position => self.set(val as usize, res),
            Relative => self.set((self.base + val) as usize, res),
        }
    }

    pub fn process_int_code_with_default_input(&mut self) -> Option<isize> {
        self.process_int_code_with_input(TEST_MODE_INPUT)
    }
    pub fn process_int_code_with_input(&mut self, input: isize) -> Option<isize> {
        self.inputs.push_back(input);
        self.run_until_halted()
    }
    // needed for days 13 and 15
    pub fn process_int_code_until_first_output(&mut self, input: isize) -> Option<isize> {
        if !self.inputs.is_empty() {
            self.inputs.clear();
        }
        self.inputs.push_back(input);
        self.run_until_first_output()
    }
    pub fn run_until_halted(&mut self) -> Option<isize> {
        if self.ptr >= self.instr.len() && PRINT_INT_CODE_COMPUTER_OUTPUT {
            println!("ptr >= len");
        }
        while !self.is_halted() {
            match self.step() {
                State::Idle => (),
                State::ExpectingInput => (),
                State::WroteOutput(out) => self.outputs.push_back(out),
                State::Halted => return self.outputs.back().cloned(),
            }
        }
        None
    }
    pub fn is_halted(&self) -> bool {
        self.ptr >= self.instr.len()
    }
    // needed for day13
    fn run_until_first_output(&mut self) -> Option<isize> {
        if self.ptr >= self.instr.len() && PRINT_INT_CODE_COMPUTER_OUTPUT {
            println!("ptr >= len");
        }
        while !self.is_halted() {
            match self.step() {
                State::Idle => (),
                State::ExpectingInput => (),
                State::WroteOutput(out) => return Some(out),
                State::Halted => return None,
            }
        }
        None
    }
    // needed for day17
    pub fn run_until_waiting_for_input(&mut self) {
        let mut line = vec![];
        while self.ptr < self.instr.len() {
            match self.step() {
                State::Idle => (),
                State::ExpectingInput => return,
                State::WroteOutput(output) => {
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        let c = output as u8 as char;
                        if c == '\n' {
                            println!("{}", line.into_iter().collect::<String>());
                            line = vec![];
                        } else {
                            line.push(c);
                        }
                    }
                }
                State::Halted => (),
            }
        }
    }
    pub fn step(&mut self) -> State {
        let s = self.next_op_as_5_digit_string_padded_with_leading_zeroes();
        let code = str_to_num(&s[(s.len() - 2)..s.len()]);
        let op = Op::from(code);
        let modes = Mode::extract_modes(&s);
        let pre = format!("{:?}: {:?}", s, op);
        match op {
            Add | Multiply | LessThan | Equals => {
                let p1 = self.value(1, &modes[0]);
                let p2 = self.value(2, &modes[1]);
                let res = match op {
                    Add => p1 + p2,
                    Multiply => p1 * p2,
                    LessThan => eval(p1 < p2),
                    Equals => eval(p1 == p2),
                    _ => unreachable!(),
                };
                // print!("{}({}, {})", pre, p1, p2);
                self.set_result(3, &modes[2], res);
                self.ptr += op.value_count();
                State::Idle
            }
            Input => {
                if PRINT_INT_CODE_COMPUTER_OUTPUT {
                    println!("{}", pre);
                }
                if let Some(input) = self.inputs.pop_front() {
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        println!("Consuming input {} ({})", input, input as u8 as char);
                    }
                    self.set_result(1, &modes[0], input);
                    self.ptr += op.value_count();
                    State::Idle
                } else {
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        // println!("Waiting for input, ptr = {}", self.ptr);
                        panic!("Waiting for input, ptr = {}", self.ptr);
                    }
                    State::ExpectingInput
                }
            }
            Output => {
                let value = self.value(1, &modes[0]);
                if PRINT_INT_CODE_COMPUTER_OUTPUT {
                    println!("{} = {}", pre, value);
                }
                self.ptr += op.value_count();
                State::WroteOutput(value)
            }
            ShiftRelativeBase => {
                let shift = self.value(1, &modes[0]);
                let old_base = self.base;
                self.base += shift;
                if PRINT_INT_CODE_COMPUTER_OUTPUT {
                    println!("{} by {} from {} to {}", pre, shift, old_base, self.base);
                }
                self.ptr += op.value_count();
                State::Idle
            }
            JumpIfTrue | JumpIfFalse => {
                let p1 = self.value(1, &modes[0]);
                let p2 = self.value(2, &modes[1]);
                if op == JumpIfTrue && p1 != 0 || op == JumpIfFalse && p1 == 0 {
                    self.ptr = p2 as usize;
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        println!("{} ({}) == true -> jump to {}", pre, p1, p2);
                    }
                    // Not increasing self.ptr here to actually jump
                } else {
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        println!("{} ({}) == false -> NO jump (to {})", pre, p1, p2);
                    }
                    self.ptr += op.value_count();
                }
                State::Idle
            }
            Stop => {
                if PRINT_INT_CODE_COMPUTER_OUTPUT {
                    println!("{}", pre);
                }
                State::Halted
            }
        }
    }
    pub fn instr(&self) -> &Vec<isize> {
        &self.instr
    }
}
pub fn str_to_num(s: &str) -> isize {
    s.parse::<isize>().unwrap()
}
pub fn eval(b: bool) -> isize {
    if b {
        1
    } else {
        0
    }
}

#[derive(PartialEq, Debug)]
pub enum Op {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    ShiftRelativeBase = 9,
    Stop = 99,
}

impl Op {
    pub fn value_count(&self) -> usize {
        match self {
            Add | Multiply | LessThan | Equals => 4,
            JumpIfTrue | JumpIfFalse => 3,
            Input | Output | ShiftRelativeBase => 2,
            Stop => 1,
        }
    }
}
impl From<isize> for Op {
    fn from(code: isize) -> Self {
        match code {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => ShiftRelativeBase,
            99 => Stop,
            _ => panic!("Unknown Op code {:?}", code),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}
impl Mode {
    pub fn extract_modes(s: &str) -> Vec<Mode> {
        vec![
            Mode::from(str_to_num(&s[2..=2])),
            Mode::from(str_to_num(&s[1..=1])),
            Mode::from(str_to_num(&s[0..=0])),
        ]
    }
}
impl From<isize> for Mode {
    fn from(code: isize) -> Self {
        match code {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Unknown Mode code {:?}", code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autoextend_on_get() {
        let mut icc = IntCodeComputer::new(&mut vec![]);
        assert_eq!(icc.get(0), 0);
        assert_eq!(icc.instr.len(), 1);
    }

    #[test]
    fn test_autoextend_on_set() {
        let mut icc = IntCodeComputer::new(&mut vec![]);
        icc.set(0, 123);
        assert_eq!(icc.instr.len(), 1);
        assert_eq!(icc.get(0), 123);
    }

    #[test]
    fn day9_part1_example1() {
        let mut icc = IntCodeComputer::new(&mut vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(99));
    }

    #[test]
    fn day9_part1_example2() {
        let mut icc = IntCodeComputer::new(&mut vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        assert_eq!(
            icc.process_int_code_with_default_input(),
            Some(1219070632396864)
        );
    }

    #[test]
    fn day9_part1_example3() {
        let mut icc = IntCodeComputer::new(&mut vec![104, 1125899906842624, 99]);
        assert_eq!(
            icc.process_int_code_with_default_input(),
            Some(1125899906842624)
        );
    }

    #[test]
    fn day9_add_with_relative_input() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 2201, 1, 2, 9, 99, 11, 22, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![109, 6, 2201, 1, 2, 9, 99, 11, 22, 33]);
    }

    #[test]
    fn day9_add_with_relative_output() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 21101, 11, 22, 1, 99, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![109, 6, 21101, 11, 22, 1, 99, 33]);
    }

    #[test]
    fn day9_part1_output_with_relative_base_above_0() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 10, 204, -5, 99, 123]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(123));
    }
    #[test]
    fn day9_part1_output_with_relative_base_below_0() {
        let mut icc = IntCodeComputer::new(&mut vec![109, -5, 204, 10, 99, 123]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(123));
    }

    #[test]
    fn day9_part1_mirror_relative_input_to_output() {
        let mut icc = IntCodeComputer::new(&mut vec![203, 3, 104, 0, 99]);
        assert_eq!(icc.process_int_code_with_input(22), Some(22));
    }

    #[test]
    fn day9_part1_mirror_shifted_relative_input_to_output() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 203, -1, 104, 0, 99]);
        if PRINT_INT_CODE_COMPUTER_OUTPUT {
            println!("{:?}", icc.instr);
        }
        assert_eq!(icc.process_int_code_with_input(33), Some(33));
    }

    // day 5 below

    #[test]
    fn ops_from_int_code() {
        assert_eq!(Op::Add, Op::from(1));
        assert_eq!(Op::Multiply, Op::from(2));
        assert_eq!(Op::Input, Op::from(3));
        assert_eq!(Op::Output, Op::from(4));
        assert_eq!(Op::JumpIfTrue, Op::from(5));
        assert_eq!(Op::JumpIfFalse, Op::from(6));
        assert_eq!(Op::LessThan, Op::from(7));
        assert_eq!(Op::Equals, Op::from(8));
        assert_eq!(Op::ShiftRelativeBase, Op::from(9));
        assert_eq!(Op::Stop, Op::from(99));
    }

    #[test]
    fn explanation_example() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(
            icc.instr,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn add_example_1() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 0, 0, 0, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn mult_example_1() {
        let mut icc = IntCodeComputer::new(&mut vec![2, 3, 0, 3, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn mult_example_2() {
        let mut icc = IntCodeComputer::new(&mut vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn add_example_2() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    // day 5 part 1

    #[test]
    fn multiply_example() {
        let mut icc = IntCodeComputer::new(&mut vec![1002, 4, 3, 4, 33]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![1002, 4, 3, 4, 99]);
    }

    // day 5 part 2

    #[test]
    fn input_equal_to_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(9), Some(0));
    }
    #[test]
    fn input_less_than_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(7), Some(1));
    }
    #[test]
    fn input_not_less_than_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(0));
    }
    #[test]
    fn input_equal_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(9), Some(0));
    }
    #[test]
    fn input_less_than_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(icc.process_int_code_with_input(7), Some(1));
    }
    #[test]
    fn input_not_less_than_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(icc.process_int_code_with_input(8), Some(0));
    }
    #[test]
    fn jump_test_position_mode_1() {
        let mut icc = IntCodeComputer::new(&mut vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        assert_eq!(icc.process_int_code_with_input(1), Some(1));
    }
    #[test]
    fn jump_test_position_mode_0() {
        let mut icc = IntCodeComputer::new(&mut vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        assert_eq!(icc.process_int_code_with_input(0), Some(0));
    }
    #[test]
    fn jump_test_immediate_mode_1() {
        let mut icc =
            IntCodeComputer::new(&mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(icc.process_int_code_with_input(1), Some(1));
    }
    #[test]
    fn jump_test_immediate_mode_0() {
        let mut icc =
            IntCodeComputer::new(&mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(icc.process_int_code_with_input(0), Some(0));
    }

    fn larger_example_input() -> Vec<isize> {
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
    }
    #[test]
    fn larger_example_less_than_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(7), Some(999));
    }
    #[test]
    fn larger_example_exactly_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(8), Some(1000));
    }
    #[test]
    fn larger_example_greater_than_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(9), Some(1001));
    }
}
