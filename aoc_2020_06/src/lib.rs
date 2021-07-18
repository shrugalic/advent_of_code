use std::collections::{HashMap, HashSet};

fn sum_of_unique_yes_answers_per_group(lines: &[String]) -> usize {
    lines_per_group(lines)
        .iter()
        .map(|group| {
            // Join each group's slice into a single String, and
            // put each String's chars into a set to count the unique chars
            group.join("").chars().collect::<HashSet<char>>().len()
        })
        .sum()
}

fn lines_per_group(lines: &[String]) -> Vec<Vec<String>> {
    lines
        // Split groups, which are separated by blank lines, into one slice per group
        .split(|line| line.is_empty())
        .map(|group| {
            // Convert each slice into a Vec<String> per group
            group.to_vec()
        })
        .collect()
}

fn sum_of_unique_yes_answers_per_group2(lines: &[String]) -> usize {
    let groups = lines_per_group(lines);
    // println!("groups.len() = {}", groups.len());
    groups
        .iter()
        // .inspect(|group| println!("{:?}", group))
        .map(|group| {
            // Count every vote of groups with only a single participant
            if group.len() == 1 {
                group[0].len()
            } else {
                // For other groups only count chars contained in all participants
                let mut map: HashMap<char, usize> = HashMap::new();
                group.iter().for_each(|line| {
                    line.chars().for_each(|c| {
                        *map.entry(c).or_insert(0) += 1;
                    })
                });
                map.iter()
                    .filter(|(_char, &count)| count == group.len())
                    .count()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{sum_of_unique_yes_answers_per_group, sum_of_unique_yes_answers_per_group2};
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

    #[test]
    fn part2_example() {
        assert_eq!(
            sum_of_unique_yes_answers_per_group2(&read_str_to_lines(
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
            6
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            sum_of_unique_yes_answers_per_group2(&read_file_to_lines("input.txt")),
            3229
        );
    }
}
