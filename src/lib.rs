use line_reader::{read_file_to_lines, read_str_to_lines};
use std::collections::HashMap;
use std::option::Option;

fn the_2020th_number_spoken(input: &[usize]) -> usize {
    let mut last_turn_by_num: HashMap<usize, usize> = HashMap::new();
    let mut second_to_last: Option<usize> = None;
    let mut num = 0;
    // let mut is_first = true;
    for turn in 1..=2020 {
        num = if turn - 1 < input.len() {
            input[turn - 1]
        } else {
            match second_to_last {
                Some(previous) => turn - 1 - previous,
                None => 0,
            }
        };
        second_to_last = last_turn_by_num.get(&num).cloned();
        last_turn_by_num.insert(num, turn);
    }
    num
}

#[cfg(test)]
mod tests {
    use crate::the_2020th_number_spoken;
    use line_reader::read_file_to_lines;

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
}
