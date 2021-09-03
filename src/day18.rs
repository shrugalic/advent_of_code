use crate::day18::Instr::*;
use crate::day18::State::*;
use crate::day18::Value::*;
use line_reader::read_file_to_lines;
use std::collections::VecDeque;

pub(crate) fn day18_part1() -> NumberValue {
    value_of_last_recovered_frequency(read_file_to_lines("input/day18.txt"))
}

pub(crate) fn day18_part2() -> usize {
    number_of_times_program_1_sent_a_value(read_file_to_lines("input/day18.txt"))
}

fn value_of_last_recovered_frequency(input: Vec<String>) -> NumberValue {
    let instr = input.iter().map(Instr::from).collect::<Vec<_>>();
    let mut program = Program::new(0, &instr);
    let mut last_played_freq = 0;
    loop {
        match program.step() {
            Running => {}
            SentOutput(out) => last_played_freq = out,
            AwaitingInput | Terminated => {
                return last_played_freq;
            }
        }
    }
}

fn number_of_times_program_1_sent_a_value(input: Vec<String>) -> usize {
    let instr = input.iter().map(Instr::from).collect::<Vec<_>>();
    let mut programs = [Program::new(0, &instr), Program::new(1, &instr)];
    let mut send_count = [0, 0];
    let mut is_blocked = [false, false];
    while !(is_blocked[0] && is_blocked[1]) {
        for id in 0..=1 {
            match programs[id].step() {
                Running => {}
                SentOutput(value) => {
                    send_count[id] += 1;
                    programs[(id + 1) % 2].receive(value);
                }
                AwaitingInput | Terminated => {
                    is_blocked[id] = true;
                }
            }
        }
    }
    send_count[1]
}

enum State {
    Running,
    SentOutput(NumberValue),
    AwaitingInput,
    Terminated,
}
struct Program<'a> {
    registers: Vec<NumberValue>,
    instr: &'a [Instr],
    instr_ptr: usize,
    received: VecDeque<NumberValue>,
}

trait KeyToIdx {
    fn to_idx(&self) -> usize;
}
impl KeyToIdx for char {
    fn to_idx(&self) -> usize {
        (*self as usize) - 97
    }
}

impl<'a> Program<'a> {
    fn new(id: u8, instr: &'a [Instr]) -> Self {
        let mut registers = vec![0; 26];
        registers['p'.to_idx()] = id as isize;
        Program {
            registers,
            instr,
            instr_ptr: 0,
            received: VecDeque::new(),
        }
    }
    fn receive(&mut self, input: NumberValue) {
        self.received.push_back(input);
    }
    fn step(&mut self) -> State {
        if let Some(instr) = self.instr.get(self.instr_ptr) {
            // println!("{}: {:?} {:?}", self.instr_ptr, instr, self.registers);
            match instr {
                Snd(Register(x)) => {
                    self.instr_ptr += 1;
                    return State::SentOutput(self.registers[x.to_idx()]);
                }
                Snd(Number(x)) => {
                    self.instr_ptr += 1;
                    return State::SentOutput(*x);
                }
                Set(x, Register(y)) => {
                    self.registers[x.to_idx()] = self.registers[y.to_idx()];
                }
                Set(x, Number(y)) => {
                    self.registers[x.to_idx()] = *y;
                }
                Add(x, Register(y)) => {
                    self.registers[x.to_idx()] += self.registers[y.to_idx()];
                }
                Add(x, Number(y)) => {
                    self.registers[x.to_idx()] += *y;
                }
                Mul(x, Register(y)) => {
                    self.registers[x.to_idx()] *= self.registers[y.to_idx()];
                }
                Mul(x, Number(y)) => {
                    self.registers[x.to_idx()] *= *y;
                }
                Mod(x, Register(y)) => {
                    self.registers[x.to_idx()] %= self.registers[y.to_idx()];
                }
                Mod(x, Number(y)) => {
                    self.registers[x.to_idx()] %= *y;
                }
                Rcv(x) => {
                    if let Some(received) = self.received.pop_front() {
                        self.registers[x.to_idx()] = received;
                    } else {
                        return AwaitingInput;
                    }
                }
                Jgz(Register(x), Register(y)) => {
                    if self.registers[x.to_idx()] > 0 {
                        self.instr_ptr =
                            (self.instr_ptr as isize + self.registers[y.to_idx()]) as usize;
                        return State::Running;
                    }
                }
                Jgz(Register(x), Number(y)) => {
                    if self.registers[x.to_idx()] > 0 {
                        self.instr_ptr = (self.instr_ptr as isize + y) as usize;
                        return State::Running;
                    }
                }
                Jgz(Number(x), Register(y)) => {
                    if *x > 0 {
                        self.instr_ptr =
                            (self.instr_ptr as isize + self.registers[y.to_idx()]) as usize;
                        return State::Running;
                    }
                }
                Jgz(Number(x), Number(y)) => {
                    if *x > 0 {
                        self.instr_ptr = (self.instr_ptr as isize + *y) as usize;
                        return State::Running;
                    }
                }
            }
            self.instr_ptr += 1;
            State::Running
        } else {
            State::Terminated
        }
    }
}

type NumberValue = isize;
type RegisterName = char;

#[derive(Debug)]
enum Value {
    Register(RegisterName),
    Number(NumberValue),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        if let Ok(number) = value.parse() {
            Value::Number(number)
        } else {
            Value::Register(value.chars().next().unwrap())
        }
    }
}

#[derive(Debug)]
enum Instr {
    Snd(Value),
    Set(RegisterName, Value),
    Add(RegisterName, Value),
    Mul(RegisterName, Value),
    Mod(RegisterName, Value),
    Rcv(RegisterName),
    Jgz(Value, Value),
}

impl From<&String> for Instr {
    fn from(instr: &String) -> Self {
        let s: Vec<&str> = instr.split_ascii_whitespace().collect();
        let r = s[1].chars().next().unwrap();
        match s[0] {
            "snd" => Instr::Snd(Value::from(s[1])),
            "set" => Instr::Set(r, Value::from(s[2])),
            "add" => Instr::Add(r, Value::from(s[2])),
            "mul" => Instr::Mul(r, Value::from(s[2])),
            "mod" => Instr::Mod(r, Value::from(s[2])),
            "rcv" => Instr::Rcv(r),
            "jgz" => Instr::Jgz(Value::from(s[1]), Value::from(s[2])),
            _ => panic!("Invalid instruction '{}'", instr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE_1: &str = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test]
    fn part1_example() {
        assert_eq!(
            4,
            value_of_last_recovered_frequency(read_str_to_lines(EXAMPLE_1))
        );
    }
    #[test]
    fn part1() {
        assert_eq!(2951, day18_part1());
    }

    const EXAMPLE_2: &str = "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    #[test]
    fn part2_example() {
        assert_eq!(
            3,
            number_of_times_program_1_sent_a_value(read_str_to_lines(EXAMPLE_2))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(7366, day18_part2());
    }
}
