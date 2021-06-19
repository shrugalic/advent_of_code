use std::collections::HashSet;

#[derive(PartialEq, Debug)]
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
#[derive(PartialEq, Debug)]
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

fn run_program(boot_code: &Vec<String>) -> Option<isize> {
    let program: Vec<_> = boot_code.iter().map(Instr::from).collect();
    let mut visited = HashSet::new();
    let mut accu = 0; // Start with accumulator 0
    let mut next = 0; // Start at instruction 0
    while let Some(instr) = program.get(next) {
        // println!("[{}]: {:?} {}; accu = {}", next, instr.op, instr.arg, accu);
        next += 1; // Here so the No-op can be a no-op, requires -1 adjustment on jump however!
        match instr.op {
            Op::Acc => accu += instr.arg,
            Op::Jmp => next = (next as isize - 1 + instr.arg) as usize, // -1 because of +1 above
            Op::Nop => {}
        }
        // Exit on infinite loop
        if visited.contains(&next) {
            return Some(accu);
        } else {
            visited.insert(next);
        }
    }
    // Successful termination
    None
}

mod tests {
    use crate::{run_program, Instr, Op};
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
        assert_eq!(run_program(&read_str_to_lines(EXAMPLE_PROGRAM)), Some(5));
    }

    #[test]
    fn part1() {
        assert_eq!(run_program(&read_file_to_lines("input.txt")), Some(1810));
    }
}
