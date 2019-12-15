use permutohedron::heap_recursive;
use rayon::prelude::*;
use std::cmp::min;
use Mode::*;
use Op::*;

const DEFAULT_INPUT: i32 = 1;

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

fn max_thruster_signal(prg: &Vec<i32>) -> i32 {
    all_phase_sequences()
        .par_iter()
        .map(|seq| calc_thrust(prg, seq))
        .max()
        .unwrap()
}

fn all_phase_sequences() -> Vec<Vec<i32>> {
    let mut phases = [0, 1, 2, 3, 4];
    let mut phase_sequences = Vec::new();
    heap_recursive(&mut phases, |seq| phase_sequences.push(seq.to_vec()));
    phase_sequences
}

fn calc_thrust(prg: &Vec<i32>, seq: &Vec<i32>) -> i32 {
    //    println!("Phase sequence {:?}", seq);
    let mut in_out = 0;
    for phase in seq {
        let inputs: Vec<i32> = vec![*phase, in_out];
        let mut v2 = prg.clone();
        if let Some(output) = process_int_code_with_inputs(&mut v2, inputs) {
            in_out = output;
        }
    }
    in_out
}

fn process_int_code_with_input(v: &mut Vec<i32>, input: i32) -> Option<i32> {
    let inputs = vec![input];
    process_int_code_with_inputs(v, inputs)
}

fn process_int_code_with_inputs(v: &mut Vec<i32>, inputs: Vec<i32>) -> Option<i32> {
    let mut idx = 0;
    let mut input_idx = 0;
    let mut output = None;
    while idx < v.len() {
        let s = to_5_digit_string_padded_with_leading_zeroes(v[idx]);
        let code = to_num(&s[(s.len() - 2)..s.len()]);
        let op = Op::from_code(code);
        let modes = extract_modes(&s);
        match op {
            Add | Multiply | LessThan | Equals => {
                let p1 = param_value(&v, idx + 1, &modes[0]);
                let p2 = param_value(&v, idx + 2, &modes[1]);
                let res = match op {
                    Add => p1 + p2,
                    Multiply => p1 * p2,
                    LessThan => eval(p1 < p2),
                    Equals => eval(p1 == p2),
                    _ => unreachable!(),
                };
                // Store result
                let res_idx = v[idx + 3] as usize;
                v[res_idx] = res;
            }
            Input => {
                let res_idx = v[idx + 1] as usize;
                // avoid out-of-bounds by repeatedly taking last input if necessary
                let input = inputs[min(input_idx, inputs.len() - 1)];
                input_idx += 1;
                v[res_idx] = input;
            }
            Output => {
                let p1 = param_value(&v, idx + 1, &modes[0]);
                output = Some(p1);
            }
            JumpIfTrue | JumpIfFalse => {
                let p1 = param_value(&v, idx + 1, &modes[0]);
                let p2 = param_value(&v, idx + 2, &modes[1]);
                if op == JumpIfTrue && p1 != 0 || op == JumpIfFalse && p1 == 0 {
                    idx = p2 as usize;
                    continue; // jump here, rather than increasing idx below
                }
            }
            Stop => {
                break;
            }
        }
        idx += op.value_count();
    }
    output
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
    fn to_code(&self) -> usize {
        match self {
            POSITION => 0,
            IMMEDIATE => 1,
        }
    }
}

mod tests {
    use crate::{max_thruster_signal, process_int_code, process_int_code_with_input, Op, Op::*};

    // day 7

    #[test]
    fn day7_part1_example_1() {
        let mut input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(max_thruster_signal(&mut input), 43210);
    }

    #[test]
    fn day7_part1_example_2() {
        let mut input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(max_thruster_signal(&mut input), 54321);
    }
    #[test]
    fn day7_part1_example_3() {
        let mut input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(max_thruster_signal(&mut input), 65210);
    }

    #[test]
    fn day7_part_1() {
        let mut v = puzzle_input();
        assert_eq!(max_thruster_signal(&mut v), 87138);
    }

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

    // day 2

    #[test]
    fn op_from_int_code() {
        assert_eq!(Add, Op::from_code(1));
    }

    #[test]
    fn explanation_example() {
        let mut v = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(process_int_code(&mut v), None);
        assert_eq!(v, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50])
    }

    #[test]
    fn add_example_1() {
        let mut v = vec![1, 0, 0, 0, 99];
        assert_eq!(process_int_code(&mut v), None);
        assert_eq!(v, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn mult_example_1() {
        let mut v = vec![2, 3, 0, 3, 99];
        assert_eq!(process_int_code(&mut v), None);
        assert_eq!(v, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn mult_example_2() {
        let mut v = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(process_int_code(&mut v), None);
        assert_eq!(v, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn add_example_2() {
        let mut v = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(process_int_code(&mut v), None);
        assert_eq!(v, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    // day 5 part 1

    #[test]
    fn multiply_example() {
        let mut v = vec![1002, 4, 3, 4, 33];
        assert_eq!(process_int_code(&mut v), None);
        assert_eq!(v, vec![1002, 4, 3, 4, 99]);
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
