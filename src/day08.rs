use line_reader::read_file_to_lines;

pub(crate) fn day08_part1() -> usize {
    count_unescaping_overhead(read_file_to_lines("input/day08.txt"))
}

pub(crate) fn day08_part2() -> usize {
    count_escaping_overhead(read_file_to_lines("input/day08.txt"))
}

fn count_unescaping_overhead(input: Vec<String>) -> usize {
    let (orig_escaped_count, unescaped_count) = input
        .iter()
        .map(|s| get_unescaped_counts(s))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();
    orig_escaped_count - unescaped_count
}

fn count_escaping_overhead(input: Vec<String>) -> usize {
    let (orig_unescaped_count, escaped_count) = input
        .iter()
        .map(|s| get_escaped_counts(s))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();
    escaped_count - orig_unescaped_count
}

fn get_unescaped_counts(s: &str) -> (usize, usize) {
    let mut replaced: Vec<char> = s
        .trim_start_matches('\"')
        .trim_end_matches('\"')
        .replace("\\\\", "\\")
        .replace("\\\"", "\"")
        .chars()
        .collect();
    while let Some(pos) = replaced.windows(4).position(|w| {
        w[0] == '\\' && w[1] == 'x' && w[2].is_ascii_hexdigit() && w[3].is_ascii_hexdigit()
    }) {
        let part = replaced[pos + 2..pos + 4].iter().collect::<String>();
        let num = u8::from_str_radix(&part, 16).unwrap();
        // Larger values are not considered ASCII and are counted as length 2 instead of 1
        let char = (num % 128) as char;
        replaced.push(char);
        replaced.swap_remove(pos);
        replaced.remove(pos + 1);
        replaced.remove(pos + 1);
        replaced.remove(pos + 1);
    }
    let replaced = replaced.into_iter().collect::<String>();
    (s.len(), replaced.len())
}

fn get_escaped_counts(s: &str) -> (usize, usize) {
    let mut replaced: String = s.replace("\\", "\\\\").replace("\"", "\\\"");
    replaced.insert(0, '\"');
    replaced.push('\"');
    (s.len(), replaced.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\\\\"
\"\\x27\"";

    #[test]
    fn part1_example() {
        assert_eq!((2, 0), get_unescaped_counts("\"\"")); // ""
        assert_eq!((5, 3), get_unescaped_counts("\"abc\"")); // "abc"
        assert_eq!((10, 7), get_unescaped_counts("\"aaa\\\"aaa\"")); // "aaa\"aaa"
        assert_eq!((4, 1), get_unescaped_counts("\"\\\\\"")); // "\\"
        assert_eq!((6, 1), get_unescaped_counts("\"\\x27\"")); // "\x27"
        assert_eq!(
            2 + 2 + 3 + 3 + 5,
            count_unescaping_overhead(read_str_to_lines(EXAMPLE))
        );

        assert_eq!(
            (41, 30),
            get_unescaped_counts("\"nbydghkfvmq\\\\\\xe0\\\"lfsrsvlsj\\\"i\\x61liif\"")
        ); // "nbydghkfvmq\\\xe0\"lfsrsvlsj\"i\x61liif"
    }
    #[test]
    fn part1() {
        assert_eq!(1371, day08_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!((2, 6), get_escaped_counts("\"\"")); // ""
        assert_eq!((5, 9), get_escaped_counts("\"abc\"")); // "abc"
        assert_eq!((10, 16), get_escaped_counts("\"aaa\\\"aaa\"")); // "aaa\"aaa"
        assert_eq!((4, 10), get_escaped_counts("\"\\\\\"")); // "\\"
        assert_eq!((6, 11), get_escaped_counts("\"\\x27\"")); // "\x27"
        assert_eq!(
            4 + 4 + 6 + 6 + 5,
            count_escaping_overhead(read_str_to_lines(EXAMPLE))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(2117, day08_part2());
    }
}
