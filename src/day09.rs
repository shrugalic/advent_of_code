use std::cmp::Ordering;
use std::collections::VecDeque;

pub(crate) fn find_encryrption_weakness(numbers: &[String], target: usize) -> usize {
    let v = set_of_numbers_summing_up_to_target(numbers, target);
    v.iter().min().unwrap() + v.iter().max().unwrap()
}

fn set_of_numbers_summing_up_to_target(numbers: &[String], target: usize) -> Vec<usize> {
    let numbers: Vec<usize> = numbers
        .iter()
        .map(|s| s.parse().unwrap())
        .filter(|&n| n < target)
        .collect();
    // println!("Numbers of remaining numbers =  {}", numbers.len());
    // println!("Remaining numbers {:?}", numbers);
    // let sum = |lo,hi| numbers.iter().skip(lo-1).take(hi-lo +1).sum();
    let mut lo = 0;
    let mut hi = lo + 1;
    let group = |lo, hi| numbers.iter().skip(lo).take(hi - lo + 1);
    while hi < numbers.len() {
        match target.cmp(&group(lo, hi).sum()) {
            Ordering::Equal => return group(lo, hi).cloned().collect(),
            Ordering::Greater => hi += 1,
            Ordering::Less => {
                lo += 1;
                hi = lo + 1;
            }
        }
        // println!("Sum of {:?} = {}, target = {}", v, s, target);
    }

    vec![]
}

pub(crate) fn first_invalid_digit(numbers: &[String], window_len: usize) -> usize {
    let numbers: Vec<usize> = numbers.iter().map(|s| s.parse().unwrap()).collect();
    let mut candidates: VecDeque<_> = numbers.iter().take(window_len).collect();
    for n in numbers.iter().skip(window_len) {
        // println!("num[{}] = {}, candidates = {:?}", i, n, candidates);
        if !is_valid(n, &candidates) {
            return *n;
        }
        candidates.pop_front();
        candidates.push_back(n);
    }
    1
}

fn is_valid(n: &usize, candidates: &VecDeque<&usize>) -> bool {
    let mut sorted: Vec<_> = candidates.iter().cloned().collect();
    sorted.sort();
    // println!("target = {}, sorted candidates = {:?}", n, sorted);
    let mut lo = 0;
    let mut hi = sorted.len() - 1;
    while lo < hi {
        match n.cmp(&(sorted[lo] + sorted[hi])) {
            Ordering::Equal => return true,
            Ordering::Greater => lo += 1,
            Ordering::Less => hi -= 1,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part1_example() {
        assert_eq!(first_invalid_digit(&read_str_to_lines(EXAMPLE), 5), 127);
    }

    #[test]
    fn part1() {
        assert_eq!(
            first_invalid_digit(&read_file_to_lines("input/day09.txt"), 25),
            258585477
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            set_of_numbers_summing_up_to_target(&read_str_to_lines(EXAMPLE), 127),
            vec![15, 25, 47, 40]
        );
    }

    #[test]
    fn part2_example_weakness() {
        assert_eq!(
            find_encryrption_weakness(&read_str_to_lines(EXAMPLE), 127),
            62
        );
    }

    // #[test]
    // fn part2_result() {
    //     assert_eq!(
    //         set_of_numbers_summing_up_to_target(&read_file_to_lines("input/day09.txt"), 258585477),
    //         vec![
    //             13858643, 9455395, 9908827, 16794010, 13221299, 11563238, 12646458, 11137204,
    //             11774548, 12220424, 14302571, 14304519, 14748447, 25865809, 22680253, 16578014,
    //             27525818
    //         ]
    //     );
    // }

    #[test]
    fn part2_weakness() {
        assert_eq!(
            find_encryrption_weakness(&read_file_to_lines("input/day09.txt"), 258585477),
            36981213
        );
    }
}
