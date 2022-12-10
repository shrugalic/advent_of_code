use crate::parse;
use Instruction::*;

const INPUT: &str = include_str!("../input/day23.txt");

pub(crate) fn day23_part1() -> usize {
    let mut computer = computer_from_input();
    computer.run();
    computer.registers['b'.to_idx()]
}

pub(crate) fn day23_part2() -> usize {
    let mut computer = computer_from_input();
    computer.registers['a'.to_idx()] = 1;
    computer.run();
    computer.registers['b'.to_idx()]
}

fn computer_from_input() -> Computer {
    let input = parse(INPUT);
    let instructions = parse_instructions(input);
    Computer::new(instructions)
}

fn parse_instructions(input: Vec<&str>) -> Vec<Instruction> {
    input.into_iter().map(Instruction::from).collect()
}

type Register = char;
type Offset = isize;
type Index = usize;
#[derive(Debug, PartialEq)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Inc(Register),
    JumpByOffset(Offset),
    JumpByOffsetIfEven(Register, Offset),
    JumpByOffsetIfOne(Register, Offset),
}
impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let parts: Vec<_> = s.split(|c| c == ' ' || c == ',').collect();
        match parts[0] {
            "hlf" => Half(parts[1].parse().unwrap()),
            "tpl" => Triple(parts[1].parse().unwrap()),
            "inc" => Inc(parts[1].parse().unwrap()),
            "jmp" => JumpByOffset(parts[1].parse().unwrap()),
            "jie" => JumpByOffsetIfEven(parts[1].parse().unwrap(), parts[3].parse().unwrap()),
            "jio" => JumpByOffsetIfOne(parts[1].parse().unwrap(), parts[3].parse().unwrap()),
            _ => panic!("Invalid instruction '{}'", s),
        }
    }
}

#[derive(Debug)]
struct Computer {
    registers: Vec<usize>,
    instructions: Vec<Instruction>,
    instr_ptr: usize,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Computer {
            registers: vec![0, 0],
            instructions,
            instr_ptr: 0,
        }
    }
    fn run(&mut self) {
        while let Some(instr) = self.instructions.get(self.instr_ptr) {
            // println!("{:?} @ {}: {:?}", instr, self.instr_ptr, self.registers);
            match instr {
                Half(reg) => self.registers[reg.to_idx()] /= 2,
                Triple(reg) => self.registers[reg.to_idx()] *= 3,
                Inc(reg) => self.registers[reg.to_idx()] += 1,
                JumpByOffset(offset) => {
                    self.instr_ptr = self.jumped_by(offset);
                    continue;
                }
                JumpByOffsetIfEven(reg, offset) => {
                    if self.registers[reg.to_idx()] % 2 == 0 {
                        self.instr_ptr = self.jumped_by(offset);
                        continue;
                    }
                }
                JumpByOffsetIfOne(reg, offset) => {
                    if self.registers[reg.to_idx()] == 1 {
                        self.instr_ptr = self.jumped_by(offset);
                        continue;
                    }
                }
            }
            self.instr_ptr += 1;
        }
    }
    fn jumped_by(&self, offset: &Offset) -> usize {
        let next = self.instr_ptr as isize + *offset;
        if next.is_negative() {
            // This is also out of range, and will stop the program, but valid for a usize
            self.instructions.len()
        } else {
            next as usize
        }
    }
}

trait ToIndex {
    fn to_idx(&self) -> Index;
}
impl ToIndex for Register {
    fn to_idx(&self) -> Index {
        (*self as u8 - b'a') as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "\
inc a
jio a, +2
tpl a
inc a";

    #[test]
    fn parse_part1_example() {
        let instructions = parse_instructions(parse(EXAMPLE));
        assert_eq!(
            vec![Inc('a'), JumpByOffsetIfOne('a', 2), Triple('a'), Inc('a')],
            instructions
        );
    }

    #[test]
    fn register_to_index() {
        assert_eq!(0, 'a'.to_idx());
        assert_eq!(25, 'z'.to_idx());
    }

    #[test]
    fn part1_example1() {
        let input = parse(EXAMPLE);
        let instructions = parse_instructions(input);
        let mut computer = Computer::new(instructions);
        computer.run();
        assert_eq!(2, computer.registers['a'.to_idx()]);
    }

    #[test]
    fn part1() {
        assert_eq!(255, day23_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(334, day23_part2());
    }
}
