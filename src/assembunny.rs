use Input::*;
use Op::*;

type Register = char;
type Value = isize;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Input {
    Register(Register),
    Value(Value),
}
impl From<&str> for Input {
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
    Cpy(Input, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Input, Input),
    // The ops below are for day 23 only. This is the toggle command
    Tgl(Register),     // toggle
    Nop(Input, Input), // no-op used to store the previous ops parameters
}
impl From<String> for Op {
    fn from(s: String) -> Self {
        let p: Vec<_> = s.split_ascii_whitespace().collect();
        match p[0] {
            "cpy" => Cpy(Input::from(p[1]), p[2].to_char()),
            "inc" => Inc(p[1].to_char()),
            "dec" => Dec(p[1].to_char()),
            "jnz" => Jnz(Input::from(p[1]), Input::from(p[2])),
            "tgl" => Tgl(p[1].to_char()),
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
        while let Some(op) = self.code.get(instr_ptr) {
            match op {
                Cpy(i, y) => {
                    self.register[y.to_idx()] = match i {
                        Input::Register(x) => self.register[x.to_idx()],
                        Input::Value(v) => *v,
                    }
                }
                Inc(r) => self.register[r.to_idx()] += 1,
                Dec(r) => self.register[r.to_idx()] -= 1,
                Jnz(i, v) => {
                    if 0 != match i {
                        Input::Register(r) => self.register[r.to_idx()],
                        Input::Value(v) => *v,
                    } {
                        let offset = match v {
                            Input::Register(r) => self.register[r.to_idx()],
                            Input::Value(v) => *v,
                        };
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
                            Cpy(i, r) => *op = Jnz(*i, Input::Register(*r)),
                            Nop(i, v) => *op = Jnz(*i, *v),
                            Inc(r) => *op = Dec(*r),
                            Dec(r) | Tgl(r) => *op = Inc(*r),
                        }
                        // println!("new op = {:?}", op);
                    } // else nothing happens if out of bounds
                }
                Nop(_, _) => {} // Just skip this no-op
            }
            instr_ptr += 1;
        }
        self.register['a'.to_idx()]
    }
    pub(crate) fn set_register(&mut self, r: Register, v: Value) {
        self.register[r.to_idx()] = v;
    }
}
