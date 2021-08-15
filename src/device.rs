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
        self.run_program_with_given_reg0(0, input).0
    }

    pub(crate) fn run_program_with_given_reg0(
        &mut self,
        reg0: Number,
        input: &[String],
    ) -> (Number, usize) {
        let binding: InstrPointerBinding = input[0].trim_start_matches("#ip ").parse().unwrap();
        let program = Device::parse_program(&input[1..]);

        let mut instr_counter = 0;
        let mut pointer: InstrPointer = 0;
        let mut registers: Register = vec![0; 6];
        registers[0] = reg0;
        while let Some((op, values)) = program.get(pointer) {
            registers[binding] = pointer;
            op.execute(&mut registers, values);
            pointer = registers[binding];
            pointer += 1;
            instr_counter += 1;
        }

        (registers[0], instr_counter)
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
