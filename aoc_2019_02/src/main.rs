use std::ops::Index;

fn main() {
    println!("Part 1 result = {:?}", process_noun_and_verb(12, 2));

    for noun in 0..=99 {
        for verb in 0..=99 {
            if process_noun_and_verb(noun, verb) == 19690720 {
                println!(
                    "Part 2 result: 100 * noun + verb = 100 * {:?} + {:?} = {:?}",
                    noun,
                    verb,
                    100 * noun + verb
                );
            }
        }
    }
}

fn process_noun_and_verb(noun: usize, verb: usize) -> usize {
    let mut input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 19, 5, 23, 1, 23, 6, 27, 2,
        9, 27, 31, 1, 5, 31, 35, 1, 35, 10, 39, 1, 39, 10, 43, 2, 43, 9, 47, 1, 6, 47, 51, 2, 51,
        6, 55, 1, 5, 55, 59, 2, 59, 10, 63, 1, 9, 63, 67, 1, 9, 67, 71, 2, 71, 6, 75, 1, 5, 75, 79,
        1, 5, 79, 83, 1, 9, 83, 87, 2, 87, 10, 91, 2, 10, 91, 95, 1, 95, 9, 99, 2, 99, 9, 103, 2,
        10, 103, 107, 2, 9, 107, 111, 1, 111, 5, 115, 1, 115, 2, 119, 1, 119, 6, 0, 99, 2, 0, 14,
        0,
    ];
    input[1] = noun;
    input[2] = verb;
    process_int_code(input)[0]
}

use Op::*;
fn process_int_code(mut v: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    while i < v.len() {
        // println!("v = {:?}", v);
        let op = v[i].clone();
        match Op::from_code(v[i]) {
            ADD => {
                let res_idx = v[i + 3];
                v[res_idx] = v[v[i + 1]] + v[v[i + 2]];
            }
            MULTIPLY => {
                let res_idx = v[i + 3];
                v[res_idx] = v[v[i + 1]] * v[v[i + 2]];
            }
            STOP => return v,
        }
        i += 4;
    }
    v
}

#[derive(PartialEq, Debug)]
enum Op {
    ADD,
    MULTIPLY,
    STOP,
}

impl Op {
    fn from_code(code: usize) -> Op {
        match code {
            1 => ADD,
            2 => MULTIPLY,
            99 => STOP,
            _ => panic!("Unknown int_code {:?}", code),
        }
    }
    fn to_code(&self) -> usize {
        match self {
            ADD => 1,
            MULTIPLY => 2,
            STOP => 99,
        }
    }
}

const EXIT: u32 = 99;

mod tests {
    use crate::{process_int_code, process_noun_and_verb, Op, Op::*};

    #[test]
    fn op_from_int_code() {
        assert_eq!(ADD, Op::from_code(1));
    }

    #[test]
    fn op_to_int_code() {
        assert_eq!(1, ADD.to_code());
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

    #[test]
    fn part_1() {
        assert_eq!(process_noun_and_verb(12, 2), 3516593);
    }
}
