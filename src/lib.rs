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

fn distance_from_origin_after_following_instructions(input: &[String]) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::{distance_from_origin_after_following_instructions, Action, Instr};
    use line_reader::{read_file_to_lines, read_str_to_lines};

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
            distance_from_origin_after_following_instructions(&read_file_to_lines("input.txt")),
            923
        );
    }
}
