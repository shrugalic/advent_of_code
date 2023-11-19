use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day06.txt");

pub(crate) fn day06_part1() -> usize {
    let message = parse(INPUT);
    message.count_received_chars_until(Marker::StartOfPacket)
}

pub(crate) fn day06_part2() -> usize {
    let message = parse(INPUT);
    message.count_received_chars_until(Marker::StartOfMessage)
}

trait ReceivedCharacterCounter {
    fn count_received_chars_until(&self, marker: Marker) -> usize;
}
impl ReceivedCharacterCounter for Vec<char> {
    fn count_received_chars_until(&self, marker: Marker) -> usize {
        let char_count_to_mark_start = marker as usize;
        for (index, chars) in self.windows(char_count_to_mark_start).enumerate() {
            let unique_chars: HashSet<_> = HashSet::from_iter(chars);
            if unique_chars.len() == char_count_to_mark_start {
                return index + char_count_to_mark_start;
            }
        }
        unreachable!()
    }
}

fn parse(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

enum Marker {
    StartOfPacket = 4,
    StartOfMessage = 14,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_example() {
        let message = parse(EXAMPLE);
        assert_eq!(7, message.count_received_chars_until(Marker::StartOfPacket));
    }

    #[test]
    fn part2_example() {
        let message = parse(EXAMPLE);
        assert_eq!(
            19,
            message.count_received_chars_until(Marker::StartOfMessage)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(1_876, day06_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(2_202, day06_part2());
    }
}
