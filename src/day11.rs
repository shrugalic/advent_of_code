use std::iter::FromIterator;
use std::ops::{Range, RangeInclusive};

pub(crate) fn count_occupied_seats_after_seating_process_became_stable(
    seats: &[String],
    seat_selection_strategy: &dyn Fn(&[Vec<char>], usize, usize) -> char,
) -> usize {
    let initial = to_vec_of_vecs_of_chars(seats);
    let stable = run_seating_process_until_stable(initial, seat_selection_strategy);
    stable
        .iter()
        .map(|s| s.iter().filter(|&c| *c == '#').count())
        .sum()
}

#[allow(unused)]
fn get_stable_seating_arrangement(
    seats: &[String],
    seat_selection_strategy: &dyn Fn(&[Vec<char>], usize, usize) -> char,
) -> Vec<String> {
    let initial = to_vec_of_vecs_of_chars(seats);
    let stable = run_seating_process_until_stable(initial, seat_selection_strategy);
    to_vec_of_strings(&stable)
}

fn run_seating_process_until_stable(
    initial: Vec<Vec<char>>,
    seat_selection_strategy: &dyn Fn(&[Vec<char>], usize, usize) -> char,
) -> Vec<Vec<char>> {
    let mut prev = vec![];
    let mut next = initial;
    while next != prev {
        prev = next.clone();
        next = run_seating_process_once(&prev, seat_selection_strategy);
    }
    next
}

fn to_vec_of_vecs_of_chars(slice_of_strings: &[String]) -> Vec<Vec<char>> {
    slice_of_strings
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

#[allow(unused)]
fn to_vec_of_strings(slice_of_vec_of_char: &[Vec<char>]) -> Vec<String> {
    slice_of_vec_of_char.iter().map(String::from_iter).collect()
}

fn run_seating_process_once(
    curr: &[Vec<char>],
    seat_selection_strategy: &dyn Fn(&[Vec<char>], usize, usize) -> char,
) -> Vec<Vec<char>> {
    let mut next = curr.to_owned();
    for (row, line) in curr.iter().enumerate() {
        for (col, _c) in line.iter().enumerate() {
            next[row][col] = seat_selection_strategy(curr, row, col);
        }
    }
    next
}

pub(crate) fn part1_seat_selection_strategy(seats: &[Vec<char>], row: usize, col: usize) -> char {
    let c = seats[row][col];
    if c == 'L' && count_occupied_neighbors(col, row, seats) == 0 {
        '#'
    } else if c == '#' && count_occupied_neighbors(col, row, seats) >= 4 {
        'L'
    } else {
        c
    }
}

fn count_occupied_neighbors(mid_col: usize, mid_row: usize, seats: &[Vec<char>]) -> usize {
    let is_neighbor = |col, row| !(col == mid_col && row == mid_row);
    let mut count = 0;
    for row in safe_range(mid_row, seats.len() - 1) {
        for col in safe_range(mid_col, seats[row].len() - 1) {
            if is_neighbor(col, row) && seats[row][col] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn safe_range(i: usize, max: usize) -> RangeInclusive<usize> {
    let min = 0;
    let lo = if i > min { i - 1 } else { i };
    let hi = if i < max { i + 1 } else { i };
    lo..=hi
}

pub(crate) fn part2_seat_selection_strategy(seats: &[Vec<char>], row: usize, col: usize) -> char {
    let c = seats[row][col];
    if c == 'L' && count_visibly_occupied_neighbors(col, row, seats) == 0 {
        '#'
    } else if c == '#' && count_visibly_occupied_neighbors(col, row, seats) >= 5 {
        'L'
    } else {
        c
    }
}

fn count_visibly_occupied_neighbors(col: usize, row: usize, seats: &[Vec<char>]) -> usize {
    // look in all 8 directions
    let directions: [(isize, isize); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    directions
        .iter()
        .filter(|dir| next_visible_seat_in_direction(dir, col, row, seats) == Some('#'))
        .count()
}

fn next_visible_seat_in_direction(
    dir: &(isize, isize),
    col: usize,
    row: usize,
    seats: &[Vec<char>],
) -> Option<char> {
    let safe_rows: Range<isize> = 0..seats.len() as isize;
    let safe_cols: Range<isize> = 0..seats[row].len() as isize;
    let mut next_col = col as isize + dir.0;
    let mut next_row = row as isize + dir.1;
    while safe_cols.contains(&next_col) && safe_rows.contains(&next_row) {
        let c = seats[next_row as usize][next_col as usize];
        if c != '.' {
            // Stop at first empty or occupied seat
            return Some(c);
        }
        next_col += dir.0;
        next_row += dir.1;
    }
    None // Outside range
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const INITIAL_STATE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    const EXAMPLE_1_STATES: [&str; 6] = [
        INITIAL_STATE,
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
        "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
        "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
    ];

    #[test]
    fn part1_single_steps() {
        for step in 1..EXAMPLE_1_STATES.len() {
            let input = to_vec_of_vecs_of_chars(&read_str_to_lines(EXAMPLE_1_STATES[step - 1]));
            let result = to_vec_of_strings(&run_seating_process_once(
                &input,
                &part1_seat_selection_strategy,
            ));
            assert_eq!(result, read_str_to_lines(EXAMPLE_1_STATES[step]));
        }
    }

    #[test]
    fn part1_example_all_steps() {
        assert_eq!(
            get_stable_seating_arrangement(
                &read_str_to_lines(EXAMPLE_1_STATES[0]),
                &part1_seat_selection_strategy
            ),
            read_str_to_lines(EXAMPLE_1_STATES[5])
        );
    }

    #[test]
    fn part1_example_count_occupied_seats() {
        assert_eq!(
            count_occupied_seats_after_seating_process_became_stable(
                &read_str_to_lines(INITIAL_STATE),
                &part1_seat_selection_strategy
            ),
            37
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            count_occupied_seats_after_seating_process_became_stable(
                &read_file_to_lines("input/day11.txt"),
                &part1_seat_selection_strategy
            ),
            2481
        );
    }

    const EXAMPLE_2_STATES: [&str; 7] = [
        INITIAL_STATE,
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
        "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
    ];

    #[test]
    fn part2_single_steps() {
        for step in 1..EXAMPLE_2_STATES.len() {
            let input = to_vec_of_vecs_of_chars(&read_str_to_lines(EXAMPLE_2_STATES[step - 1]));
            let result = to_vec_of_strings(&run_seating_process_once(
                &input,
                &part2_seat_selection_strategy,
            ));
            assert_eq!(result, read_str_to_lines(EXAMPLE_2_STATES[step]));
        }
    }

    #[test]
    fn part2_example_all_steps() {
        assert_eq!(
            get_stable_seating_arrangement(
                &read_str_to_lines(EXAMPLE_2_STATES[0]),
                &part2_seat_selection_strategy
            ),
            read_str_to_lines(EXAMPLE_2_STATES[6])
        );
    }

    #[test]
    fn part2_example_count_occupied_seats() {
        assert_eq!(
            count_occupied_seats_after_seating_process_became_stable(
                &read_str_to_lines(INITIAL_STATE),
                &part2_seat_selection_strategy
            ),
            26
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            count_occupied_seats_after_seating_process_became_stable(
                &read_file_to_lines("input/day11.txt"),
                &part2_seat_selection_strategy
            ),
            2227
        );
    }
}
