const PUZZLE_INPUT: &str = "1321131112";

pub(crate) fn day10_part1() -> usize {
    generate_next_sequence(PUZZLE_INPUT, 40)
}

pub(crate) fn day10_part2() -> usize {
    generate_next_sequence(PUZZLE_INPUT, 50)
}

fn generate_next_sequence(input: &str, count: usize) -> usize {
    let mut numbers: Vec<_> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    for _ in 0..count {
        numbers = gen_next_seq(&numbers);
    }
    numbers.len()
}

fn gen_next_seq(input: &[u32]) -> Vec<u32> {
    if input.is_empty() {
        return vec![];
    }
    let mut output: Vec<_> = vec![];
    let mut prev = None;
    let mut count = 0;
    for curr in input {
        if let Some(prev) = prev {
            if curr != &prev {
                output.push(count);
                output.push(prev);
                count = 0;
            }
        }
        count += 1;
        prev = Some(*curr);
    }
    output.push(count);
    output.push(prev.unwrap());

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(vec![1, 1,], gen_next_seq(&[1,]));
        assert_eq!(vec![2, 1,], gen_next_seq(&[1, 1]));
        assert_eq!(vec![1, 2, 1, 1,], gen_next_seq(&[2, 1]));
        assert_eq!(vec![1, 1, 1, 2, 2, 1,], gen_next_seq(&[1, 2, 1, 1]));
        assert_eq!(vec![3, 1, 2, 2, 1, 1,], gen_next_seq(&[1, 1, 1, 2, 2, 1]));
    }

    #[test]
    fn part1() {
        assert_eq!(492982, day10_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(6989950, day10_part2());
    }
}
