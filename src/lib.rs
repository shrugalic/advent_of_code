use permutohedron::heap_recursive;
use rayon::prelude::*;
use std::cmp::min;
use Mode::*;
use Op::*;

const DEFAULT_INPUT: i32 = 1;
const PRINT_OPS: bool = false;

fn to_5_digit_string_padded_with_leading_zeroes(n: i32) -> String {
    let s = n.to_string();
    "0".repeat(5 - s.len()) + s.as_ref()
}

fn to_num(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn extract_modes(s: &str) -> Vec<Mode> {
    vec![
        Mode::from_code(to_num(&s[2..=2])),
        Mode::from_code(to_num(&s[1..=1])),
        Mode::from_code(to_num(&s[0..=0])),
    ]
}

fn param_value(v: &Vec<i32>, idx: usize, mode: &Mode) -> i32 {
    match mode {
        POSITION => v[v[idx] as usize],
        IMMEDIATE => v[idx],
    }
}

fn process_int_code(v: &mut Vec<i32>) -> Option<i32> {
    process_int_code_with_input(v, DEFAULT_INPUT)
}

fn eval(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}

fn max_thrust_in_serial_mode(prg: &Vec<i32>) -> i32 {
    let phases = [0, 1, 2, 3, 4];
    permutations_of(phases)
        .iter()
        .map(|seq| calc_thrust(prg, seq, true))
        .max()
        .unwrap()
}

fn max_thrust_in_feedback_loop_mode(prg: &Vec<i32>) -> i32 {
    let phases = [5, 6, 7, 8, 9];
    permutations_of(phases)
        .par_iter()
        .map(|seq| calc_thrust(prg, seq, false))
        .max()
        .unwrap()
}

fn permutations_of(mut phases: [i32; 5]) -> Vec<Vec<i32>> {
    let mut phase_sequences = Vec::new();
    heap_recursive(&mut phases, |seq| phase_sequences.push(seq.to_vec()));
    phase_sequences
}

#[derive(Debug)]
struct Amplifier {
    instructions: Vec<i32>, //instructions
    ptr: usize,             // instruction pointer
}
impl Amplifier {
    fn process(&mut self, inputs: Vec<i32>) -> Option<i32> {
        let mut input_idx = 0;
        let mut output = None;
        while self.ptr < self.instructions.len() {
            let s = to_5_digit_string_padded_with_leading_zeroes(self.instructions[self.ptr]);
            let code = to_num(&s[(s.len() - 2)..s.len()]);
            let op = Op::from_code(code);
            let modes = extract_modes(&s);
            let pre = format!("{:?}: {:?}", s, op);
            match op {
                Add | Multiply | LessThan | Equals => {
                    let p1 = param_value(&self.instructions, self.ptr + 1, &modes[0]);
                    let p2 = param_value(&self.instructions, self.ptr + 2, &modes[1]);
                    let res = match op {
                        Add => p1 + p2,
                        Multiply => p1 * p2,
                        LessThan => eval(p1 < p2),
                        Equals => eval(p1 == p2),
                        _ => unreachable!(),
                    };
                    // Store result
                    let res_idx = self.instructions[self.ptr + 3] as usize;
                    if PRINT_OPS {
                        println!("{} ({}, {}) -> v[{}] = {}", pre, p1, p2, res_idx, res);
                    }
                    self.instructions[res_idx] = res;
                }
                Input => {
                    let res_idx = self.instructions[self.ptr + 1] as usize;
                    // avoid out-of-bounds by repeatedly taking last input if necessary
                    let input = inputs[min(input_idx, inputs.len() - 1)];
                    input_idx += 1;
                    if PRINT_OPS {
                        println!("{} -> v[{}] = {}", pre, res_idx, input);
                    }
                    self.instructions[res_idx] = input;
                }
                Output => {
                    let p1 = param_value(&self.instructions, self.ptr + 1, &modes[0]);
                    self.ptr += op.value_count();
                    if PRINT_OPS {
                        println!("{} = {}", pre, p1);
                    }
                    return Some(p1);
                }
                JumpIfTrue | JumpIfFalse => {
                    let p1 = param_value(&self.instructions, self.ptr + 1, &modes[0]);
                    let p2 = param_value(&self.instructions, self.ptr + 2, &modes[1]);
                    if op == JumpIfTrue && p1 != 0 || op == JumpIfFalse && p1 == 0 {
                        self.ptr = p2 as usize;
                        if PRINT_OPS {
                            println!("{} ({}) == true -> jump to {}", pre, p1, p2);
                        }
                        continue; // to jump here, rather than increasing idx below
                    }
                }
                Stop => {
                    if PRINT_OPS {
                        println!("{}", pre);
                    }
                    return None;
                }
            }
            self.incr_ptr(op);
        }
        output
    }

    fn incr_ptr(&mut self, op: Op) {
        self.ptr += op.value_count();
    }
}

fn calc_thrust(prg: &Vec<i32>, seq: &Vec<i32>, single_run: bool) -> i32 {
    // Init & run through each amplifier once
    let mut signal = 0;
    let mut amplifiers: Vec<Amplifier> = vec![];
    for &phase in seq {
        let mut amp = Amplifier {
            instructions: prg.clone(),
            ptr: 0,
        };
        let inputs: Vec<i32> = vec![phase, signal];
        if let Some(output) = amp.process(inputs) {
            signal = output;
        }
        amplifiers.push(amp);
    }
    if single_run {
        return signal;
    }
    // Run repeatedly
    loop {
        let mut loop_cnt = 0;
        let mut finished = false;
        for idx in 0..5 {
            let inputs: Vec<i32> = vec![signal];
            if let Some(output) = amplifiers[idx].process(inputs) {
                signal = output;
            } else {
                finished = true;
            }
        }
        loop_cnt += 1;
        if finished {
            break;
        }
    }
    signal
}

fn process_int_code_with_input(instructions: &mut Vec<i32>, input: i32) -> Option<i32> {
    let inputs = vec![input];
    let mut amp = Amplifier {
        instructions: instructions.to_vec(),
        ptr: 0,
    };
    amp.process(inputs)
}

#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Stop,
}

