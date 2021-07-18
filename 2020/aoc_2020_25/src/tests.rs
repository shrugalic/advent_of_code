use crate::*;

const EXAMPLE_CARD_PUB_KEY: usize = 5764801;
const EXAMPLE_DOOR_PUB_KEY: usize = 17807724;

const PUZZLE_INPUT: (usize, usize) = (13316116, 13651422);

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
    assert_eq!(find_encryption_key(PUZZLE_INPUT.0, PUZZLE_INPUT.1), 12929);
}
