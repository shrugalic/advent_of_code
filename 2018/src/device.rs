use crate::opcode::{Number, Op, Register, Values};

type Instruction = (Op, Values);
type InstrPointerBinding = Number;
pub(crate) type InstrPointer = Number;
pub(crate) type RegisterIndex = usize;

pub(crate) struct Device;

impl Default for Device {
    fn default() -> Self {
        Device {}
    }
}

const GET_REGISTER_0_WHEN_IT_HALTS_NATURALLY: InstrPointer = usize::MAX;

impl Device {
    pub(crate) fn run_program(&mut self, input: &[String]) -> Number {
        *self
            .halting_values(input, GET_REGISTER_0_WHEN_IT_HALTS_NATURALLY, 0)
            .first()
            .unwrap()
    }

    /// Return the value of `register[halting_reg]` when the instruction pointer reaches `halting_ip`
    pub(crate) fn halting_values(
        &mut self,
        input: &[String],
        halting_ip: InstrPointer,
        halting_reg: RegisterIndex,
    ) -> Vec<Number> {
        let (binding, program) = Device::parse_input(&input);
        let mut halting_values = vec![];

        let mut ip: InstrPointer = 0;
        let mut registers: Register = vec![0; 6];
        while let Some((op, values)) = program.get(ip) {
            registers[binding] = ip;
            op.execute(&mut registers, values);
            ip = registers[binding];
            ip += 1;
            if ip == halting_ip {
                halting_values.push(registers[halting_reg]);
                break;
            }
        }
        if halting_ip == GET_REGISTER_0_WHEN_IT_HALTS_NATURALLY {
            halting_values.push(registers[0]);
        }
        halting_values
    }

    fn parse_input(input: &&[String]) -> (usize, Vec<(Op, (usize, usize, usize))>) {
        let binding: InstrPointerBinding = input[0].trim_start_matches("#ip ").parse().unwrap();
        let program = Device::parse_program(&input[1..]);
        (binding, program)
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
