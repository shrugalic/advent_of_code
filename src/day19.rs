type Number = usize;
type Input = Number;
type Output = Number;
type Register = [Number; 6];
type Values = (Input, Input, Output);
type Instruction = (Op, Values);
type InstrPointer = Number;
type InstrPointerBinding = Number;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Op {
    AddRegister,  // stores into register C the result of adding register A and register B.
    AddImmediate, // stores into register C the result of adding register A and value B.
    MultiplyRegister, // stores into register C the result of multiplying register A and register B.
    MultiplyImmediate, // stores into register C the result of multiplying register A and value B.
    BitwiseAndRegister, // stores into register C the result of the bitwise AND of register A and register B.
    BitwiseAndImmediate, // stores into register C the result of the bitwise AND of register A and value B.
    BitwiseOrRegister, // stores into register C the result of the bitwise OR of register A and register B.
    BitwiseOrImmediate, // stores into register C the result of the bitwise OR of register A and value B.
    SetRegister,        // copies the contents of register A into register C. (Input B is ignored.)
    SetImmediate,       // stores value A into register C. (Input B is ignored.)
    GreaterThanImmediateRegister, // sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    GreaterThanRegisterImmediate, // sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0
    GreaterThanRegisterRegister, // sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    EqualImmediateRegister, // sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    EqualRegisterImmediate, // sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    EqualRegisterRegister, // sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
}

impl<T: AsRef<str>> From<T> for Op {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "addr" => Op::AddRegister,
            "addi" => Op::AddImmediate,
            "mulr" => Op::MultiplyRegister,
            "muli" => Op::MultiplyImmediate,
            "banr" => Op::BitwiseAndRegister,
            "bani" => Op::BitwiseAndImmediate,
            "borr" => Op::BitwiseOrRegister,
            "bori" => Op::BitwiseOrImmediate,
            "setr" => Op::SetRegister,
            "seti" => Op::SetImmediate,
            "gtir" => Op::GreaterThanImmediateRegister,
            "gtri" => Op::GreaterThanRegisterImmediate,
            "gtrr" => Op::GreaterThanRegisterRegister,
            "eqir" => Op::EqualImmediateRegister,
            "eqri" => Op::EqualRegisterImmediate,
            "eqrr" => Op::EqualRegisterRegister,
            op => panic!("Illegal op code {}", op),
        }
    }
}

impl Op {
    fn execute(&self, input: &mut Register, values: &Values) {
        let (a, b, c) = *values;
        input[c] = match self {
            Op::AddRegister => input[a] + input[b],
            Op::AddImmediate => input[a] + b,
            Op::MultiplyRegister => input[a] * input[b],
            Op::MultiplyImmediate => input[a] * b,
            Op::BitwiseAndRegister => input[a] & input[b],
            Op::BitwiseAndImmediate => input[a] & b,
            Op::BitwiseOrRegister => input[a] | input[b],
            Op::BitwiseOrImmediate => input[a] | b,
            Op::SetRegister => input[a],
            Op::SetImmediate => a,
            Op::GreaterThanImmediateRegister => {
                if a > input[b] {
                    1
                } else {
                    0
                }
            }
            Op::GreaterThanRegisterImmediate => {
                if input[a] > b {
                    1
                } else {
                    0
                }
            }
            Op::GreaterThanRegisterRegister => {
                if input[a] > input[b] {
                    1
                } else {
                    0
                }
            }
            Op::EqualImmediateRegister => {
                if a == input[b] {
                    1
                } else {
                    0
                }
            }
            Op::EqualRegisterImmediate => {
                if input[a] == b {
                    1
                } else {
                    0
                }
            }
            Op::EqualRegisterRegister => {
                if input[a] == input[b] {
                    1
                } else {
                    0
                }
            }
        };
    }
}

pub(crate) struct Device;

impl Default for Device {
    fn default() -> Self {
        Device {}
    }
}

impl Device {
    pub(crate) fn run_program(&mut self, input: &[String]) -> Number {
        let binding: InstrPointerBinding = input[0].trim_start_matches("#ip ").parse().unwrap();
        let program = Device::parse_program(&input[1..]);

        let mut pointer: InstrPointer = 0;
        let mut registers: Register = [0; 6];
        while let Some((op, values)) = program.get(pointer) {
            registers[binding] = pointer;
            op.execute(&mut registers, values);
            pointer = registers[binding];
            pointer += 1;
        }

        registers[0]
    }

    pub(crate) fn run_program2(&mut self) -> Number {
        let number = 10_551_430; // 2 * 5 * 1055143
        let mut sum = 0;
        let mut divisor = 1;
        loop {
            let mut factor = 1;
            loop {
                if factor * divisor == number {
                    sum += divisor;
                }
                factor += 1;
                if factor > number {
                    break;
                }
            }
            divisor += 1;
            if divisor > number {
                break;
            }
        }
        sum
    }

    fn parse_program(program: &[String]) -> Vec<Instruction> {
        program
            .iter()
            .filter_map(|line| {
                line.split_once(" ").map(|(op, values)| {
                    let op = Op::from(op);
                    let values = values
                        .split(char::is_whitespace)
                        .filter_map(|s| s.parse().ok())
                        .collect::<Vec<Number>>();
                    (op, (values[0], values[1], values[2]))
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE_PROGRAM: &str = "\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn run_example_program() {
        let program = read_str_to_lines(EXAMPLE_PROGRAM);
        assert_eq!(6, Device::default().run_program(&program));
    }

    #[test]
    fn part1() {
        let program = read_file_to_lines("input/day19.txt");
        assert_eq!(1872, Device::default().run_program(&program));
    }
}
