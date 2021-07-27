use std::collections::HashSet;

pub(crate) fn cumulate_frequency_adjustments(input: &[String]) -> isize {
    input.iter().map(|s| s.parse::<isize>().unwrap()).sum()
}

pub(crate) fn find_first_repeated_frequency(input: &[String]) -> isize {
    let adjustments: Vec<isize> = input.iter().map(|s| s.parse().unwrap()).collect();
    let mut seen: HashSet<isize> = HashSet::new();
    let mut freq = 0;
    loop {
        for adj in &adjustments {
            freq += adj;
            if seen.contains(&freq) {
                return freq;
            } else {
                seen.insert(freq);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE_1: &str = "1
-2
+3
+1";

    #[test]
    fn example_1() {
        assert_eq!(
            cumulate_frequency_adjustments(&read_str_to_lines(EXAMPLE_1)),
            3
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(
            cumulate_frequency_adjustments(&read_file_to_lines("input/day01.txt")),
            454
        );
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(
            find_first_repeated_frequency(&read_str_to_lines(EXAMPLE_1)),
            2
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            find_first_repeated_frequency(&read_file_to_lines("input/day01.txt")),
            566
        );
    }
}
