use std::collections::HashSet;

fn sum_of_unique_yes_answers_per_group(lines: &[String]) -> usize {
    lines
        // Split groups, which are separated by blank lines, into one slice per group
        .split(|line| line.is_empty())
        .map(|group| {
            // Join each group's slice into a single String, and
            // put each String's chars into a set to count the unique chars
            group.join("").chars().collect::<HashSet<char>>().len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::sum_of_unique_yes_answers_per_group;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    #[test]
    fn part1_example() {
        assert_eq!(
            sum_of_unique_yes_answers_per_group(&read_str_to_lines(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )),
            11
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            sum_of_unique_yes_answers_per_group(&read_file_to_lines("input.txt")),
            6437
        );
    }
}
