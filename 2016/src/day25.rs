use crate::assembunny::Computer;
use crate::parse;

const INPUT: &str = include_str!("../input/day25.txt");

pub(crate) fn day25_part1() -> isize {
    for i in 1.. {
        let mut computer = Computer::from(parse(INPUT));
        computer.set_register('a', i);
        let result = computer.run();
        if result == 1 {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(175, day25_part1());
    }
}
