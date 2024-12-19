use std::fmt::Display;
use OpCode::*;

const INPUT: &str = include_str!("../../2024/input/day17.txt");

pub(crate) fn part1() -> String {
    solve_part1(INPUT)
}
type Integer = u64;
pub(crate) fn part2() -> Integer {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> String {
    let (mut computer, program) = parse(input);
    computer.execute(&program);
    computer.output_as_string()
}

const WINDOW_SIZE: Integer = 1024;
fn solve_part2(input: &str) -> Integer {
    let (mut computer, program) = parse(input);

    /* Example program 0,3,5,4,3,0:
    0: adv 3 -> a = a / 2.pow(3) = a / 8 = a >> 3 -> cut the last octal digit off a
    2: out 4 -> out (a % 8) -> output the last octal digit of a
    4: jnz 0 -> if a != 0 repeat

    => The solution for the example is 117440, or 0o345300 in octal,
    -> which is basically the program in reverse.
    */

    /* Input program:
     0: bst 4 -> bst a -> b = a % 8; -> b is the last (3-bit) digit of a
     2: bxl 5 -> b1 = b ^ 5
     4: cdv 5 -> cdv b -> c = a / 2.pow(b); -> c = a >> b1 -> a 0 to 7-bit shift
     6: adv 3 -> a1 = a / 2.pow(3) -> a >> 3 -> cut the last 3-bit digit off a
     8: bxl 6 -> b2 = b1 ^ 6
    10: bxc 3 -> bxc -> b3 = b2 ^ c
    12: out 5 -> out (b3 % 8)
    14: jnz 0 -> if a != 0 repeat

    Only the last part of a is relevant to the output. With 0 shift at 4:
    it's only the last 3 bits, and each further shift moves this window of 3 bits
    one bit further to the left. So the relevant window is 10 bits wide.
    */

    // Precalculate the outputs for all possible 10-bit wide windows of a
    // a is the index, the first output is the value at this index
    let mut first_output_by_a: Vec<Integer> = Vec::with_capacity(1024);
    for a in 0..WINDOW_SIZE {
        computer.reset(a);
        computer.execute(&program);
        first_output_by_a.push(*computer.outputs.first().unwrap());
    }
    // Use these precalculated outputs to find the program digits recursively
    find_a(0, &program.0, &first_output_by_a).unwrap()
}

fn find_a(
    curr_a: Integer,
    wanted_outputs: &[Number],
    first_output_by_a: &[Integer],
) -> Option<Integer> {
    if wanted_outputs.is_empty() {
        return Some(curr_a);
    }
    let wanted_output = *wanted_outputs.last().unwrap() as Integer;
    (0..8)
        .map(|next_a| (curr_a << 3) + next_a)
        .filter(|next_a| {
            let actual_output = first_output_by_a[(*next_a & (WINDOW_SIZE - 1)) as usize];
            actual_output == wanted_output
        })
        .find_map(|next_a| {
            find_a(
                next_a,
                &wanted_outputs[..wanted_outputs.len() - 1],
                first_output_by_a,
            )
        })
}

type Number = u8;

#[derive(Debug, Clone)]
struct Program(Vec<Number>);

impl Program {
    fn instruction_at(&self, ip: usize) -> Option<(OpCode, Number)> {
        if ip >= self.0.len() - 1 {
            return None;
        }
        let opcode = OpCode::from(self.0[ip]);
        let operand = self.0[ip + 1];
        Some((opcode, operand))
    }
}

#[derive(Debug)]
enum OpCode {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<Number> for OpCode {
    fn from(s: Number) -> Self {
        match s {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Computer {
    a: Integer,
    b: Integer,
    c: Integer,
    ip: usize,
    outputs: Vec<Integer>,
}

impl Computer {
    fn execute(&mut self, program: &Program) {
        while let Some((opcode, operand)) = program.instruction_at(self.ip) {
            let combo_operand = || match operand {
                0..=3 => operand as Integer,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                7 => unreachable!(),
                _ => unreachable!(),
            };
            let division = |numerator| {
                let exponent = combo_operand();
                let denominator = (2 as Integer).wrapping_pow(exponent as u32);
                numerator / denominator
            };
            self.ip += 2;
            match opcode {
                Adv => {
                    self.a = division(self.a);
                }
                Bxl => {
                    let result = self.b ^ operand as Integer;
                    self.b = result;
                }
                Bst => {
                    let result = combo_operand() % 8;
                    self.b = result;
                }
                Jnz => {
                    if self.a != 0 {
                        if operand % 2 != 0 {
                            // This would switch the meaning of opcodes and operations
                            panic!("odd jnz operand {operand}");
                        }
                        self.ip = operand as usize;
                    }
                }
                Bxc => {
                    let result = self.b ^ self.c;
                    self.b = result;
                }
                Out => {
                    let result = combo_operand() % 8;
                    self.outputs.push(result);
                }
                Bdv => {
                    self.b = division(self.a);
                }
                Cdv => {
                    self.c = division(self.a);
                }
            }
        }
    }
    fn output_as_string(&self) -> String {
        self.outputs
            .iter()
            .map(Integer::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
    fn reset(&mut self, a: Integer) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
        self.outputs = vec![];
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .0
            .windows(2)
            .step_by(2)
            .map(|w| {
                let opcode = OpCode::from(w[0]);
                let operand = w[1];
                format!("{opcode:?} {operand}").to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", str)
    }
}

fn parse(input: &str) -> (Computer, Program) {
    let (registers, program) = input.trim().split_once("\n\n").unwrap();
    let registers: Vec<Integer> = registers
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(_, n)| n.parse().ok())
        .collect();
    let program = program
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse::<Number>().ok())
        .collect();
    let computer = Computer {
        a: registers[0],
        b: registers[1],
        c: registers[2],
        ..Default::default()
    };
    (computer, Program(program))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const EXAMPLE_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_part1_example() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", solve_part1(EXAMPLE_1));
    }

    #[test]
    fn test_part1_small_example_1() {
        let mut computer = Computer {
            c: 9,
            ..Default::default()
        };
        computer.execute(&Program(vec![2, 6]));
        assert_eq!(1, computer.b);
    }

    #[test]
    fn test_part1_small_example_2() {
        let mut computer = Computer {
            a: 10,
            ..Default::default()
        };
        computer.execute(&Program(vec![5, 0, 5, 1, 5, 4]));
        assert_eq!(vec![0, 1, 2], computer.outputs);
    }

    #[test]
    fn test_part1_small_example_3() {
        let mut computer = Computer {
            a: 2024,
            ..Default::default()
        };
        computer.execute(&Program(vec![0, 1, 5, 4, 3, 0]));
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], computer.outputs);
        assert_eq!(0, computer.a);
    }

    #[test]
    fn test_part1_small_example_4() {
        let mut computer = Computer {
            b: 29,
            ..Default::default()
        };
        computer.execute(&Program(vec![1, 7]));
        assert_eq!(26, computer.b);
    }

    #[test]
    fn test_part1_small_example_5() {
        let mut computer = Computer {
            b: 2024,
            c: 43690,
            ..Default::default()
        };
        computer.execute(&Program(vec![4, 0]));
        assert_eq!(44354, computer.b);
    }

    #[test]
    fn test_part1() {
        assert_eq!("6,5,7,4,5,7,3,1,0", solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(117440, solve_part2(EXAMPLE_2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(105_875_099_912_602, solve_part2(INPUT));
    }
}
