use crate::opcode::{Number, Op, Register, Values};

type Instruction = (Op, Values);
type InstrPointer = Number;
type InstrPointerBinding = Number;

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
        let mut registers: Register = vec![0; 6];
        while let Some((op, values)) = program.get(pointer) {
            registers[binding] = pointer;
            op.execute(&mut registers, values);
            pointer = registers[binding];
            pointer += 1;
        }

        registers[0]
    }

    pub(crate) fn sum_of_divisors(number: Number) -> Number {
        let mut sum = 0;
        let mut divisor = 1;
        while divisor <= number {
            if number % divisor == 0 {
                sum += divisor;
            }
            divisor += 1;
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

    #[test]
    fn part2() {
        assert_eq!(
            1 + 2 + 5 + 10 + 1_055_143 + 2_110_286 + 5_275_715 + 10_551_430,
            Device::sum_of_divisors(10_551_430)
        );
    }
}
