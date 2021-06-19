use std::cmp::Ordering;
use std::collections::VecDeque;

fn first_invalid_digit(numbers: &[String], window_len: usize) -> usize {
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
    use crate::first_invalid_digit;
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
            first_invalid_digit(&read_file_to_lines("input.txt"), 25),
            258585477
        );
    }
}
