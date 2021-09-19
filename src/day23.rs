use intcode::{IntCodeComputer, State};
use line_reader::read_file_to_lines;

const DEFAULT_INPUT: isize = -1;
pub(crate) fn day23_part1() -> isize {
    run_computers(true)
}

pub(crate) fn day23_part2() -> isize {
    run_computers(false)
}

fn run_computers(stop_on_first_packet_to_address_255: bool) -> isize {
    let mut computers = initialize_computers();
    let mut nat = [DEFAULT_INPUT, DEFAULT_INPUT];
    let mut idle_states = vec![false; 50];
    let mut last_y_sent = None;
    loop {
        for i in 0..50 {
            let mut outputs = vec![];
            while !computers[i].is_halted() {
                match computers[i].step() {
                    State::ExpectingInput => {
                        idle_states[i] = true;
                        computers[i].add_input(DEFAULT_INPUT);
                        break; // Waiting for input, let's give others a chance
                    }
                    State::WroteOutput(out) => {
                        idle_states[i] = false;
                        outputs.push(out);
                        if outputs.len() == 3 {
                            let addr = outputs[0] as usize;
                            // println!("{} sends {:?} to {}", i, &outputs[1..], addr);
                            if addr == 255 {
                                nat = [outputs[1], outputs[2]];
                                if stop_on_first_packet_to_address_255 {
                                    return nat[1];
                                }
                            } else {
                                computers[addr].add_inputs(&outputs[1..]);
                            }
                            break;
                        }
                    }
                    State::Idle | State::Halted => (),
                }
            }
        }
        if idle_states.iter().all(|&is_idle| is_idle) {
            if let Some(last) = last_y_sent {
                // Exit on repeated value
                if last == nat[1] {
                    return last;
                }
            }
            last_y_sent = Some(nat[1]);
            computers[0].add_inputs(&nat);
        }
    }
}

fn initialize_computers() -> Vec<IntCodeComputer> {
    let software = parse_software_from_puzzle_input();
    (0..50)
        .into_iter()
        .map(|i| {
            let mut c = IntCodeComputer::new(software.clone());
            c.add_input(i);
            c.step(); // consume the input
            c
        })
        .collect()
}

fn parse_software_from_puzzle_input() -> Vec<isize> {
    let input = read_file_to_lines("input/day23.txt");
    input[0].split(',').map(|n| n.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(20764, day23_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(14805, day23_part2());
    }
}
