use permutohedron::heap_recursive;
use rayon::prelude::*;
use Mode::*;
use Op::*;

const DEFAULT_INPUT: isize = 1;
const PRINT_OPS: bool = false;

pub(crate) fn day7_part1() -> isize {
    max_thrust_in_serial_mode(day07_puzzle_input())
}

pub(crate) fn day7_part2() -> isize {
    max_thrust_in_feedback_loop_mode(day07_puzzle_input())
}

fn to_5_digit_string_padded_with_leading_zeroes(n: isize) -> String {
    let s = n.to_string();
    "0".repeat(5 - s.len()) + s.as_ref()
}

fn to_num(s: &str) -> isize {
    s.parse::<isize>().unwrap()
}

fn extract_modes(s: &str) -> Vec<Mode> {
    vec![
        Mode::from_code(to_num(&s[2..=2])),
        Mode::from_code(to_num(&s[1..=1])),
        Mode::from_code(to_num(&s[0..=0])),
    ]
}

fn param_value(v: &[isize], idx: usize, mode: &Mode) -> isize {
    match mode {
        Position => v[v[idx] as usize],
        Immediate => v[idx],
    }
}

#[allow(unused)]
fn process_int_code(v: Vec<isize>) -> Option<isize> {
    process_int_code_with_input(v, DEFAULT_INPUT)
}

fn eval(b: bool) -> isize {
    if b {
        1
    } else {
        0
    }
}

fn max_thrust_in_serial_mode(prg: Vec<isize>) -> isize {
    let phases = [0, 1, 2, 3, 4];
    permutations_of(phases)
        .iter()
        .map(|seq| calc_thrust(&prg, seq, true))
        .max()
        .unwrap()
}

fn max_thrust_in_feedback_loop_mode(prg: Vec<isize>) -> isize {
    let phases = [5, 6, 7, 8, 9];
    permutations_of(phases)
        .par_iter()
        .map(|seq| calc_thrust(&prg, seq, false))
        .max()
        .unwrap()
}

fn permutations_of(mut phases: [isize; 5]) -> Vec<Vec<isize>> {
    let mut phase_sequences = Vec::new();
    heap_recursive(&mut phases, |seq| phase_sequences.push(seq.to_vec()));
    phase_sequences
}

#[derive(Debug)]
struct Amplifier {
    instructions: Vec<isize>, //instructions
    ptr: usize,               // instruction pointer
}
impl Amplifier {
    fn process(&mut self, mut inputs: Vec<isize>) -> Option<isize> {
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
                    let input = inputs.remove(0);
                    if PRINT_OPS {
                        println!("{} -> v[{}] = {}", pre, res_idx, input);
                    }
                    self.instructions[res_idx] = input;
                }
                Output => {
                    let out = param_value(&self.instructions, self.ptr + 1, &modes[0]);
                    self.ptr += op.value_count();
                    if PRINT_OPS {
                        println!("{} = {}", pre, out);
                    }
                    return Some(out);
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
        None
    }

    fn incr_ptr(&mut self, op: Op) {
        self.ptr += op.value_count();
    }
}

fn calc_thrust(prg: &[isize], seq: &[isize], single_run: bool) -> isize {
    let mut signal = 0;

    // Init & run through each amplifier once
    let mut amplifiers: Vec<Amplifier> = vec![];
    for &phase in seq {
        let mut amp = Amplifier {
            instructions: prg.to_vec(),
            ptr: 0,
        };
        let inputs: Vec<isize> = vec![phase, signal];
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
        for amplifier in amplifiers.iter_mut() {
            let inputs: Vec<isize> = vec![signal];
            if let Some(output) = amplifier.process(inputs) {
                signal = output;
            } else {
                return signal;
            }
        }
    }
}

fn process_int_code_with_input(instructions: Vec<isize>, input: isize) -> Option<isize> {
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
    fn from_code(code: isize) -> Op {
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
    Position,
    Immediate,
}

impl Mode {
    fn from_code(code: isize) -> Mode {
        match code {
            0 => Position,
            1 => Immediate,
            _ => panic!("Unknown Mode code {:?}", code),
        }
    }
}

fn day07_puzzle_input() -> Vec<isize> {
    vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 67, 76, 101, 118, 199, 280, 361, 442, 99999, 3,
        9, 1002, 9, 4, 9, 1001, 9, 2, 9, 102, 3, 9, 9, 101, 3, 9, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9,
        1001, 9, 3, 9, 102, 2, 9, 9, 1001, 9, 2, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 101, 3, 9, 9, 4,
        9, 99, 3, 9, 1001, 9, 2, 9, 1002, 9, 5, 9, 101, 5, 9, 9, 1002, 9, 4, 9, 101, 5, 9, 9, 4, 9,
        99, 3, 9, 102, 2, 9, 9, 1001, 9, 5, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102,
        2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9,
        9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102,
        2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4,
        9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // day 7 part 1

    #[test]
    fn day7_part1_example_1() {
        let input = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(max_thrust_in_serial_mode(input), 43210);
    }

    #[test]
    fn day7_part1_example_2() {
        let input = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(max_thrust_in_serial_mode(input), 54321);
    }
    #[test]
    fn day7_part1_example_3() {
        let input = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(max_thrust_in_serial_mode(input), 65210);
    }

    #[test]
    fn day7_part_1() {
        assert_eq!(day7_part1(), 87138);
    }

    // day 7 part 2

    #[test]
    fn day7_part2_example_1() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(max_thrust_in_feedback_loop_mode(input), 139629729);
    }
    #[test]
    fn day7_part2_example_2() {
        let input = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(max_thrust_in_feedback_loop_mode(input), 18216);
    }

    #[test]
    fn day7_part_2() {
        assert_eq!(day7_part2(), 17279674);
    }
}
