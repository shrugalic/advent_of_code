use crate::assembunny::Computer;
use crate::parse;

const INPUT: &str = include_str!("../input/day12.txt");

pub(crate) fn day12_part1() -> isize {
    let mut computer = Computer::from(parse(INPUT));
    computer.run()
}

pub(crate) fn day12_part2() -> isize {
    let mut computer = Computer::from(parse(INPUT));
    computer.set_register('c', 1);
    computer.run()
}

#[cfg(test)]
mod tests {
    use crate::parse;

    use super::*;

    const EXAMPLE: &str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn part1_example() {
        let mut computer = Computer::from(parse(EXAMPLE));
        assert_eq!(42, computer.run());
    }

    #[test]
    fn part1() {
        assert_eq!(318_003, day12_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(9_227_657, day12_part2());
    }
}
