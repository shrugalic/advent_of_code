const DIVIDEND: usize = 20201227;
const CARD_AND_DOOR_SUBJECT_NUMBER: usize = 7;

pub(crate) fn find_encryption_key(pub_key_1: usize, pub_key_2: usize) -> usize {
    let loop_size_1 = find_loop_size(pub_key_1);
    transform_subject_number(pub_key_2, loop_size_1)
}

fn find_loop_size(pub_key: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    while value != pub_key {
        value = iterate(value, CARD_AND_DOOR_SUBJECT_NUMBER);
        loop_size += 1;
    }
    loop_size
}

fn transform_subject_number(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = iterate(value, subject_number);
    }
    value
}

fn iterate(value: usize, subject_number: usize) -> usize {
    let value = value * subject_number;
    value % DIVIDEND
}

pub(crate) const DAY_25_PUZZLE_INPUT: (usize, usize) = (13316116, 13651422);

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_CARD_PUB_KEY: usize = 5764801;
    const EXAMPLE_DOOR_PUB_KEY: usize = 17807724;

    #[test]
    fn iterate_once_1() {
        assert_eq!(iterate(1, 1), 1);
    }

    #[test]
    fn iterate_once_20201227() {
        assert_eq!(iterate(20201227, 1), 0);
    }

    #[test]
    fn find_card_loop_size() {
        assert_eq!(find_loop_size(EXAMPLE_CARD_PUB_KEY), 8);
    }

    #[test]
    fn find_door_loop_size() {
        assert_eq!(find_loop_size(EXAMPLE_DOOR_PUB_KEY), 11);
    }

    #[test]
    fn example_find_encryption_key() {
        assert_eq!(
            find_encryption_key(EXAMPLE_CARD_PUB_KEY, EXAMPLE_DOOR_PUB_KEY),
            14897079
        );
    }

    #[test]
    fn part1_find_encryption_key() {
        assert_eq!(
            find_encryption_key(DAY_25_PUZZLE_INPUT.0, DAY_25_PUZZLE_INPUT.1),
            12929
        );
    }
}
