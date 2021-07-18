use crate::*;
use line_reader::*;

const EXAMPLE_1: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

#[test]
fn score_deck() {
    let deck: Deck = VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
    assert_eq!(score(&deck), 306);
}

#[test]
fn part_1_example_1() {
    assert_eq!(winning_players_score(&read_str_to_lines(EXAMPLE_1)), 306);
}

#[test]
fn part_1() {
    assert_eq!(
        winning_players_score(&read_file_to_lines("input.txt")),
        35299
    );
}
