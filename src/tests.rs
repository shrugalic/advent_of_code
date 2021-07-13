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
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 0), to_vec("389125467"));
}

#[test]
fn part1_example1_1round() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 1), to_vec("328915467"));
}

#[test]
fn part1_example1_2rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 2), to_vec("325467891"));
}

#[test]
fn part1_example1_3rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 3), to_vec("725891346"));
}

#[test]
fn part1_example1_4rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 4), to_vec("325846791"));
}

#[test]
fn part1_example1_5rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 5), to_vec("925841367"));
}

#[test]
fn part1_example1_6rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 6), to_vec("725841936"));
}

#[test]
fn part1_example1_7rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 7), to_vec("836741925"));
}

#[test]
fn part1_example1_8rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 8), to_vec("741583926"));
}

#[test]
fn part1_example1_9rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 9), to_vec("574183926"));
}

#[test]
fn part1_example1_10rounds() {
    assert_eq!(play(&mut to_vec(EXAMPLE_1), 10), to_vec("583741926"));
}

#[test]
fn part1_example1_label_10rounds() {
    assert_eq!(
        label_part1(&mut play(&mut to_vec(EXAMPLE_1), 10)),
        to_vec("92658374")
    );
}

#[test]
fn part1_example1_label_100rounds() {
    assert_eq!(
        label_part1(&mut play(&mut to_vec(EXAMPLE_1), 100)),
        to_vec("67384529")
    );
}

#[test]
fn part1() {
    assert_eq!(
        label_part1(&mut play(&mut to_vec(PUZZLE_INPUT), 100)),
        to_vec("82934675")
    );
}
