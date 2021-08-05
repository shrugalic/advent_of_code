use std::collections::{HashMap, HashSet};

type Number = usize;
type Input = Number;
type Output = Number;
type Register = [Number; 4];
type Values = (Input, Input, Output);
type OpCode = Number;
type Instruction = (OpCode, Values);
type Sample = (Register, Instruction, Register);

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

pub(crate) fn number_of_samples_matching_3_or_more_opcodes(input: &[String]) -> usize {
    let (samples, _program) = parse_input(input);

    samples
        .into_iter()
        .filter(|&sample| op_codes_matching_sample(sample).1.len() >= 3)
        .count()
}

pub(crate) fn figure_out_op_code_numbers_and_run_program(input: &[String]) -> usize {
    let (samples, program) = parse_input(input);

    let hints: Vec<(OpCode, HashSet<Op>)> = samples
        .iter()
        .map(|&sample| op_codes_matching_sample(sample))
        .collect();

    let op_by_code: HashMap<OpCode, Op> = work_out_op_code_to_op_mapping(hints);

    // Run program
    let mut regs: Register = [0, 0, 0, 0];
    program.iter().for_each(|(op_code, values)| {
        let op = op_by_code.get(op_code).unwrap();
        regs = execute(regs, op, *values)
    });

    regs[0]
}

fn work_out_op_code_to_op_mapping(mut hints: Vec<(OpCode, HashSet<Op>)>) -> HashMap<OpCode, Op> {
    let mut op_by_code = HashMap::new();
    while let Some((code, op)) = hints
        .iter()
        .find(|(_, set)| set.len() == 1)
        .map(|(code, set)| (*code, set.iter().next().unwrap().clone()))
    {
        // Resolved a code that only matches a single op, so add this pair to the mapping
        op_by_code.insert(code, op.clone());
        // Remove all hints for this code
        while let Some(pos) = hints.iter().position(|(other, _)| other == &code) {
            hints.remove(pos);
        }
        // And remove this op from all sets
        hints.iter_mut().for_each(|(_, ops)| {
            ops.remove(&op);
        });
    }
    op_by_code
}

fn parse_input(input: &[String]) -> (Vec<Sample>, Vec<Instruction>) {
    // samples are divided by single empty lines from each other
    let parts = input.split(|line| line.is_empty());
    // after the samples there are 3 empty lines, so the parts from above
    // will be contain two empty slices between samples and program
    let samples: Vec<Sample> = parts
        .clone()
        .take_while(|slice| !slice.is_empty())
        .map(|sample| parse_sample(sample))
        .collect();
    let instructions: &[String] = parts.skip_while(|slice| !slice.is_empty()).nth(2).unwrap();
    let program = parse_program(instructions);
    (samples, program)
}

fn parse_sample(sample: &[String]) -> (Register, Instruction, Register) {
    let before = sample[0]
        .trim_start_matches("Before: [")
        .trim_end_matches(']')
        .split(", ")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<Number>>();

    let instr = parse_instruction(&sample[1]);

    let after = sample[2]
        .trim_start_matches("After:  [")
        .trim_end_matches(']')
        .split(", ")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<Number>>();

    (vec_to_reg(before), instr, vec_to_reg(after))
}

fn parse_program(instructions: &[String]) -> Vec<Instruction> {
    instructions
        .iter()
        .map(|instr| parse_instruction(instr))
        .collect()
}

fn parse_instruction(instr: &str) -> Instruction {
    let instr = instr
        .split(char::is_whitespace)
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<Number>>();
    (instr[0], (instr[1], instr[2], instr[3]))
}

fn vec_to_reg(v: Vec<Number>) -> Register {
    [v[0], v[1], v[2], v[3]]
}

/// Returns the sample's OpCode together with a set of Ops that match the given sample's before
/// and after registers. One of the Ops must be represented by the OpCode
fn op_codes_matching_sample(sample: Sample) -> (OpCode, HashSet<Op>) {
    let (regs_before, instr, regs_after) = sample;

    (
        instr.0, // the opcode number
        ALL_OPS
            .iter()
            .filter(|&op| execute(regs_before, op, instr.1) == regs_after)
            .cloned()
            .collect(),
    )
}

fn execute(input_regs: Register, op: &Op, (a, b, c): Values) -> Register {
    let mut output_regs = input_regs;
    output_regs[c] = match op {
        Op::AddRegister => input_regs[a] + input_regs[b],
        Op::AddImmediate => input_regs[a] + b,
        Op::MultiplyRegister => input_regs[a] * input_regs[b],
        Op::MultiplyImmediate => input_regs[a] * b,
        Op::BitwiseAndRegister => input_regs[a] & input_regs[b],
        Op::BitwiseAndImmediate => input_regs[a] & b,
        Op::BitwiseOrRegister => input_regs[a] | input_regs[b],
        Op::BitwiseOrImmediate => input_regs[a] | b,
        Op::SetRegister => input_regs[a],
        Op::SetImmediate => a,
        Op::GreaterThanImmediateRegister => {
            if a > input_regs[b] {
                1
            } else {
                0
            }
        }
        Op::GreaterThanRegisterImmediate => {
            if input_regs[a] > b {
                1
            } else {
                0
            }
        }
        Op::GreaterThanRegisterRegister => {
            if input_regs[a] > input_regs[b] {
                1
            } else {
                0
            }
        }
        Op::EqualImmediateRegister => {
            if a == input_regs[b] {
                1
            } else {
                0
            }
        }
        Op::EqualRegisterImmediate => {
            if input_regs[a] == b {
                1
            } else {
                0
            }
        }
        Op::EqualRegisterRegister => {
            if input_regs[a] == input_regs[b] {
                1
            } else {
                0
            }
        }
    };
    output_regs
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
        let ops: HashSet<_> = [Op::MultiplyRegister, Op::AddImmediate, Op::SetImmediate]
            .iter()
            .cloned()
            .collect();
        let sample = parse_sample(&read_str_to_lines(EXAMPLE));
        assert_eq!(ops, op_codes_matching_sample(sample).1);
    }

    #[test]
    fn part1() {
        assert_eq!(
            605,
            number_of_samples_matching_3_or_more_opcodes(&read_file_to_lines("input/day16.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            653,
            figure_out_op_code_numbers_and_run_program(&read_file_to_lines("input/day16.txt"))
        );
    }
}
