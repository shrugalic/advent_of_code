use crate::program::Instr;
use crate::program::Program;
use crate::program::State::SentOutput;
use crate::program::State::*;
use crate::parse;

const INPUT: &str = include_str!("../input/day23.txt");

pub(crate) fn day23_part1() -> usize {
    count_mul_instructions(parse(INPUT))
}

pub(crate) fn day23_part2() -> usize {
    value_in_reg_h()
}

fn count_mul_instructions(input: Vec<&str>) -> usize {
    let instr = input.into_iter().map(Instr::from).collect::<Vec<_>>();
    let mut program = Program::new(0, &instr);
    let mut mul_count = 0;
    loop {
        let instr = program.instruction();
        if matches!(instr, Some(Instr::Mul(_, _))) {
            mul_count += 1;
        }
        match program.step() {
            Running => {}
            SentOutput(_) => (),
            AwaitingInput | Terminated => {
                return mul_count;
            }
        }
    }
}

fn value_in_reg_h() -> usize {
    let mut b = 93 * 100 + 100000;
    let c = b + 17000;
    let mut h = 0;
    while b <= c {
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
        b += 17;
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(8281, day23_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(911, day23_part2());
    }
}
