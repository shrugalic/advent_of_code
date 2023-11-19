use std::collections::VecDeque;

const PUZZLE_INPUT: usize = 316;
const PART1_ITERATION_COUNT: usize = 2017;
const PART2_ITERATION_COUNT: usize = 50_000_000;

pub(crate) fn day17_part1() -> usize {
    spin_lock_part1(PUZZLE_INPUT, PART1_ITERATION_COUNT)
}

pub(crate) fn day17_part2() -> usize {
    spin_lock_part2(PUZZLE_INPUT, PART2_ITERATION_COUNT)
}

fn spin_lock_part1(step_size: usize, iteration_count: usize) -> usize {
    let mut buffer = VecDeque::with_capacity(iteration_count + 1);
    buffer.push_front(0usize);
    let mut cursor = 0;
    for i in 1..=iteration_count {
        // i is not only the element to insert, but also equal to the buffer length
        cursor = (cursor + step_size) % i + 1;
        buffer.insert(cursor, i);
    }
    buffer[cursor + 1]
}

fn spin_lock_part2(step_size: usize, iteration_count: usize) -> usize {
    let mut value = 0;
    let mut cursor = 0;
    for i in 1..=iteration_count {
        // i is not only the element to insert, but also equal to the buffer length
        cursor = (cursor + step_size) % i + 1;
        if cursor == 1 {
            value = i;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_for_9_iterations() {
        assert_eq!(5, spin_lock_part1(3, 9));
    }

    #[test]
    fn part1_example_for_2017_iterations() {
        assert_eq!(638, spin_lock_part1(3, PART1_ITERATION_COUNT));
    }

    #[test]
    fn part1() {
        assert_eq!(180, day17_part1());
    }

    #[test]
    fn part2_example_for_9_iterations() {
        assert_eq!(9, spin_lock_part2(3, 9));
    }

    #[test]
    fn part2() {
        assert_eq!(13326437, day17_part2());
    }
}
