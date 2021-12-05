const INPUT: &str = include_str!("../input/day02.txt");

pub(crate) fn day02_part1() -> usize {
    let commands = parse(INPUT);
    follow_part1_commands(commands)
}

fn follow_part1_commands(commands: Vec<Command>) -> usize {
    let mut pos = Pos::default();
    for command in commands {
        match command {
            Command::Forward(v) => pos.horizontal += v,
            Command::Down(v) => pos.depth += v,
            Command::Up(v) => pos.depth -= v,
        }
    }
    pos.horizontal * pos.depth
}
pub(crate) fn day02_part2() -> usize {
    let commands = parse(INPUT);
    follow_part2_commands(commands)
}

fn follow_part2_commands(commands: Vec<Command>) -> usize {
    let mut pos = Pos::default();
    for command in commands {
        match command {
            Command::Forward(v) => {
                pos.horizontal += v;
                pos.depth += pos.aim * v;
            }
            Command::Down(v) => pos.aim += v,
            Command::Up(v) => pos.aim -= v,
        }
    }
    pos.horizontal * pos.depth
}

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}
impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let (command, value) = s.split_once(' ').unwrap();
        let value = value.parse().unwrap();
        match command {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Pos {
    horizontal: usize,
    depth: usize,
    aim: usize,
}

fn parse(input: &str) -> Vec<Command> {
    input.trim().lines().map(Command::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn example1() {
        let commands = parse(EXAMPLE);
        assert_eq!(150, follow_part1_commands(commands));
    }

    #[test]
    fn example2() {
        let commands = parse(EXAMPLE);
        assert_eq!(900, follow_part2_commands(commands));
    }

    #[test]
    fn part1() {
        assert_eq!(day02_part1(), 2322630);
    }

    #[test]
    fn part2() {
        assert_eq!(day02_part2(), 2105273490);
    }
}
