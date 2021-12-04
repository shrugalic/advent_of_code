use line_reader::read_file_to_lines;
use std::collections::HashSet;

pub(crate) fn day04_part1() -> usize {
    let mut bingo = Bingo::from(read_file_to_lines("input/day04.txt"));
    bingo.score_of_winning_board()
}

pub(crate) fn day04_part2() -> usize {
    let mut bingo = Bingo::from(read_file_to_lines("input/day04.txt"));
    bingo.score_of_losing_board()
}

struct Bingo {
    drawn_numbers: Vec<u8>,
    boards: Vec<Board>,
}
impl From<Vec<String>> for Bingo {
    fn from(input: Vec<String>) -> Self {
        Bingo {
            drawn_numbers: input[0].split(',').filter_map(|s| s.parse().ok()).collect(),
            boards: input[2..]
                .split(String::is_empty)
                .map(Board::from)
                .collect(),
        }
    }
}
impl Bingo {
    fn score_of_winning_board(&mut self) -> usize {
        for number in self.drawn_numbers.iter() {
            for board in self.boards.iter_mut() {
                board.draw(number);
                if board.has_complete_row_or_column() {
                    return board.score(number);
                }
            }
        }
        unreachable!("No board won at all")
    }
    fn score_of_losing_board(&mut self) -> usize {
        let board_count = self.boards.len();
        let mut winners = HashSet::new();
        for number in self.drawn_numbers.iter() {
            for (idx, board) in self.boards.iter_mut().enumerate() {
                board.draw(number);
                if board.has_complete_row_or_column()
                    && winners.insert(idx)
                    && winners.len() == board_count
                {
                    return board.score(number);
                }
            }
        }
        unreachable!("At least one board did not win at all")
    }
}

struct Board {
    board: Vec<Vec<(u8, bool)>>,
}
impl From<&[String]> for Board {
    fn from(lines: &[String]) -> Self {
        Board {
            board: lines
                .iter()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|n| (n.parse().unwrap(), false))
                        .collect()
                })
                .collect(),
        }
    }
}
impl Board {
    fn draw(&mut self, drawn_number: &u8) {
        for row in self.board.iter_mut() {
            for (number, marked) in row.iter_mut() {
                if number == drawn_number {
                    *marked = true;
                    return; // Just to safe time
                }
            }
        }
    }
    fn has_complete_row_or_column(&self) -> bool {
        self.board
            .iter()
            .any(|row| row.iter().all(|(_, marked)| *marked))
            || (0..5)
                .into_iter()
                .any(|col_idx| self.board.iter().all(|row| row[col_idx].1))
    }

    fn score(&self, last_number: &u8) -> usize {
        let sum_of_unmarked_numbers = self
            .board
            .iter()
            .flat_map(|row| row.iter())
            .filter(|(_, marked)| !*marked)
            .map(|(unmarked_number, _)| *unmarked_number as usize)
            .sum::<usize>();
        sum_of_unmarked_numbers * *last_number as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_example() {
        let mut bingo = Bingo::from(read_str_to_lines(EXAMPLE));
        assert_eq!(188 * 24, bingo.score_of_winning_board());
    }

    #[test]
    fn part2_example() {
        let mut bingo = Bingo::from(read_str_to_lines(EXAMPLE));
        assert_eq!(148 * 13, bingo.score_of_losing_board());
    }

    #[test]
    fn part1() {
        assert_eq!(day04_part1(), 640 * 46);
    }

    #[test]
    fn part2() {
        assert_eq!(day04_part2(), 267 * 52);
    }
}
