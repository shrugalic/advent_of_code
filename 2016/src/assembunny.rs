use std::collections::HashSet;
use Op::*;
use Param::*;

type Register = char;
type Value = isize;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Param {
    Register(Register),
    Value(Value),
}
impl From<&str> for Param {
    fn from(s: &str) -> Self {
        if let Ok(number) = s.parse() {
            Value(number)
        } else {
            Register(s.to_char())
        }
    }
}

#[derive(Debug)]
pub(crate) enum Op {
    Cpy(Param, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Param, Param),
    // The two ops below are for day 23 only. This is the toggle command
    Tgl(Register),     // toggle
    Nop(Param, Param), // no-op used to store the previous ops parameters
    // The op below is for day 25 only
    Out(Param), // Transmit the next clock signal value
}
impl From<String> for Op {
    fn from(s: String) -> Self {
        let p: Vec<_> = s.split_ascii_whitespace().collect();
        match p[0] {
            "cpy" => Cpy(Param::from(p[1]), p[2].to_char()),
            "inc" => Inc(p[1].to_char()),
            "dec" => Dec(p[1].to_char()),
            "jnz" => Jnz(Param::from(p[1]), Param::from(p[2])),
            "tgl" => Tgl(p[1].to_char()),
            "out" => Out(Param::from(p[1])),
            _ => panic!("Invalid op {}", s),
        }
    }
}

trait ToChar {
    fn to_char(&self) -> char;
}
impl ToChar for &str {
    fn to_char(&self) -> char {
        self.chars().next().unwrap()
    }
}

trait ToIndex {
    fn to_idx(&self) -> usize;
}
impl ToIndex for char {
    fn to_idx(&self) -> usize {
        (*self as u8 - b'a') as usize
    }
}

#[derive(Debug)]
pub(crate) struct Computer {
    code: Vec<Op>,
    register: Vec<Value>,
}
impl From<Vec<String>> for Computer {
    fn from(s: Vec<String>) -> Self {
        let code = s.into_iter().map(Op::from).collect();
        let register = vec![0; 4];
        Computer { code, register }
    }
}

impl Computer {
    pub(crate) fn run(&mut self) -> isize {
        let mut instr_ptr = 0;
        let mut prev_output = None;
        let mut visited_states = HashSet::new();
        while let Some(op) = self.code.get(instr_ptr) {
            match op {
                Cpy(i, r) => self.register[r.to_idx()] = self.get_value(i),
                Inc(r) => self.register[r.to_idx()] += 1,
                Dec(r) => self.register[r.to_idx()] -= 1,
                Jnz(i, p) => {
                    if 0 != self.get_value(i) {
                        let offset = self.get_value(p);
                        let ip = instr_ptr as isize + offset;
                        if ip < 0 {
                            // still out of bounds, but valid for a usize
                            instr_ptr = self.code.len();
                        } else {
                            instr_ptr = ip as usize;
                        }
                        continue; // Avoid increasing of instr_ptr below
                    }
                }
                // This is for day 23 only
                Tgl(r) => {
                    let offset = self.register[r.to_idx()];
                    let ip = instr_ptr as isize + offset;
                    if 0 <= ip && (ip as usize) < self.code.len() {
                        let op = self.code.get_mut(ip as usize).unwrap();
                        // println!("old op = {:?}", op);
                        match op {
                            Jnz(i, v) => match v {
                                Register(r) => *op = Cpy(*i, *r),
                                Value(_) => *op = Nop(*i, *v),
                            },
                            Cpy(i, r) => *op = Jnz(*i, Param::Register(*r)),
                            Nop(i, v) => *op = Jnz(*i, *v),
                            Inc(r) => *op = Dec(*r),
                            Dec(r) | Tgl(r) => *op = Inc(*r),
                            // Day 25 "Out" does not need to be handled for the day 23-only "Tgl"
                            Out(_) => {}
                        }
                        // println!("new op = {:?}", op);
                    } // else nothing happens if out of bounds
                }
                Nop(_, _) => {} // Just skip this no-op
                Out(p) => {
                    let curr_output = self.get_value(p);
                    match (curr_output, prev_output) {
                        (0, None) | (0, Some(1)) | (1, Some(0)) => prev_output = Some(curr_output),
                        // Not a sequence of 0, 1, 0, 1, 0, 1, …
                        (_, _) => return -1, // denotes error
                    }
                    // Copy the computer's registers into a set to see if we're in an infinite loop,
                    // and stop if we are
                    if !visited_states.insert(format!("{:?}", self.register)) {
                        return 1; // denotes success
                    }
                }
            }
            instr_ptr += 1;
        }
        self.register['a'.to_idx()]
    }

    fn get_value(&self, p: &Param) -> Value {
        match p {
            Param::Register(r) => self.register[r.to_idx()],
            Param::Value(v) => *v,
        }
    }
    pub(crate) fn set_register(&mut self, r: Register, v: Value) {
        self.register[r.to_idx()] = v;
    }
}
