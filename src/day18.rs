use crate::program::Instr;
use crate::program::NumberValue;
use crate::program::Program;
use crate::program::State::*;
use line_reader::read_file_to_lines;

pub(crate) fn day18_part1() -> NumberValue {
    value_of_last_recovered_frequency(read_file_to_lines("input/day18.txt"))
}

pub(crate) fn day18_part2() -> usize {
    number_of_times_program_1_sent_a_value(read_file_to_lines("input/day18.txt"))
}

fn value_of_last_recovered_frequency(input: Vec<String>) -> NumberValue {
    let instr = input.iter().map(Instr::from).collect::<Vec<_>>();
    let mut program = Program::new(0, &instr);
    let mut last_played_freq = 0;
    loop {
        match program.step() {
            Running => {}
            SentOutput(out) => last_played_freq = out,
            AwaitingInput | Terminated => {
                return last_played_freq;
            }
        }
    }
}

fn number_of_times_program_1_sent_a_value(input: Vec<String>) -> usize {
    let instr = input.iter().map(Instr::from).collect::<Vec<_>>();
    let mut programs = [Program::new(0, &instr), Program::new(1, &instr)];
    let mut send_count = [0, 0];
    let mut is_blocked = [false, false];
    while !(is_blocked[0] && is_blocked[1]) {
        for id in 0..=1 {
            match programs[id].step() {
                Running => {}
                SentOutput(value) => {
                    send_count[id] += 1;
                    programs[(id + 1) % 2].receive(value);
                }
                AwaitingInput | Terminated => {
                    is_blocked[id] = true;
                }
            }
        }
    }
    send_count[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE_1: &str = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test]
    fn part1_example() {
        assert_eq!(
            4,
            value_of_last_recovered_frequency(read_str_to_lines(EXAMPLE_1))
        );
    }
    #[test]
    fn part1() {
        assert_eq!(2951, day18_part1());
    }

    const EXAMPLE_2: &str = "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    #[test]
    fn part2_example() {
        assert_eq!(
            3,
            number_of_times_program_1_sent_a_value(read_str_to_lines(EXAMPLE_2))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(7366, day18_part2());
    }
}
