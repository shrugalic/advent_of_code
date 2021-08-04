type Number = usize;
type Input = Number;
type Output = Number;
type Register = [Number; 4];
type Values = (Input, Input, Output);
type OpCode = Number;
type Instruction = (OpCode, Values);

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

const ALL_OPS: [Op; 16] = [
    Op::AddRegister,
    Op::AddImmediate,
    Op::MultiplyRegister,
    Op::MultiplyImmediate,
    Op::BitwiseAndRegister,
    Op::BitwiseAndImmediate,
    Op::BitwiseOrRegister,
    Op::BitwiseOrImmediate,
    Op::SetRegister,
    Op::SetImmediate,
    Op::GreaterThanImmediateRegister,
    Op::GreaterThanRegisterImmediate,
    Op::GreaterThanRegisterRegister,
    Op::EqualImmediateRegister,
    Op::EqualRegisterImmediate,
    Op::EqualRegisterRegister,
];

fn number_of_samples_matching_3_or_more_opcodes(sample: &[String]) -> usize {
    let samples = vec![sample];

    // TODO properly split input into samples

    samples
        .iter()
        .filter(|sample| number_of_op_codes_matching_sample(sample) > 3)
        .count()
}

fn number_of_op_codes_matching_sample(sample: &[String]) -> usize {
    let (regs_before, instr, regs_after) = parse_sample(sample);

    ALL_OPS
        .iter()
        .filter(|&op| execute(op, regs_before, instr.1) == regs_after)
        .count()
}

fn execute(op: &Op, reg: Register, (a, b, c): Values) -> Register {
    let mut res = reg;
    res[c] = match op {
        Op::AddRegister => reg[a] + reg[b],
        Op::AddImmediate => reg[a] + b,
        Op::MultiplyRegister => reg[a] * reg[b],
        Op::MultiplyImmediate => reg[a] * b,
        Op::BitwiseAndRegister => reg[a] & reg[b],
        Op::BitwiseAndImmediate => reg[a] & b,
        Op::BitwiseOrRegister => reg[a] | reg[b],
        Op::BitwiseOrImmediate => reg[a] | b,
        Op::SetRegister => reg[a],
        Op::SetImmediate => a,
        Op::GreaterThanImmediateRegister => {
            if a > reg[b] {
                1
            } else {
                0
            }
        }
        Op::GreaterThanRegisterImmediate => {
            if reg[a] > b {
                1
            } else {
                0
            }
        }
        Op::GreaterThanRegisterRegister => {
            if reg[a] > reg[b] {
                1
            } else {
                0
            }
        }
        Op::EqualImmediateRegister => {
            if a == reg[b] {
                1
            } else {
                0
            }
        }
        Op::EqualRegisterImmediate => {
            if reg[a] == b {
                1
            } else {
                0
            }
        }
        Op::EqualRegisterRegister => {
            if reg[a] == reg[b] {
                1
            } else {
                0
            }
        }
    };
    res
}

fn parse_sample(sample: &[String]) -> (Register, Instruction, Register) {
    let before = sample[0]
        .trim_start_matches("Before: [")
        .trim_end_matches(']')
        .split(", ")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<Number>>();

    let instr = sample[1]
        .split(char::is_whitespace)
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<Number>>();

    let after = sample[2]
        .trim_start_matches("After:  [")
        .trim_end_matches(']')
        .split(", ")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<Number>>();

    (
        vec_to_reg(before),
        (instr[0], (instr[1], instr[2], instr[3])),
        vec_to_reg(after),
    )
}

fn vec_to_reg(v: Vec<Number>) -> Register {
    [v[0], v[1], v[2], v[3]]
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";

    #[test]
    fn three_op_codes_match_example() {
        assert_eq!(
            3,
            number_of_op_codes_matching_sample(&read_str_to_lines(EXAMPLE))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            0,
            number_of_samples_matching_3_or_more_opcodes(&read_file_to_lines("input/day16.txt"))
        );
    }
}
