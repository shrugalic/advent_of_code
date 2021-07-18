fn the_2020th_number_spoken(input: &[usize]) -> usize {
    the_nth_number_spoken(input, 2_020)
}

fn the_30_000_000th_number_spoken(input: &[usize]) -> usize {
    the_nth_number_spoken(input, 30_000_000)
}

fn the_nth_number_spoken(input: &[usize], limit: usize) -> usize {
    let mut last_turn_by_num = vec![None; limit];
    input
        .iter()
        .enumerate()
        .for_each(|(i, num)| last_turn_by_num[*num] = Some(i + 1));
    let mut second_to_last = None;
    let mut num = input[input.len() - 1];
    for turn in input.len() + 1..=limit {
        num = match second_to_last {
            Some(previous) => turn - 1 - previous,
            None => 0,
        };
        second_to_last = last_turn_by_num[num];
        last_turn_by_num[num] = Some(turn);
    }
    num
}

#[cfg(test)]
mod tests {
    use crate::{the_2020th_number_spoken, the_30_000_000th_number_spoken};

    #[test]
    fn part1_examples() {
        assert_eq!(the_2020th_number_spoken(&[0, 3, 6]), 436);
        assert_eq!(the_2020th_number_spoken(&[1, 3, 2]), 1);
        assert_eq!(the_2020th_number_spoken(&[2, 1, 3]), 10);
        assert_eq!(the_2020th_number_spoken(&[1, 2, 3]), 27);
        assert_eq!(the_2020th_number_spoken(&[2, 3, 1]), 78);
        assert_eq!(the_2020th_number_spoken(&[3, 2, 1]), 438);
        assert_eq!(the_2020th_number_spoken(&[3, 1, 2]), 1836);
    }

    #[test]
    fn part1() {
        assert_eq!(the_2020th_number_spoken(&[0, 6, 1, 7, 2, 19, 20]), 706);
    }

    #[test]
    fn part2_examples_1() {
        assert_eq!(the_30_000_000th_number_spoken(&[0, 3, 6]), 175_594);
    }

    #[test]
    fn part2_examples_2() {
        assert_eq!(the_30_000_000th_number_spoken(&[1, 3, 2]), 2_578);
    }

    #[test]
    fn part2_examples_3() {
        assert_eq!(the_30_000_000th_number_spoken(&[2, 1, 3]), 3544_142);
    }

    #[test]
    fn part2_examples_4() {
        assert_eq!(the_30_000_000th_number_spoken(&[1, 2, 3]), 261_214);
    }

    #[test]
    fn part2_examples_5() {
        assert_eq!(the_30_000_000th_number_spoken(&[2, 3, 1]), 6_895_259);
    }

    #[test]
    fn part2_examples_6() {
        assert_eq!(the_30_000_000th_number_spoken(&[3, 2, 1]), 18);
    }

    #[test]
    fn part2_examples_7() {
        assert_eq!(the_30_000_000th_number_spoken(&[3, 1, 2]), 362);
    }

    #[test]
    fn part2() {
        assert_eq!(
            the_30_000_000th_number_spoken(&[0, 6, 1, 7, 2, 19, 20]),
            19331
        );
    }
}
