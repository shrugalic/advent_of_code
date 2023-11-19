use crate::opcode::{Number, Op, Register, Values, ALL_OPS};
use crate::parse;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day16.txt");

pub(crate) fn day16_part1() -> usize {
    number_of_samples_matching_3_or_more_opcodes(&parse(INPUT))
}

pub(crate) fn day16_part2() -> usize {
    figure_out_op_code_numbers_and_run_program(&parse(INPUT))
}

type OpCode = Number;
type Instruction = (OpCode, Values);
type Sample = (Register, Instruction, Register);

pub(crate) fn number_of_samples_matching_3_or_more_opcodes(input: &[&str]) -> usize {
    let (samples, _program) = parse_input(input);

    samples
        .into_iter()
        .filter(|sample| op_codes_matching_sample(sample.clone()).1.len() >= 3)
        .count()
}

pub(crate) fn figure_out_op_code_numbers_and_run_program(input: &[&str]) -> usize {
    let (samples, program) = parse_input(input);

    let hints: Vec<(OpCode, HashSet<Op>)> =
        samples.into_iter().map(op_codes_matching_sample).collect();

    let op_by_code: HashMap<OpCode, Op> = work_out_op_code_to_op_mapping(hints);

    // Run program
    let mut regs: Register = vec![0; 4];
    program.iter().for_each(|(op_code, values)| {
        let op = op_by_code.get(op_code).unwrap();
        op.execute(&mut regs, values)
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

fn parse_input(input: &[&str]) -> (Vec<Sample>, Vec<Instruction>) {
    // samples are divided by single empty lines from each other
    let parts = input.split(|line| line.is_empty());
    // after the samples there are 3 empty lines, so the parts from above
    // will be contain two empty slices between samples and program
    let samples: Vec<Sample> = parts
        .clone()
        .take_while(|slice| !slice.is_empty())
        .map(|sample| parse_sample(sample))
        .collect();
    let instructions: &[&str] = parts.skip_while(|slice| !slice.is_empty()).nth(2).unwrap();
    let program = parse_program(instructions);
    (samples, program)
}

fn parse_sample(sample: &[&str]) -> (Register, Instruction, Register) {
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

    (before, instr, after)
}

fn parse_program(instructions: &[&str]) -> Vec<Instruction> {
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

/// Returns the sample's OpCode together with a set of Ops that match the given sample's before
/// and after registers. One of the Ops must be represented by the OpCode
fn op_codes_matching_sample(sample: Sample) -> (OpCode, HashSet<Op>) {
    let (input, instr, output) = sample;
    (
        instr.0, // the opcode number
        ALL_OPS
            .iter()
            .filter(|&op| {
                let mut result: Register = input.clone();
                op.execute(&mut result, &instr.1);
                result == output
            })
            .cloned()
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

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
        let sample = parse_sample(&parse(EXAMPLE));
        assert_eq!(ops, op_codes_matching_sample(sample).1);
    }

    #[test]
    fn part1() {
        assert_eq!(605, day16_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(653, day16_part2());
    }
}
