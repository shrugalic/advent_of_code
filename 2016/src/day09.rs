use crate::parse;

const INPUT: &str = include_str!("../input/day09.txt");

pub(crate) fn day09_part1() -> usize {
    let input = &parse(INPUT)[0];
    outer_only_decompress(input).len()
}

pub(crate) fn day09_part2() -> usize {
    let input = &parse(INPUT)[0];
    full_decompressed_len(input)
}

fn outer_only_decompress<T: AsRef<str>>(s: T) -> String {
    part1_decompress(&to_char_vec(s)).iter().collect()
}
fn part1_decompress(s: &[char]) -> Vec<char> {
    let mut res: Vec<char> = vec![];
    let mut i = 0;
    while i < s.len() {
        // println!("looking at {}", s[i]);
        if s[i] == '(' {
            let (marker_len, seq_len, seq_count) = parse_marker(s, i);
            i += marker_len;
            let seq = &s[i..i + seq_len];
            i += seq_len;
            // println!("to_repeat = {:?}", seq);
            res.append(&mut seq.repeat(seq_count));
        } else {
            res.push(s[i]);
            i += 1;
        }
    }
    res
}

fn full_decompressed_len<T: AsRef<str>>(s: T) -> usize {
    part2_decompress_len(&to_char_vec(s))
}
fn part2_decompress_len(s: &[char]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == '(' {
            let (marker_len, seq_len, seq_count) = parse_marker(s, i);
            i += marker_len;
            let seq = &s[i..i + seq_len];
            i += seq_len;
            len += seq_count * part2_decompress_len(seq);
        } else {
            i += 1;
            len += 1;
        }
    }
    len
}

fn to_char_vec<T: AsRef<str>>(s: T) -> Vec<char> {
    s.as_ref().chars().collect()
}

fn parse_marker(s: &[char], start: usize) -> (usize, usize, usize) {
    let mut end = start;
    while s[end] != ')' {
        end += 1;
    }
    end += 1; // Include ')'

    let marker: String = s[start + 1..end - 1].iter().collect::<String>();
    let marker_len = marker.len() + 2; // '(' ')'
    let (seq_len, seq_count) = marker.split_once("x").unwrap();
    let seq_len: usize = seq_len.parse().unwrap();
    let seq_count: usize = seq_count.parse().unwrap();
    // println!(
    //     "marker ({}) of len {} -> seq_len {}, seq_count = {}",
    //     marker.to_string(),
    //     marker_len,
    //     seq_len,
    //     seq_count,
    // );
    (marker_len, seq_len, seq_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_decompress_without_marker() {
        assert_eq!("ADVENT", outer_only_decompress("ADVENT"));
    }

    #[test]
    fn part1_decompress_with_1x5_marker() {
        assert_eq!("ABBBBBC", outer_only_decompress("A(1x5)BC"));
    }

    #[test]
    fn part1_decompress_with_3x3_marker() {
        assert_eq!("XYZXYZXYZ", outer_only_decompress("(3x3)XYZ"));
    }

    #[test]
    fn part1_decompress_with_ignored_1x3_marker() {
        assert_eq!("(1x3)A", outer_only_decompress("(6x1)(1x3)A"));
    }

    #[test]
    fn part1_decompress_with_ignored_3x3_marker() {
        assert_eq!(
            "X(3x3)ABC(3x3)ABCY",
            outer_only_decompress("X(8x2)(3x3)ABCY")
        );
    }

    #[test]
    fn part1() {
        assert_eq!(112_830, day09_part1());
    }

    fn decompress_repeatedly<T: AsRef<str>>(s: T) -> String {
        let mut decompressed = part1_decompress(&to_char_vec(s));
        while decompressed.contains(&'(') {
            decompressed = part1_decompress(&decompressed);
        }
        decompressed.iter().collect()
    }

    #[test]
    fn part2_decompress_with_3x3_marker() {
        assert_eq!("XYZXYZXYZ", decompress_repeatedly("(3x3)XYZ"));
    }

    #[test]
    fn part2_decompress_with_8x2_and_3x3_markers() {
        assert_eq!(
            "XABCABCABCABCABCABCY",
            decompress_repeatedly("X(8x2)(3x3)ABCY")
        );
    }

    #[test]
    fn part2_decompress_very_long() {
        assert_eq!(
            241_920,
            full_decompressed_len("(27x12)(20x12)(13x14)(7x10)(1x12)A")
        );
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(
            445,
            full_decompressed_len("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }

    #[test]
    fn part2() {
        assert_eq!(10_931_789_799, day09_part2());
    }
}
