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

pub(crate) fn process_int_code_with_input(v: &mut Vec<i32>, input: i32) -> Option<i32> {
    let mut idx = 0;
    let mut output = None;
    while idx < v.len() {
        let s = to_5_digit_string_padded_with_leading_zeroes(v[idx]);
        let code = to_num(&s[(s.len() - 2)..s.len()]);
        let op = Op::from_code(code);
        let modes = extract_modes(&s);
        // let pre = format!("{:?}: {:?}", s, op);
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
                // println!("{} ({}, {}) -> v[{}] = {}", pre, p1, p2, res_idx, res);
                v[res_idx] = res;
            }
            Input => {
                let res_idx = v[idx + 1] as usize;
                // println!("{} -> v[{}] = {}", pre, res_idx, input);
                v[res_idx] = input;
            }
            Output => {
                let p1 = param_value(&v, idx + 1, &modes[0]);
                output = Some(p1);
                // println!("{} = {}", pre, p1);
            }
            JumpIfTrue | JumpIfFalse => {
                let p1 = param_value(&v, idx + 1, &modes[0]);
                let p2 = param_value(&v, idx + 2, &modes[1]);
                if op == JumpIfTrue && p1 != 0 || op == JumpIfFalse && p1 == 0 {
                    idx = p2 as usize;
                    // println!("{} ({}) == true -> jump to {}", pre, p1, p2);
                    continue; // jump, rather than increasing idx below
                }
                // println!("{} ({}) == false -> NO jump (to {})", pre, p1, p2);
            }
            Stop => {
                // println!("{}", pre);
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
    #[allow(unused)]
    fn to_code(&self) -> usize {
        match self {
            POSITION => 0,
            IMMEDIATE => 1,
        }
    }
}

pub(crate) fn day05_puzzle_input() -> Vec<i32> {
    vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1, 192, 154, 224, 101, -161, 224, 224, 4,
        224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1001, 157, 48, 224, 1001, 224,
        -61, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1102, 15, 28, 225,
        1002, 162, 75, 224, 1001, 224, -600, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1,
        224, 223, 223, 102, 32, 57, 224, 1001, 224, -480, 224, 4, 224, 102, 8, 223, 223, 101, 1,
        224, 224, 1, 224, 223, 223, 1101, 6, 23, 225, 1102, 15, 70, 224, 1001, 224, -1050, 224, 4,
        224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223, 223, 101, 53, 196, 224, 1001, 224,
        -63, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1101, 64, 94, 225,
        1102, 13, 23, 225, 1101, 41, 8, 225, 2, 105, 187, 224, 1001, 224, -60, 224, 4, 224, 1002,
        223, 8, 223, 101, 6, 224, 224, 1, 224, 223, 223, 1101, 10, 23, 225, 1101, 16, 67, 225,
        1101, 58, 10, 225, 1101, 25, 34, 224, 1001, 224, -59, 224, 4, 224, 1002, 223, 8, 223, 1001,
        224, 3, 224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1,
        99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274,
        1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0,
        1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0,
        1105, 1, 99999, 1108, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223,
        107, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 107, 677, 226,
        224, 102, 2, 223, 223, 1005, 224, 359, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223,
        223, 1005, 224, 374, 101, 1, 223, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1006, 224,
        389, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 404, 101, 1, 223,
        223, 7, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 419, 101, 1, 223, 223, 1107, 226, 677,
        224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1108, 226, 677, 224, 102, 2,
        223, 223, 1005, 224, 449, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1005,
        224, 464, 1001, 223, 1, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 479, 1001,
        223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 494, 101, 1, 223, 223, 1008,
        226, 677, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 1107, 677, 226, 224,
        1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 108, 677, 677, 224, 1002, 223, 2,
        223, 1005, 224, 539, 1001, 223, 1, 223, 1107, 226, 226, 224, 1002, 223, 2, 223, 1006, 224,
        554, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1,
        223, 8, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1, 223, 223, 1008, 677, 677,
        224, 102, 2, 223, 223, 1005, 224, 599, 101, 1, 223, 223, 1007, 226, 677, 224, 1002, 223, 2,
        223, 1006, 224, 614, 1001, 223, 1, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1005, 224,
        629, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 644, 101, 1, 223,
        223, 1108, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 659, 101, 1, 223, 223, 1008, 226,
        226, 224, 102, 2, 223, 223, 1006, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226,
    ]
}

mod tests {
    use super::*;

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

    #[test]
    fn part_1() {
        let mut v = day05_puzzle_input();
        assert_eq!(process_int_code_with_input(&mut v, 1), Some(11049715));
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

    #[test]
    fn part_2() {
        let mut v = day05_puzzle_input();
        assert_eq!(process_int_code_with_input(&mut v, 5), Some(2140710));
    }
}
