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
    computer.output_of_running(&program).unwrap()
}

fn solve_part2(_input: &str) -> Integer {
    // TODO
    0
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
    fn output_of_running(&mut self, program: &Program) -> Option<String> {
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
        Some(self.output_to_string())
    }
    fn output_to_string(&self) -> String {
        self.outputs
            .iter()
            .map(Integer::to_string)
            .collect::<Vec<_>>()
            .join(",")
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
        computer.output_of_running(&Program(vec![2, 6]));
        assert_eq!(1, computer.b);
    }

    #[test]
    fn test_part1_small_example_2() {
        let mut computer = Computer {
            a: 10,
            ..Default::default()
        };
        computer.output_of_running(&Program(vec![5, 0, 5, 1, 5, 4]));
        assert_eq!(vec![0, 1, 2], computer.outputs);
    }

    #[test]
    fn test_part1_small_example_3() {
        let mut computer = Computer {
            a: 2024,
            ..Default::default()
        };
        computer.output_of_running(&Program(vec![0, 1, 5, 4, 3, 0]));
        assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], computer.outputs);
        assert_eq!(0, computer.a);
    }

    #[test]
    fn test_part1_small_example_4() {
        let mut computer = Computer {
            b: 29,
            ..Default::default()
        };
        computer.output_of_running(&Program(vec![1, 7]));
        assert_eq!(26, computer.b);
    }

    #[test]
    fn test_part1_small_example_5() {
        let mut computer = Computer {
            b: 2024,
            c: 43690,
            ..Default::default()
        };
        computer.output_of_running(&Program(vec![4, 0]));
        assert_eq!(44354, computer.b);
    }

    #[test]
    fn test_part1() {
        assert_eq!("6,5,7,4,5,7,3,1,0", solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(117440, solve_part2(EXAMPLE_2));
        // program 0,3,5,4,3,0
        println!("{} = 0b{:b} = 0o{:o}", 117440, 117440, 117440);
        // 117440 = 0b11100101011000000 = 0o345300 ->
        // note this octal number is basically the program in reverse
        // 011'100'101'011'000'000
        //                 000'000  -> input  0 for output 0
        //             011'000      -> input 24 for output 3
        //         101'000          -> input 40 for output 5
        //     100'000              -> input 32 for output 4
        // 011'000                  -> input 24 for output 3
        //                          -> input  0 for output 0
        let mut a = 24 as Integer;
        a <<= 3;
        a += 32;
        a <<= 3;
        a += 40;
        a <<= 3;
        a += 24;
        a <<= 3;
        assert_eq!(a, solve_part2(EXAMPLE_2));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, solve_part2(INPUT));
    }
}
