use crate::program::Instr::*;
use crate::program::State::*;
use crate::program::Value::*;
use std::collections::VecDeque;

pub(crate) type NumberValue = isize;
type RegisterName = char;

pub(crate) enum State {
    Running,
    SentOutput(NumberValue),
    AwaitingInput,
    Terminated,
}
pub(crate) struct Program<'a> {
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
    pub(crate) fn new(id: u8, instr: &'a [Instr]) -> Self {
        let mut registers = vec![0; 26];
        registers['p'.to_idx()] = id as isize;
        Program {
            registers,
            instr,
            instr_ptr: 0,
            received: VecDeque::new(),
        }
    }

    pub(crate) fn receive(&mut self, input: NumberValue) {
        self.received.push_back(input);
    }

    pub(crate) fn step(&mut self) -> State {
        self.execute(self.instruction())
    }

    pub(crate) fn instruction(&self) -> Option<Instr> {
        self.instr.get(self.instr_ptr).cloned()
    }

    pub(crate) fn execute(&mut self, instr: Option<Instr>) -> State {
        if let Some(instr) = instr {
            // println!("{}: {:?} {:?}", self.instr_ptr, instr, self.registers);
            match instr {
                Snd(Register(x)) => {
                    self.instr_ptr += 1;
                    return SentOutput(self.registers[x.to_idx()]);
                }
                Snd(Number(x)) => {
                    self.instr_ptr += 1;
                    return SentOutput(x);
                }
                Set(x, Register(y)) => {
                    self.registers[x.to_idx()] = self.registers[y.to_idx()];
                }
                Set(x, Number(y)) => {
                    self.registers[x.to_idx()] = y;
                }
                Add(x, Register(y)) => {
                    self.registers[x.to_idx()] += self.registers[y.to_idx()];
                }
                Add(x, Number(y)) => {
                    self.registers[x.to_idx()] += y;
                }
                Sub(x, Register(y)) => {
                    self.registers[x.to_idx()] -= self.registers[y.to_idx()];
                }
                Sub(x, Number(y)) => {
                    self.registers[x.to_idx()] -= y;
                }
                Mul(x, Register(y)) => {
                    self.registers[x.to_idx()] *= self.registers[y.to_idx()];
                }
                Mul(x, Number(y)) => {
                    self.registers[x.to_idx()] *= y;
                }
                Mod(x, Register(y)) => {
                    self.registers[x.to_idx()] %= self.registers[y.to_idx()];
                }
                Mod(x, Number(y)) => {
                    self.registers[x.to_idx()] %= y;
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
                        return Running;
                    }
                }
                Jgz(Register(x), Number(y)) => {
                    if self.registers[x.to_idx()] > 0 {
                        self.instr_ptr = (self.instr_ptr as isize + y) as usize;
                        return Running;
                    }
                }
                Jgz(Number(x), Register(y)) => {
                    if x > 0 {
                        self.instr_ptr =
                            (self.instr_ptr as isize + self.registers[y.to_idx()]) as usize;
                        return Running;
                    }
                }
                Jgz(Number(x), Number(y)) => {
                    if x > 0 {
                        self.instr_ptr = (self.instr_ptr as isize + y) as usize;
                        return Running;
                    }
                }
                Jnz(Register(x), Register(y)) => {
                    if self.registers[x.to_idx()] != 0 {
                        self.instr_ptr =
                            (self.instr_ptr as isize + self.registers[y.to_idx()]) as usize;
                        return Running;
                    }
                }
                Jnz(Register(x), Number(y)) => {
                    if self.registers[x.to_idx()] != 0 {
                        self.instr_ptr = (self.instr_ptr as isize + y) as usize;
                        return Running;
                    }
                }
                Jnz(Number(x), Register(y)) => {
                    if x != 0 {
                        self.instr_ptr =
                            (self.instr_ptr as isize + self.registers[y.to_idx()]) as usize;
                        return Running;
                    }
                }
                Jnz(Number(x), Number(y)) => {
                    if x != 0 {
                        self.instr_ptr = (self.instr_ptr as isize + y) as usize;
                        return Running;
                    }
                }
            }
            self.instr_ptr += 1;
            Running
        } else {
            Terminated
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Value {
    Register(RegisterName),
    Number(NumberValue),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        if let Ok(number) = value.parse() {
            Number(number)
        } else {
            Register(value.chars().next().unwrap())
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Instr {
    Snd(Value),
    Set(RegisterName, Value),
    Add(RegisterName, Value),
    Sub(RegisterName, Value),
    Mul(RegisterName, Value),
    Mod(RegisterName, Value),
    Rcv(RegisterName),
    Jgz(Value, Value),
    Jnz(Value, Value),
}

impl From<&str> for Instr {
    fn from(instr: &str) -> Self {
        let s: Vec<&str> = instr.split_ascii_whitespace().collect();
        let r = s[1].chars().next().unwrap();
        match s[0] {
            "snd" => Snd(Value::from(s[1])),
            "set" => Set(r, Value::from(s[2])),
            "add" => Add(r, Value::from(s[2])),
            "sub" => Sub(r, Value::from(s[2])),
            "mul" => Mul(r, Value::from(s[2])),
            "mod" => Mod(r, Value::from(s[2])),
            "rcv" => Rcv(r),
            "jgz" => Jgz(Value::from(s[1]), Value::from(s[2])),
            "jnz" => Jnz(Value::from(s[1]), Value::from(s[2])),
            _ => panic!("Invalid instruction '{}'", instr),
        }
    }
}
