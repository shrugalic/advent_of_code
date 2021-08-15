use crate::opcode::{Number, Op, Register, Values};

type Instruction = (Op, Values);
type InstrPointer = Number;
type InstrPointerBinding = Number;
type RegisterIndex = usize;

pub(crate) struct Device;

impl Default for Device {
    fn default() -> Self {
        Device {}
    }
}

impl Device {
    pub(crate) fn run_program(&mut self, input: &[String]) -> Number {
        self.halting_value(input, usize::MAX, 0).unwrap()
    }

    /// Return the value of `register[halting_reg]` when the instruction pointer reaches `halting_ip`
    pub(crate) fn halting_value(
        &mut self,
        input: &[String],
        halting_ip: InstrPointer,
        halting_reg: RegisterIndex,
    ) -> Number {
        let (binding, program) = Device::parse_input(&input);

        let mut ip: InstrPointer = 0;
        let mut registers: Register = vec![0; 6];
        while let Some((op, values)) = program.get(ip) {
            registers[binding] = ip;
            op.execute(&mut registers, values);
            ip = registers[binding];
            ip += 1;
            if ip == halting_ip {
                return registers[halting_reg];
            }
        }
        registers[halting_reg]
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
