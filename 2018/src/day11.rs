type Coord = isize;
type GridSerial = isize;
type PowerLevel = isize;

pub(crate) fn largest_total_power_3x3_square(sn: GridSerial) -> (PowerLevel, Coord, Coord) {
    let three_hundred: Vec<Coord> = (1..=300).into_iter().collect();
    three_hundred
        .windows(3)
        .flat_map(|x| {
            three_hundred.windows(3).map(move |y| {
                let power_level = power_level(x[0], y[0], sn)
                    + power_level(x[0], y[1], sn)
                    + power_level(x[0], y[2], sn)
                    + power_level(x[1], y[0], sn)
                    + power_level(x[1], y[1], sn)
                    + power_level(x[1], y[2], sn)
                    + power_level(x[2], y[0], sn)
                    + power_level(x[2], y[1], sn)
                    + power_level(x[2], y[2], sn);
                (power_level, x[0], y[0])
            })
        })
        .max()
        .unwrap()
}

pub(crate) fn largest_total_power_variable_size_square(
    sn: GridSerial,
) -> (PowerLevel, Coord, Coord) {
    let three_hundred = (1..=300).into_iter().collect::<Vec<Coord>>();
    let mut max = (0, 0, 0);
    for size in 1..=300 {
        max = max.max(three_hundred
            .windows(size)
            .flat_map(|x| {
                three_hundred.windows(size).map(move |y| {
                    let power_level = power_level(x[0], y[0], sn) + power_level(x[0], y[1], sn);
                    (power_level, x[0].clone(), y[0].clone())
                })
            })
            .max()
            .unwrap());
    }
    (0, 0, 0)
}

fn power_level(x: Coord, y: Coord, serial: GridSerial) -> PowerLevel {
    let rack_id = x + 10;
    (rack_id * y + serial) * rack_id / 100 % 10 - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_power_levels() {
        assert_eq!(4, power_level(3, 5, 8));
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }

    #[test]
    fn top_left_coord_of_largest_total_power_square_examples() {
        assert_eq!((29, 33, 45), largest_total_power_3x3_square(18));
        assert_eq!((30, 21, 61), largest_total_power_3x3_square(42));
    }

    #[test]
    fn part1() {
        assert_eq!((28, 235, 87), largest_total_power_3x3_square(8199));
    }
}
