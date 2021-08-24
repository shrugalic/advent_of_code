use line_reader::read_file_to_lines;

pub(crate) fn day5_part1() -> usize {
    let jump_offsets = parse_input();
    steps_to_reach_the_exit_part1(jump_offsets)
}

pub(crate) fn day5_part2() -> usize {
    let jump_offsets = parse_input();
    steps_to_reach_the_exit_part2(jump_offsets)
}

fn parse_input() -> Vec<isize> {
    read_file_to_lines("input/day05.txt")
        .iter()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn steps_to_reach_the_exit_part1(jump_offsets: Vec<isize>) -> usize {
    steps_to_reach_the_exit(jump_offsets, false)
}

fn steps_to_reach_the_exit_part2(jump_offsets: Vec<isize>) -> usize {
    steps_to_reach_the_exit(jump_offsets, true)
}

fn steps_to_reach_the_exit(mut jump_offsets: Vec<isize>, use_part2_rule: bool) -> usize {
    let mut counter = 0;
    let mut ptr = 0isize;
    while 0 <= ptr && (ptr as usize) < jump_offsets.len() {
        let offset = jump_offsets[ptr as usize];
        if use_part2_rule && offset >= 3 {
            jump_offsets[ptr as usize] -= 1;
        } else {
            jump_offsets[ptr as usize] += 1;
        }
        ptr += offset;
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(5, steps_to_reach_the_exit_part1(vec![0, 3, 0, 1, -3]))
    }

    #[test]
    fn part1() {
        assert_eq!(375042, day5_part1());
    }

    #[test]
    fn examples_part2() {
        assert_eq!(10, steps_to_reach_the_exit_part2(vec![0, 3, 0, 1, -3]))
    }

    #[test]
    fn part2() {
        assert_eq!(28707598, day5_part2());
    }
}
