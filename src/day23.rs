use crate::assembunny::Computer;
use crate::parse;

const INPUT: &str = include_str!("../input/day23.txt");

pub(crate) fn day23_part1() -> isize {
    let mut computer = Computer::from(parse(INPUT));
    computer.set_register('a', 7);
    computer.run()
}

pub(crate) fn day23_part2() -> isize {
    let mut computer = Computer::from(parse(INPUT));
    computer.set_register('a', 12);
    computer.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE: &str = "\
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn part1_example() {
        let mut computer = Computer::from(parse(EXAMPLE));
        assert_eq!(3, computer.run());
    }

    #[test]
    fn part1() {
        assert_eq!(12_330, day23_part1());
    }

    // #[test] // Slow at 4 min 17s
    #[allow(unused)]
    fn part2() {
        assert_eq!(479_008_890, day23_part2());
    }
}
