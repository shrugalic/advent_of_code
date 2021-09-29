use line_reader::read_file_to_lines;
use std::cmp::Ordering;
use std::collections::HashMap;

pub(crate) fn day04_part1() -> usize {
    sum_of_valid_sector_ids(read_file_to_lines("input/day04.txt"))
}

pub(crate) fn day04_part2() -> usize {
    read_file_to_lines("input/day04.txt")
        .into_iter()
        .filter_map(extract_valid_room)
        .filter(|(name, id)| decrypt(name, id) == "northpole object storage")
        .map(|(_, id)| id)
        .next()
        .unwrap()
}

fn sum_of_valid_sector_ids(input: Vec<String>) -> usize {
    input
        .into_iter()
        .filter_map(extract_valid_room)
        .map(|(_, id)| id)
        .sum()
}

fn extract_valid_room<T: AsRef<str>>(name: T) -> Option<(String, usize)> {
    let (enc_name, id_checksum) = name.as_ref().rsplit_once('-').unwrap();
    let (sector_id, actual_checksum) = id_checksum.trim_end_matches(']').split_once('[').unwrap();
    if actual_checksum == expected_checksum(enc_name) {
        Some((enc_name.to_string(), sector_id.parse().unwrap()))
    } else {
        None
    }
}

fn decrypt<T: AsRef<str>>(name: T, id: &usize) -> String {
    let mut name = name.as_ref().replace("-", " ").chars().collect::<Vec<_>>();
    let shift = (id % 26) as u8;
    name.iter_mut().for_each(|c| {
        let mut i = *c as u8;
        if (b'a'..=b'z').contains(&i) {
            i += shift;
            if i > b'z' {
                i -= 26;
            }
            *c = i as char
        }
    });
    name.iter().collect()
}

fn expected_checksum(name: &str) -> String {
    let mut frequencies = HashMap::new();
    let letters = name.replace("-", "");
    letters.chars().into_iter().for_each(|c| {
        *frequencies.entry(c).or_insert(0usize) += 1;
    });
    let mut frequencies: Vec<_> = frequencies.into_iter().collect();
    frequencies.sort_unstable_by(|(char_a, count_a), (char_b, count_b)| {
        match count_a.cmp(count_b).reverse() {
            Ordering::Equal => char_a.cmp(char_b),
            by_count => by_count,
        }
    });
    let expected_checksum: String = frequencies.into_iter().take(5).map(|(c, _)| c).collect();
    expected_checksum
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn test_extract_valid_room() {
        assert_eq!(
            Some(("aaaaa-bbb-z-y-x".to_string(), 123)),
            extract_valid_room("aaaaa-bbb-z-y-x-123[abxyz]")
        );
        assert_eq!(
            Some(("a-b-c-d-e-f-g-h".to_string(), 987)),
            extract_valid_room("a-b-c-d-e-f-g-h-987[abcde]")
        );
        assert_eq!(
            Some(("not-a-real-room".to_string(), 404)),
            extract_valid_room("not-a-real-room-404[oarel]")
        );
        assert_eq!(None, extract_valid_room("totally-real-room-200[decoy]"));
    }

    #[test]
    fn part1_example() {
        assert_eq!(1514, sum_of_valid_sector_ids(read_str_to_lines(EXAMPLE)));
    }

    #[test]
    fn part1() {
        assert_eq!(158835, day04_part1());
    }

    #[test]
    fn test_get_real_name() {
        assert_eq!("very encrypted name", decrypt("qzmt-zixmtkozy-ivhz", &343));
    }

    #[test]
    fn part2() {
        assert_eq!(993, day04_part2());
    }
}
