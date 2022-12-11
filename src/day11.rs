use rayon::prelude::*;
use std::ops::RangeInclusive;

pub(crate) fn day11_part1() -> (PowerLevel, Coord, Coord) {
    largest_total_power_3x3_square(8199)
}

pub(crate) fn day11_part2() -> (PowerLevel, Coord, Coord, usize) {
    largest_total_power_variable_size_square(8199)
}

type Coord = usize;
type GridSerial = usize;
type PowerLevel = isize;
type ColumnOf<PowerLevel> = Vec<PowerLevel>;
type RowOf<ColumnOfPowerLevels> = Vec<ColumnOfPowerLevels>;

const SIZE_RANGE: RangeInclusive<usize> = 1..=300;
// This range should be 1..=300 for part 2, but that used to be quite slow
//
// For the second large example, a range of 145..150 takes 1m25s with a single core (!),
// or about 10s with rayon (8-core 16-thread 9900K). So I guess at most 10 minutes for 1..=300.
//
// It actually took 5 min and 10s to brute force either of the large examples or part 2 itself.
//
// A smarter approach, particularly for larger squares, might be to only fully calculate its
// total power once, say at coordinate (1,1), and when subsequently shifting it by 1 position to
// 1. subtract the total power of the removed border and
// 2. add the total power of the added border
//
// Another idea might be binary space partitioning?
// In addition to pre-computing all 1x1 power levels, one could add 2x2, 4x4, 8x8 etc.,
// in order to combine these power-of-two squares into the larger square to test.
//
// For example a 7x7 square contains a 4x4s, five 2x2 and 13 1x1 squares,
// so a sum of 19 numbers instead of the 49 1x1 with the naive approach.
//
// The position and set-up of these inner power-of-two squares changes when the
// outer square shifts around, but their number won't change.
// A 7x7 square shifting to the right:
// |_|_|_|_|_|_|_|    |_|_|_|_|_|_|_|    |_|_|_|_|_|_|_|
// |_|       | B |    |       | B |_|    |_| H |       |
// |_|   A   |___|    |   A   |___|_|    |_|___|   G   |
// |_|       | C | -> |       | C |_| -> |_| I |       |
// |_|_______|___|    |_______|___|_|    |_|___|_______|
// |_| D | E | F |    | D | E | F |_|    |_| E | F | J |
// |_|___|___|___|    |___|___|___|_|    |_|___|___|___|
//
// Also, when doing 3x3 squares for example, one could also do 6x6, 12x12 etc at the same time.
// Or more generally for all multiples of prime numbers maybe?

struct PowerLevelRow {
    row: RowOf<PowerLevel>,
}
impl PowerLevelRow {
    fn new() -> Self {
        PowerLevelRow { row: vec![0; 301] }
    }
    fn total_power_of_square_with_x_range(&self, x_range: RangeInclusive<Coord>) -> PowerLevel {
        self.row[x_range].iter().sum()
    }
    fn diff_to_next_x_range(&self, x_range: RangeInclusive<Coord>) -> PowerLevel {
        self.row[*x_range.end()] - self.row[x_range.start() - 1]
    }
}
struct PowerLevelSquare {
    row_of_cols: RowOf<ColumnOf<PowerLevel>>,
}
impl PowerLevelSquare {
    fn new(sn: GridSerial) -> Self {
        // This starting at 0 also calculates power levels for x and y of 0, which is not needed,
        // but otherwise collecting would only fill indices 0..299
        let power_levels = (0..=300)
            .into_iter()
            .map(|x| {
                (0..=300)
                    .into_iter()
                    .map(|y| PowerLevelSquare::power_level(&x, &y, &sn))
                    .collect::<Vec<PowerLevel>>()
            })
            .collect();
        PowerLevelSquare {
            row_of_cols: power_levels,
        }
    }

    fn total_power(&self, xs: &[Coord], ys: &[Coord]) -> PowerLevel {
        let run_slowly = false;
        if run_slowly {
            // This version is considerably slower than the one below:
            // Size range 30..=30 takes ~5s, 50 ~13s, 100 ~32s, 150 ~42s (single core)
            xs.iter()
                .flat_map(|x| ys.iter().map(move |y| self.row_of_cols[*x][*y]))
                .sum()
        } else {
            // This version takes ~2s, ~4s ~11s and ~14s for the same sizes (single core)
            let x_range = *xs.first().unwrap()..=*xs.last().unwrap();
            let y_range = *ys.first().unwrap()..=*ys.last().unwrap();
            self.row_of_cols[x_range]
                .iter()
                .map(|col| col[y_range.clone()].iter().sum::<PowerLevel>())
                .sum()
        }
    }

