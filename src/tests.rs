#[cfg(test)]
use crate::*;
use line_reader::read_file_to_lines;

#[test]
fn part1_example_1() {
    assert_eq!(evaluate_line_part1(&"1"), 1);
}
#[test]
fn part1_example_2() {
    assert_eq!(evaluate_line_part1(&"1 + 2 * 3"), 9);
}
#[test]
fn part1_example_3() {
    assert_eq!(evaluate_line_part1(&"2 * 3 + 4"), 10);
}
#[test]
fn part1_example_4() {
    assert_eq!(evaluate_line_part1(&"1 + 2 * 3 + 4 * 5 + 6"), 71);
}
#[test]
fn part1_example_5() {
    assert_eq!(evaluate_line_part1(&"1 + (2 * 3) + (4 * (5 + 6))"), 51);
}
#[test]
fn part1_example_6() {
    assert_eq!(evaluate_line_part1(&"2 * 3 + (4 * 5)"), 26);
}
#[test]
fn part1_example_7() {
    assert_eq!(evaluate_line_part1(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
}
#[test]
fn part1_example_8() {
    assert_eq!(
        evaluate_line_part1(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        12240
    );
}
#[test]
fn part1_example_9() {
    assert_eq!(
        evaluate_line_part1(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
}
#[test]
fn part1_example_10() {
    assert_eq!(evaluate_line_part1(&"(1 * 2)"), 2);
}
#[test]
fn part1_example_11() {
    assert_eq!(evaluate_line_part1(&"((1 * 2))"), 2);
}
#[test]
fn part1() {
    assert_eq!(
        evaluate_part1(&read_file_to_lines("input.txt")),
        3348222486398
    );
}

#[test]
fn part2_example_1() {
    assert_eq!(evaluate_line_part2(&"1"), 1);
}
#[test]
fn part2_example_2() {
    assert_eq!(evaluate_line_part2(&"1 + 2 * 3"), 9);
}
#[test]
fn part2_example_3() {
    assert_eq!(evaluate_line_part2(&"2 * 3 + 4"), 14);
}
#[test]
fn part2_example_4() {
    assert_eq!(evaluate_line_part2(&"1 + 2 * 3 + 4 * 5 + 6"), 231);
}
#[test]
fn part2_example_5() {
    assert_eq!(evaluate_line_part2(&"1 + (2 * 3) + (4 * (5 + 6))"), 51);
}
#[test]
fn part2_example_6() {
    assert_eq!(evaluate_line_part2(&"2 * 3 + (4 * 5)"), 46);
}
#[test]
fn part2_example_7() {
    assert_eq!(evaluate_line_part2(&"5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
}
#[test]
fn part2_example_8() {
    assert_eq!(
        evaluate_line_part2(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        669060
    );
}
#[test]
fn part2_example_9() {
    assert_eq!(
        evaluate_line_part2(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}
#[test]
fn part2_example_10() {
    assert_eq!(evaluate_line_part2(&"(1 * 2)"), 2);
}
#[test]
fn part2_example_11() {
    assert_eq!(evaluate_line_part2(&"((1 * 2))"), 2);
}
#[test]
fn part2() {
    assert_eq!(
        evaluate_part2(&read_file_to_lines("input.txt")),
        43423343619505
    );
}
