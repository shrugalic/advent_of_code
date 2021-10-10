use line_reader::read_file_to_lines;
use Input::*;
use Op::*;

pub(crate) fn day12_part1() -> isize {
    let mut computer = Computer::from(read_file_to_lines("input/day12.txt"));
    computer.run()
}

pub(crate) fn day12_part2() -> isize {
    let mut computer = Computer::from(read_file_to_lines("input/day12.txt"));
    computer.register['c'.to_idx()] = 1;
    computer.run()
}

type Register = char;
type Value = isize;

#[derive(Debug)]
enum Input {
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
enum Op {
    Cpy(Input, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Input, Value),
}
impl From<String> for Op {
    fn from(s: String) -> Self {
        let p: Vec<_> = s.split_ascii_whitespace().collect();
        match p[0] {
            "cpy" => Cpy(Input::from(p[1]), p[2].to_char()),
            "inc" => Inc(p[1].to_char()),
            "dec" => Dec(p[1].to_char()),
            "jnz" => Jnz(Input::from(p[1]), p[2].parse().unwrap()),
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
struct Computer {
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
    fn run(&mut self) -> isize {
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
                Jnz(i, diff) => {
                    if 0 != match i {
                        Input::Register(r) => self.register[r.to_idx()],
                        Input::Value(v) => *v,
                    } {
                        let ip = instr_ptr as isize + *diff;
                        if ip < 0 {
                            // still out of bounds, but valid for a usize
                            instr_ptr = self.code.len();
                        } else {
                            instr_ptr = ip as usize;
                        }
                        continue; // Avoid increasing of instr_ptr below
                    }
                }
            }
            instr_ptr += 1;
        }
        self.register['a'.to_idx()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn part1_example() {
        let mut computer = Computer::from(read_str_to_lines(EXAMPLE));
        assert_eq!(42, computer.run());
    }

    #[test]
    fn part1() {
        assert_eq!(318_003, day12_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(9_227_657, day12_part2());
    }
}
