use std::convert::TryInto;
use std::fmt::format;
use std::ops::Index;
use Mode::*;
use Op::*;

const THE_ONE_AND_ONLY_INPUT: i32 = 1;

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

fn process_int_code(mut v: Vec<i32>) -> Vec<i32> {
    let mut idx = 0;
    while idx < v.len() {
        let s = to_5_digit_string_padded_with_leading_zeroes(v[idx]);
        let code = to_num(&s[(s.len() - 2)..s.len()]);
        let op = Op::from_code(code);
        let modes = extract_modes(&s);
        let pre = format!("{:?}: {:?}", s, op);
        match op {
            ADD | MULTIPLY => {
                let p1 = param_value(&v, idx + 1, &modes[0]);
                let p2 = param_value(&v, idx + 2, &modes[1]);
                let res = match op {
                    ADD => p1 + p2,
                    MULTIPLY => p1 * p2,
                    _ => unreachable!(),
                };
                println!("{} ({:?}, {:2}) = {:?}", pre, p1, p2, res);
                // Store result
                let res_idx = v[idx + 3] as usize;
                v[res_idx] = res
            }
            INPUT => {
                let res_idx = v[idx + 1] as usize;
                v[res_idx] = THE_ONE_AND_ONLY_INPUT;
                println!("{} = {:?}", pre, v[idx + 1])
            }
            OUTPUT => {
                let p = param_value(&v, idx + 1, &modes[0]);
                println!("{} = {:?}", pre, p)
            }

            STOP => {
                println!("{}", pre);
                return v;
            }
        }
        idx += op.value_count();
    }
    v
}

#[derive(PartialEq, Debug)]
enum Op {
    ADD,
    MULTIPLY,
    INPUT,
    OUTPUT,
    STOP,
}

impl Op {
    fn from_code(code: i32) -> Op {
        match code {
            1 => ADD,
            2 => MULTIPLY,
            3 => INPUT,
            4 => OUTPUT,
            99 => STOP,
            _ => panic!("Unknown Op code {:?}", code),
        }
    }
    fn value_count(&self) -> usize {
        match self {
            ADD | MULTIPLY => 4,
            INPUT | OUTPUT => 2,
            STOP => 1,
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
    use crate::{process_int_code, Op, Op::*};

    #[test]
    fn op_from_int_code() {
        assert_eq!(ADD, Op::from_code(1));
    }

    #[test]
    fn explanation_example() {
        assert_eq!(
            process_int_code(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            // expected:
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn add_example_1() {
        assert_eq!(
            process_int_code(vec![1, 0, 0, 0, 99]),
            // expected:
            vec![2, 0, 0, 0, 99]
        );
    }

    #[test]
    fn mult_example_1() {
        assert_eq!(
            process_int_code(vec![2, 3, 0, 3, 99]),
            // expected:
            vec![2, 3, 0, 6, 99]
        );
    }

    #[test]
    fn mult_example_2() {
        assert_eq!(
            process_int_code(vec![2, 4, 4, 5, 99, 0]),
            // expected:
            vec![2, 4, 4, 5, 99, 9801]
        );
    }

    #[test]
    fn add_example_2() {
        assert_eq!(
            process_int_code(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            // expected:
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    // new day 5 tests

    #[test]
    fn multiply_example() {
        assert_eq!(
            process_int_code(vec![1002, 4, 3, 4, 33]),
            // expected:
            vec![1002, 4, 3, 4, 99]
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            process_int_code(vec![
                3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1, 192, 154, 224, 101, -161, 224,
                224, 4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1001, 157, 48,
                224, 1001, 224, -61, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223, 224,
                223, 1102, 15, 28, 225, 1002, 162, 75, 224, 1001, 224, -600, 224, 4, 224, 1002,
                223, 8, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 102, 32, 57, 224, 1001, 224,
                -480, 224, 4, 224, 102, 8, 223, 223, 101, 1, 224, 224, 1, 224, 223, 223, 1101, 6,
                23, 225, 1102, 15, 70, 224, 1001, 224, -1050, 224, 4, 224, 1002, 223, 8, 223, 101,
                5, 224, 224, 1, 224, 223, 223, 101, 53, 196, 224, 1001, 224, -63, 224, 4, 224, 102,
                8, 223, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1101, 64, 94, 225, 1102, 13, 23,
                225, 1101, 41, 8, 225, 2, 105, 187, 224, 1001, 224, -60, 224, 4, 224, 1002, 223, 8,
                223, 101, 6, 224, 224, 1, 224, 223, 223, 1101, 10, 23, 225, 1101, 16, 67, 225,
                1101, 58, 10, 225, 1101, 25, 34, 224, 1001, 224, -59, 224, 4, 224, 1002, 223, 8,
                223, 1001, 224, 3, 224, 1, 223, 224, 223, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999,
                1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006,
                0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225,
                225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999,
                1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1108, 226, 226, 224,
                102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223,
                2, 223, 1005, 224, 344, 1001, 223, 1, 223, 107, 677, 226, 224, 102, 2, 223, 223,
                1005, 224, 359, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1005, 224,
                374, 101, 1, 223, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 389, 101,
                1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 404, 101, 1, 223,
                223, 7, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 419, 101, 1, 223, 223, 1107,
                226, 677, 224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1108, 226,
                677, 224, 102, 2, 223, 223, 1005, 224, 449, 101, 1, 223, 223, 108, 226, 677, 224,
                102, 2, 223, 223, 1005, 224, 464, 1001, 223, 1, 223, 8, 226, 677, 224, 1002, 223,
                2, 223, 1005, 224, 479, 1001, 223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223,
                1006, 224, 494, 101, 1, 223, 223, 1008, 226, 677, 224, 102, 2, 223, 223, 1006, 224,
                509, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 524,
                1001, 223, 1, 223, 108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 539, 1001,
                223, 1, 223, 1107, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 554, 1001, 223, 1,
                223, 7, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1, 223, 8,
                677, 226, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1, 223, 223, 1008, 677, 677,
                224, 102, 2, 223, 223, 1005, 224, 599, 101, 1, 223, 223, 1007, 226, 677, 224, 1002,
                223, 2, 223, 1006, 224, 614, 1001, 223, 1, 223, 8, 677, 677, 224, 1002, 223, 2,
                223, 1005, 224, 629, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1005,
                224, 644, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 659,
                101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 674, 1001, 223,
                1, 223, 4, 223, 99, 226
            ]),
            // expected: intentionally wrong to make it show up in the test results
            vec![]
        );
    }
}
