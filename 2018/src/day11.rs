use rayon::prelude::*;
use std::ops::RangeInclusive;

type Coord = usize;
type GridSerial = usize;
type PowerLevel = isize;
type ColumnOf<PowerLevel> = Vec<PowerLevel>;
type RowOf<ColumnOfPowerLevels> = Vec<ColumnOfPowerLevels>;

const SIZE_RANGE: RangeInclusive<usize> = 1..=300;
// This range should be 1..=300 for part 2, but that is quite slow
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

struct PowerLevels {
    power_levels: RowOf<ColumnOf<PowerLevel>>,
}
impl PowerLevels {
    fn new(sn: GridSerial) -> Self {
        // This starting at 0 also calculates power levels for x and y of 0, which is not needed,
        // but otherwise collecting would only fill indices 0..299
        let power_levels = (0..=300)
            .into_iter()
            .map(|x| {
                (0..=300)
                    .into_iter()
                    .map(|y| PowerLevels::power_level(&x, &y, &sn))
                    .collect::<Vec<PowerLevel>>()
            })
            .collect();
        PowerLevels { power_levels }
    }

    fn total_power(&self, xs: &[Coord], ys: &[Coord]) -> PowerLevel {
        if false {
            // This version is considerably slower than the one below:
            // Size range 30..=30 takes ~5s, 50 ~13s, 100 ~32s, 150 ~42s (single core)
            xs.iter()
                .flat_map(|x| ys.iter().map(move |y| self.power_levels[*x][*y]))
                .sum()
        } else {
            // This version takes ~2s, ~4s ~11s and ~14s for the same sizes (single core)
            let x_range = *xs.first().unwrap()..=*xs.last().unwrap();
            let y_range = *ys.first().unwrap()..=*ys.last().unwrap();
            self.power_levels[x_range]
                .iter()
                .map(|col| col[y_range.clone()].iter().sum::<PowerLevel>())
                .sum()
        }
    }

    fn power_level(x: &Coord, y: &Coord, serial: &GridSerial) -> PowerLevel {
        let rack_id = x + 10;
        ((rack_id * y + serial) * rack_id / 100 % 10) as PowerLevel - 5
    }
}

pub(crate) fn largest_total_power_3x3_square(sn: GridSerial) -> (PowerLevel, Coord, Coord) {
    let power_levels = PowerLevels::new(sn);
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

pub(crate) fn largest_total_power_variable_size_square(
    sn: GridSerial,
) -> (PowerLevel, Coord, Coord, usize) {
    let power_levels = PowerLevels::new(sn);
    let three_hundred = (1..=300).into_iter().collect::<Vec<Coord>>();
    let mut max = (0, 0, 0, 0);
    for size in SIZE_RANGE {
        max = max.max(
            three_hundred
                .par_windows(size)
                .map(|x| {
                    three_hundred
                        .windows(size)
                        .map(|y| (power_levels.total_power(x, y), x[0], y[0], size))
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap(),
        );
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_power_levels() {
        assert_eq!(4, PowerLevels::power_level(&3, &5, &8));
        assert_eq!(-5, PowerLevels::power_level(&122, &79, &57));
        assert_eq!(0, PowerLevels::power_level(&217, &196, &39));
        assert_eq!(4, PowerLevels::power_level(&101, &153, &71));
    }

    #[test]
    fn top_left_coord_of_largest_total_power_3x3_square_examples() {
        assert_eq!((29, 33, 45), largest_total_power_3x3_square(18));
        assert_eq!((30, 21, 61), largest_total_power_3x3_square(42));
    }

    #[test]
    fn part1() {
        assert_eq!((28, 235, 87), largest_total_power_3x3_square(8199));
    }

    // #[test] // Slow, ~5 minutes
    fn largest_total_power_variable_size_square_example_1() {
        assert_eq!(
            (113, 90, 269, 16),
            largest_total_power_variable_size_square(18)
        );
    }

    // #[test] // Slow, ~5 minutes
    fn largest_total_power_variable_size_square_example_2() {
        assert_eq!(
            (119, 232, 251, 12),
            largest_total_power_variable_size_square(42)
        );
    }

    // #[test] // Slow, ~5 minutes
    fn part2() {
        assert_eq!(
            (119, 234, 272, 18),
            largest_total_power_variable_size_square(8199)
        );
    }
}
