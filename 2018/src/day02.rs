use std::cmp::min;
use std::collections::HashMap;

pub(crate) fn product_of_2_and_3_counts(input: &[String]) -> usize {
    let (twos, threes) = input
        .iter()
        .map(count_2_and_3_identical_letters)
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    twos * threes
}

fn count_2_and_3_identical_letters(line: &String) -> (usize, usize) {
    let mut count_by_letter: HashMap<char, usize> = HashMap::new();
    line.chars().for_each(|c| {
        *count_by_letter.entry(c).or_insert(0) += 1;
    });
    let count_of = |target| {
        count_by_letter
            .iter()
            .filter(|(_, count)| count == &&target)
            .count()
    };
    (min(1, count_of(2)), min(1, count_of(3)))
}

pub(crate) fn differing_letters_of_correct_boxes(input: &[String]) -> String {
    let mut strings = input.to_vec();
    strings.sort();
    strings
        .windows(2)
        .filter(|s| {
            s[0].chars()
                .zip(s[1].chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
        })
        .map(|s| {
            s[0].chars()
                .zip(s[1].chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect::<String>()
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    #[test]
    fn count_one_of_each() {
        assert_eq!(
            count_2_and_3_identical_letters(&"bababc".to_string()),
            (1, 1)
        );
    }

    const EXAMPLE1: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";

    #[test]
    fn example_1() {
        assert_eq!(product_of_2_and_3_counts(&read_str_to_lines(EXAMPLE1)), 12);
    }

    #[test]
    fn part_1() {
        assert_eq!(
            product_of_2_and_3_counts(&read_file_to_lines("input/day02.txt")),
            7936
        );
    }

    const EXAMPLE2: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";

    #[test]
    fn example_2() {
        assert_eq!(
            differing_letters_of_correct_boxes(&read_str_to_lines(EXAMPLE2)),
            "fgij"
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            differing_letters_of_correct_boxes(&read_file_to_lines("input/day02.txt")),
            "lnfqdscwjyteorambzuchrgpx"
        );
    }
}
