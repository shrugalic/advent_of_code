use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
enum Op {
    Acc,
    Jmp,
    Nop,
}
impl<T> From<T> for Op
where
    T: AsRef<str>,
{
    fn from(op: T) -> Self {
        match op.as_ref() {
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            "nop" => Op::Nop,
            inv_op => panic!("Invalid operation '{}'", inv_op),
        }
    }
}
#[derive(PartialEq, Debug, Clone)]
struct Instr {
    op: Op,
    arg: isize,
}
impl<T> From<T> for Instr
where
    T: AsRef<str>,
{
    fn from(instr: T) -> Self {
        match instr.as_ref().split_once(' ') {
            Some((op, count)) => Instr {
                op: Op::from(op),
                arg: count.parse().expect("number"),
            },
            instr => panic!("Invalid instruction '{:?}'", instr),
        }
    }
}

fn run_program_until_infinite_loop(boot_code: &[String]) -> (isize, bool) {
    let program: Vec<_> = boot_code.iter().map(Instr::from).collect();
    run_program(&program)
}

fn fix_program_until_no_more_infinite_loop(boot_code: &[String]) -> (isize, bool) {
    let program: Vec<_> = boot_code.iter().map(Instr::from).collect();
    for (idx, instr) in program.iter().enumerate() {
        match instr.op {
            Op::Jmp | Op::Nop => {
                let mut adapted_program = program.clone();
                adapted_program[idx].op = if instr.op == Op::Jmp {
                    Op::Nop
                } else {
                    Op::Jmp
                };
                if let (accu, false) = run_program(&adapted_program) {
                    return (accu, false);
                }
            }
            Op::Acc => {}
        }
    }
    (-1, true)
}

fn run_program(program: &Vec<Instr>) -> (isize, bool) {
    let mut visited = HashSet::new();
    let mut accu = 0; // Start with accumulator 0
    let mut next = 0; // Start at instruction 0
    let mut inf_loop = false;
    while let Some(instr) = program.get(next) {
        // println!("[{}]: {:?} {}; accu = {}", next, instr.op, instr.arg, accu);
        next += 1; // Here so the No-op can be a no-op, requires -1 adjustment on jump however!
        match instr.op {
            Op::Acc => accu += instr.arg,
            Op::Jmp => next = (next as isize - 1 + instr.arg) as usize, // -1 because of +1 above
            Op::Nop => {}
        }
        inf_loop = !visited.insert(next);
        if inf_loop {
            break;
        }
    }
    (accu, inf_loop)
}

mod tests {
    use crate::{
        fix_program_until_no_more_infinite_loop, run_program_until_infinite_loop, Instr, Op,
    };
    use line_reader::{read_file_to_lines, read_str_to_lines};

    #[test]
    fn operation_from_text() {
        assert_eq!(Op::from("acc"), Op::Acc);
    }

    #[test]
    fn instruction_from_text() {
        assert_eq!(
            Instr::from("jmp -2"),
            Instr {
                op: Op::Jmp,
                arg: -2
            }
        );
    }

    const EXAMPLE_PROGRAM: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part1_example() {
        assert_eq!(
            run_program_until_infinite_loop(&read_str_to_lines(EXAMPLE_PROGRAM)),
            (5, true)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            run_program_until_infinite_loop(&read_file_to_lines("input.txt")),
            (1810, true)
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            fix_program_until_no_more_infinite_loop(&read_str_to_lines(EXAMPLE_PROGRAM)),
            (8, false)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            fix_program_until_no_more_infinite_loop(&read_file_to_lines("input.txt")),
            (969, false)
        );
    }
}
