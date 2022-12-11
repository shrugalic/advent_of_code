use intcode::IntCodeComputer;
use crate::parse;

const INPUT: &str = include_str!("../input/day21.txt");

pub(crate) fn day21_part1() -> usize {
    run_springscript(WALK_AND_JUMP_OVER_ANY_HOLE)
}

pub(crate) fn day21_part2() -> usize {
    run_springscript(RUN_AND_JUMP_OVER_ANY_HOLE)
}

// If there's any hole that can be jumped over with a safe landing spot, do it:
// (not A or not B or not C) and D -> jump
const WALK_AND_JUMP_OVER_ANY_HOLE: &str = "\
NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
WALK
";

// Same as above, but only if E or H are OK as well:
// - if H is OK it can be jumped to from D
// - if E is OK one can jump from E directly to I, and not worry about H
// (not (A or B or C)) and D and (E or H) -> jump
const RUN_AND_JUMP_OVER_ANY_HOLE: &str = "\
NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
NOT H T
NOT T T
OR E T
AND T J
RUN
";

fn run_springscript(program: &str) -> usize {
    let mut icc = intcode_computer_from_puzzle_input();
    let inputs: Vec<_> = program.chars().map(|c| c as u8 as isize).collect();
    icc.add_inputs(&inputs);
    icc.run_until_halted();

    let last = *icc.outputs().iter().last().unwrap() as usize;
    if last > 127 {
        last
    } else {
        println!(
            "Failed with err_msg:\n{}",
            icc.outputs()
                .iter()
                .map(|c| *c as u8 as char)
                .collect::<String>()
        );
        0
    }
}

fn intcode_computer_from_puzzle_input() -> IntCodeComputer {
    let input = parse(INPUT);
    let instr = input[0].split(',').map(|n| n.parse().unwrap()).collect();
    intcode::IntCodeComputer::new(instr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        assert_eq!(1142530574, run_springscript(RUN_AND_JUMP_OVER_ANY_HOLE));
    }

    #[test]
    fn part1() {
        assert_eq!(19355364, day21_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1142530574, day21_part2());
    }
}
