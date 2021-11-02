use crate::assembunny::Computer;
use line_reader::read_file_to_lines;

pub(crate) fn day23_part1() -> isize {
    let mut computer = Computer::from(read_file_to_lines("input/day23.txt"));
    computer.set_register('a', 7);
    computer.run()
}

pub(crate) fn day23_part2() -> isize {
    let mut computer = Computer::from(read_file_to_lines("input/day23.txt"));
    computer.set_register('a', 12);
    computer.run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

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
        let mut computer = Computer::from(read_str_to_lines(EXAMPLE));
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
