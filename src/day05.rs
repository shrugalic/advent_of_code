pub(crate) fn row_and_col(s: &str) -> (usize, usize) {
    let to_0_or_1 = |c| match c {
        'F' | 'L' => Some('0'),
        'B' | 'R' => Some('1'),
        _ => None,
    };
    let row = s
        .chars()
        .filter(|&c| c == 'F' || c == 'B')
        .filter_map(to_0_or_1)
        .collect::<String>();
    let row = usize::from_str_radix(&row, 2).expect("not a binary number!");
    let col = s
        .chars()
        .filter(|&c| c == 'L' || c == 'R')
        .filter_map(to_0_or_1)
        .collect::<String>();
    let col = usize::from_str_radix(&col, 2).expect("not a binary number!");
    // println!("{} -> {}", s, row);
    (row, col)
}

pub(crate) fn seat_id(row_n_col: (usize, usize)) -> usize {
    row_n_col.0 * 8 + row_n_col.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_file_to_lines;

    #[test]
    fn seat_id_works() {
        assert_eq!(seat_id((44, 5)), 357);
    }

    #[test]
    fn row_and_col_works() {
        assert_eq!(row_and_col("FBFBBFFRLR"), (44, 5));
        assert_eq!(row_and_col("BFFFBBFRRR"), (70, 7));
        assert_eq!(row_and_col("FFFBBBFRRR"), (14, 7));
        assert_eq!(row_and_col("BBFFBBFRLL"), (102, 4));
    }

    #[test]
    fn part1() {
        let lines = read_file_to_lines("input/day05.txt");
        let max_seat_id = lines
            .iter()
            .map(|line| seat_id(row_and_col(line)))
            .max()
            .expect("Empty list?");
        assert_eq!(max_seat_id, 970);
    }

    #[test]
    fn part2() {
        let lines = read_file_to_lines("input/day05.txt");
        let mut seat_ids: Vec<usize> = lines
            .iter()
            .map(|line| seat_id(row_and_col(line)))
            .collect();
        seat_ids.sort_unstable();
        let neighbors: Vec<usize> = seat_ids
            .iter()
            .zip(seat_ids.iter().skip(1))
            .zip(seat_ids.iter().skip(2))
            .filter_map(|((&a, &b), &c)| {
                // println!("{} < {} < {}", a, b, c);
                if a != b - 1 || b + 1 != c {
                    Some(b)
                } else {
                    None
                }
                // } else
            })
            .collect();
        assert_eq!(neighbors.len(), 2);
        assert_eq!((neighbors[0] + neighbors[1]) / 2, 587);
    }
}