impl Op {
    fn from_code(code: i32) -> Op {
        match code {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            99 => Stop,
            _ => panic!("Unknown Op code {:?}", code),
        }
    }
    fn value_count(&self) -> usize {
        match self {
            Add | Multiply | LessThan | Equals => 4,
            JumpIfTrue | JumpIfFalse => 3,
            Input | Output => 2,
            Stop => 1,
        }
    }
}

#[derive(Debug)]
enum Mode {
    POSITION,
    IMMEDIATE,
}

impl Mode {
    fn from_code(code: i32) -> Mode {
        match code {
            0 => POSITION,
            1 => IMMEDIATE,
            _ => panic!("Unknown Mode code {:?}", code),
        }
    }
}

mod tests {
    use crate::{
        max_thrust_in_feedback_loop_mode, max_thrust_in_serial_mode, process_int_code_with_input,
    };

    // day 7 part 1

    #[test]
    fn day7_part1_example_1() {
        let mut input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(max_thrust_in_serial_mode(&mut input), 43210);
    }

    #[test]
    fn day7_part1_example_2() {
        let mut input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(max_thrust_in_serial_mode(&mut input), 54321);
    }
    #[test]
    fn day7_part1_example_3() {
        let mut input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(max_thrust_in_serial_mode(&mut input), 65210);
    }

    #[test]
    fn day7_part_1() {
        let mut v = puzzle_input();
        assert_eq!(max_thrust_in_serial_mode(&mut v), 87138);
    }

    // day 7 part 1

    #[test]
    fn day7_part2_example_1() {
        let mut input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(max_thrust_in_feedback_loop_mode(&mut input), 139629729);
    }
    #[test]
    fn day7_part2_example_2() {
        let mut input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(max_thrust_in_feedback_loop_mode(&mut input), 18216);
    }
    #[test]
    fn day7_part2() {
        let mut input = puzzle_input();
        assert_eq!(max_thrust_in_feedback_loop_mode(&mut input), 17279674);
    }

    // puzzle input

    fn puzzle_input() -> Vec<i32> {
        vec![
            3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 67, 76, 101, 118, 199, 280, 361, 442,
            99999, 3, 9, 1002, 9, 4, 9, 1001, 9, 2, 9, 102, 3, 9, 9, 101, 3, 9, 9, 102, 2, 9, 9, 4,
            9, 99, 3, 9, 1001, 9, 3, 9, 102, 2, 9, 9, 1001, 9, 2, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9,
            101, 3, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 1002, 9, 5, 9, 101, 5, 9, 9, 1002, 9, 4,
            9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 1001, 9, 5, 9, 102, 2, 9, 9, 4, 9, 99,
            3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
            101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
            2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4,
            9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
            101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2,
            9, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2,
            9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
            3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
            101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
            101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2,
            9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
            9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
            3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002,
            9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99,
        ]
    }

    // day 5 part 2

    #[test]
    fn input_equal_to_8_position_mode() {
        let mut v = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(process_int_code_with_input(&mut v, 8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_position_mode() {
        let mut v = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(process_int_code_with_input(&mut v, 9), Some(0));
    }
    #[test]
    fn input_less_than_8_position_mode() {
        let mut v = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(process_int_code_with_input(&mut v, 7), Some(1));
    }
    #[test]
    fn input_not_less_than_8_position_mode() {
        let mut v = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(process_int_code_with_input(&mut v, 8), Some(0));
    }
    #[test]
    fn input_equal_to_8_immediate_mode() {
        let mut v = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8];
        assert_eq!(process_int_code_with_input(&mut v, 8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_immediate_mode() {
        let mut v = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8];
        assert_eq!(process_int_code_with_input(&mut v, 9), Some(0));
    }
    #[test]
    fn input_less_than_to_8_immediate_mode() {
        let mut v = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(process_int_code_with_input(&mut v, 7), Some(1));
    }
    #[test]
    fn input_not_less_than_to_8_immediate_mode() {
        let mut v = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(process_int_code_with_input(&mut v, 8), Some(0));
    }
    #[test]
    fn jump_test_position_mode_1() {
        let mut v = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(process_int_code_with_input(&mut v, 1), Some(1));
    }
    #[test]
    fn jump_test_position_mode_0() {
        let mut v = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(process_int_code_with_input(&mut v, 0), Some(0));
    }
    #[test]
    fn jump_test_immediate_mode_1() {
        let mut v = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(process_int_code_with_input(&mut v, 1), Some(1));
    }
    #[test]
    fn jump_test_immediate_mode_0() {
        let mut v = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(process_int_code_with_input(&mut v, 0), Some(0));
    }

    fn larger_example_input() -> Vec<i32> {
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
    }
    #[test]
    fn larger_example_less_than_8() {
        let mut v = larger_example_input();
        assert_eq!(process_int_code_with_input(&mut v, 7), Some(999));
    }
    #[test]
    fn larger_example_exactly_8() {
        let mut v = larger_example_input();
        assert_eq!(process_int_code_with_input(&mut v, 8), Some(1000));
    }
    #[test]
    fn larger_example_greater_than_8() {
        let mut v = larger_example_input();
        assert_eq!(process_int_code_with_input(&mut v, 9), Some(1001));
    }
}
