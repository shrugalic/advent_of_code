use crate::*;

const EXAMPLE_1: &str = "389125467";
const PUZZLE_INPUT: &str = "327465189";

fn to_vec(input: &str) -> Vec<Label> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Label)
        .collect()
}

#[test]
fn part1_example1_0rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 0)),
        to_vec("389125467")
    );
}

#[test]
fn part1_example1_1round() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 1)),
        to_vec("328915467")
    );
}

#[test]
fn part1_example1_2rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 2)),
        to_vec("325467891")
    );
}

#[test]
fn part1_example1_3rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 3)),
        to_vec("346725891")
    );
}

#[test]
fn part1_example1_4rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 4)),
        to_vec("325846791")
    );
}

#[test]
fn part1_example1_5rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 5)),
        to_vec("367925841")
    );
}

#[test]
fn part1_example1_6rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 6)),
        to_vec("367258419")
    );
}

#[test]
fn part1_example1_7rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 7)),
        to_vec("367419258")
    );
}

#[test]
fn part1_example1_8rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 8)),
        to_vec("392674158")
    );
}

#[test]
fn part1_example1_9rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 9)),
        to_vec("392657418")
    );
}

#[test]
fn part1_example1_10rounds() {
    assert_eq!(
        convert_back_to_labels(play(&mut to_vec(EXAMPLE_1), 10)),
        to_vec("374192658")
    );
}

#[test]
fn part1_example1_label_10rounds() {
    assert_eq!(
        label_part1(play(&mut to_vec(EXAMPLE_1), 10)),
        to_vec("92658374")
    );
}

#[test]
fn part1_example1_label_100rounds() {
    assert_eq!(
        label_part1(play(&mut to_vec(EXAMPLE_1), 100)),
        to_vec("67384529")
    );
}

#[test]
fn part1() {
    assert_eq!(
        label_part1(play(&mut to_vec(PUZZLE_INPUT), 100)),
        to_vec("82934675")
    );
}

// 1_000 rounds take 23s, so 10M would take around 64 hours!
const ROUND_COUNT: usize = 10_000_000;

#[test]
fn part2_example1_label_10_000_000rounds() {
    let mut cups: Vec<Label> = (1..=1_000_000).into_iter().collect();
    to_vec(EXAMPLE_1)
        .iter()
        .enumerate()
        .for_each(|(i, v)| cups[i] = *v);

    assert_eq!(label_part2(play(&mut cups, ROUND_COUNT)), 934001 * 159792);
}

#[test]
fn part2() {
    let mut cups: Vec<Label> = (1..=1_000_000).into_iter().collect();
    to_vec(PUZZLE_INPUT)
        .iter()
        .enumerate()
        .for_each(|(i, v)| cups[i] = *v);

    assert_eq!(label_part2(play(&mut cups, ROUND_COUNT)), 749102 * 633559);
}
