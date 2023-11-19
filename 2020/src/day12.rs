#[derive(PartialEq, Debug, Clone)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}
impl<T> From<T> for Action
where
    T: AsRef<str>,
{
    fn from(op: T) -> Self {
        match op.as_ref() {
            "N" => Action::N,
            "S" => Action::S,
            "E" => Action::E,
            "W" => Action::W,
            "L" => Action::L,
            "R" => Action::R,
            "F" => Action::F,
            inv_act => panic!("Invalid action '{}'", inv_act),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Instr {
    action: Action,
    value: usize,
}
impl<T> From<T> for Instr
where
    T: AsRef<str>,
{
    fn from(instr: T) -> Self {
        let (act, count) = instr.as_ref().split_at(1);
        Instr {
            action: Action::from(act),
            value: count.parse().expect("number"),
        }
    }
}

pub(crate) fn distance_from_origin_after_following_instructions(input: &[String]) -> usize {
    let instructions: Vec<Instr> = input.iter().map(Instr::from).collect();

    let mut pos: (isize, isize) = (0, 0); // start at origin…
    let mut dir: usize = 0; // …facing east (0 is east, 90 is south, 180 is west, 270 is north
    for mut instr in instructions {
        // Convert forward action into a directional action
        if instr.action == Action::F {
            instr.action = match dir {
                0 => Action::E,
                90 => Action::S,
                180 => Action::W,
                270 => Action::N,
                dir => panic!("Unexpected direction {}", dir),
            };
        }
        match instr.action {
            Action::N => pos.1 += instr.value as isize,
            Action::S => pos.1 -= instr.value as isize,
            Action::E => pos.0 += instr.value as isize,
            Action::W => pos.0 -= instr.value as isize,
            Action::L => dir = (dir + 360 - instr.value) % 360,
            Action::R => dir = (dir + instr.value) % 360,
            _ => panic!("Unexpected instr {:?}", instr),
        }
    }

    (pos.0.abs() + pos.1.abs()) as usize
}

pub(crate) fn distance_from_origin_after_following_instructions_part2(input: &[String]) -> usize {
    let instructions: Vec<Instr> = input.iter().map(Instr::from).collect();

    let mut pos: (isize, isize) = (0, 0); // start at origin…
    let mut waypoint: (isize, isize) = (10, 1);
    // println!(
    //     "Pos ({},{}), Waypoint ({}, {}) Dir = {}",
    //     pos.0, pos.1, waypoint.0, waypoint.1, dir
    // );
    for instr in instructions {
        match instr.action {
            Action::N => waypoint.1 += instr.value as isize,
            Action::S => waypoint.1 -= instr.value as isize,
            Action::E => waypoint.0 += instr.value as isize,
            Action::W => waypoint.0 -= instr.value as isize,
            Action::L => waypoint = rotate(waypoint, -(instr.value as isize)),
            Action::R => waypoint = rotate(waypoint, instr.value as isize),
            Action::F => {
                pos.0 += instr.value as isize * waypoint.0;
                pos.1 += instr.value as isize * waypoint.1;
            }
        }
        // println!(
        //     "Pos ({},{}) Waypoint ({}, {}) Dir = {}; Instr = {:?}",
        //     pos.0, pos.1, waypoint.0, waypoint.1, dir, instr
        // );
    }
    // println!(
    //     "Pos ({},{}), Waypoint ({}, {}) Dir = {}",
    //     pos.0, pos.1, waypoint.0, waypoint.1, dir
    // );

    (pos.0.abs() + pos.1.abs()) as usize
}

fn rotate(waypoint: (isize, isize), mut deg: isize) -> (isize, isize) {
    if deg < 0 {
        deg += 360;
    }
    match deg {
        90 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (-waypoint.1, waypoint.0),
        _ => panic!("Unexpected rotation {}", deg),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line_reader::{read_file_to_lines, read_str_to_lines};

    #[test]
    fn action_from_text() {
        assert_eq!(Action::from("N"), Action::N);
    }

    #[test]
    fn instruction_from_text() {
        assert_eq!(
            Instr::from("N10"),
            Instr {
                action: Action::N,
                value: 10
            }
        );
    }

    const EXAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn part1_example() {
        assert_eq!(
            distance_from_origin_after_following_instructions(&read_str_to_lines(EXAMPLE)),
            25
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            distance_from_origin_after_following_instructions(&read_file_to_lines(
                "input/day12.txt"
            )),
            923
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            distance_from_origin_after_following_instructions_part2(&read_str_to_lines(EXAMPLE)),
            286
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            distance_from_origin_after_following_instructions_part2(&read_file_to_lines(
                "input/day12.txt"
            )),
            24769
        );
    }
}