    fn init_row_of_power_levels_of_partial_columns(
        &self,
        y_range: RangeInclusive<Coord>,
    ) -> PowerLevelRow {
        PowerLevelRow {
            row: self
                .row_of_cols
                .iter()
                .enumerate()
                // .skip(1) // x == 0 is not needed
                .map(|(_x, col)| col[y_range.clone()].iter().sum())
                .collect(),
        }
    }

    fn update_row_with_next_y_range(
        &self,
        y_range: RangeInclusive<Coord>,
        row: &mut PowerLevelRow,
    ) {
        self.row_of_cols
            .iter()
            .enumerate()
            .skip(1)
            .for_each(|(x, col)| {
                row.row[x] += col[*y_range.end()] - col[y_range.start() - 1];
            });
    }

    fn power_level(x: &Coord, y: &Coord, serial: &GridSerial) -> PowerLevel {
        let rack_id = x + 10;
        ((rack_id * y + serial) * rack_id / 100 % 10) as PowerLevel - 5
    }
}

fn largest_total_power_3x3_square(sn: GridSerial) -> (PowerLevel, Coord, Coord) {
    let power_levels = PowerLevelSquare::new(sn);
    let three_hundred: Vec<Coord> = (1..=300).into_iter().collect();
    three_hundred
        .windows(3)
        .map(|x| {
            three_hundred
                .windows(3)
                .map(|y| (power_levels.total_power(x, y), x[0], y[0]))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn largest_total_power_variable_size_square(sn: GridSerial) -> (PowerLevel, Coord, Coord, usize) {
    let power_levels = PowerLevelSquare::new(sn);
    let three_hundred = (1..=300).into_iter().collect::<Vec<Coord>>();
    SIZE_RANGE
        .into_par_iter()
        .map(|size| {
            let mut row: PowerLevelRow = PowerLevelRow::new();
            three_hundred
                .windows(size)
                .map(|ys| {
                    let y_start = *ys.first().unwrap();
                    let y_end = *ys.last().unwrap();
                    let y_range = y_start..=y_end;
                    // init or update a full row of columns with height size
                    if y_start == 1 {
                        // init all columns for this row 1
                        row = power_levels.init_row_of_power_levels_of_partial_columns(y_range);
                    } else {
                        // update row of columns by adding row below and removing row above
                        power_levels.update_row_with_next_y_range(y_range, &mut row);
                    }

                    // form squares of given size
                    let mut square_power = 0;
                    three_hundred
                        .windows(size)
                        .map(|xs| {
                            let x_start = *xs.first().unwrap();
                            let x_end = *xs.last().unwrap();
                            let x_range = x_start..=x_end;
                            square_power = if x_start == 1 {
                                // init square
                                row.total_power_of_square_with_x_range(x_range)
                            } else {
                                // update square by adding next column and subtracting last column
                                square_power + row.diff_to_next_x_range(x_range)
                            };
                            (square_power, xs[0], ys[0], size)
                        })
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_power_levels() {
        assert_eq!(4, PowerLevelSquare::power_level(&3, &5, &8));
        assert_eq!(-5, PowerLevelSquare::power_level(&122, &79, &57));
        assert_eq!(0, PowerLevelSquare::power_level(&217, &196, &39));
        assert_eq!(4, PowerLevelSquare::power_level(&101, &153, &71));
    }

    #[test]
    fn top_left_coord_of_largest_total_power_3x3_square_examples() {
        assert_eq!((29, 33, 45), largest_total_power_3x3_square(18));
        assert_eq!((30, 21, 61), largest_total_power_3x3_square(42));
    }

    #[test]
    fn part1() {
        assert_eq!((28, 235, 87), day11_part1());
    }

    #[test] // Slow, ~5 minutes
    fn largest_total_power_variable_size_square_example_1() {
        assert_eq!(
            (113, 90, 269, 16),
            largest_total_power_variable_size_square(18)
        );
    }

    #[test] // Slow, ~5 minutes
    fn largest_total_power_variable_size_square_example_2() {
        assert_eq!(
            (119, 232, 251, 12),
            largest_total_power_variable_size_square(42)
        );
    }

    #[test] // Slow, ~5 minutes
    fn part2() {
        assert_eq!((119, 234, 272, 18), day11_part2());
    }
}
