fn part1impl(input: &[String]) -> usize {
    let earliest_time: usize = input[0].parse().expect("a number");
    let bus_ids = input[1]
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect();
    println!("{}: {:?}", earliest_time, bus_ids);
    shortest_wait_multiplied_by_bus_id(earliest_time, bus_ids)
}

fn shortest_wait_multiplied_by_bus_id(earliest_time: usize, bus_ids: Vec<usize>) -> usize {
    let (min, id) = bus_ids
        .iter()
        .map(|id| {
            let dividend = earliest_time / id;
            if earliest_time % dividend == 0 {
                (0, id)
            } else {
                ((dividend + 1) * id - earliest_time, id)
            }
        })
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .unwrap();
    min * id
}

fn part2impl(input: &[String]) -> usize {
    let mut offset_and_frequency: Vec<(usize, usize)> = input[1]
        .split(',')
        .enumerate()
        .filter_map(|(idx, id)| {
            if let Ok(id) = id.parse() {
                Some((idx, id))
            } else {
                None
            }
        })
        .collect();
    let base = offset_and_frequency.iter().map(|(_, freq)| *freq).collect();
    let offset = offset_and_frequency
        .iter()
        .map(|(offset, _)| *offset)
        .collect();
    find_factors(base, offset).1
}

/// Find factors a and b where a * first + offset = b * second
fn base_factors_for_offset(first: isize, second: isize, offset: isize) -> (isize, isize) {
    let mut a = 1;
    let mut b = 1;
    while a * first + offset != b * second {
        if a * first + offset < b * second {
            a += 1;
        } else {
            b += 1;
        }
    }
    (a, b)
}

fn find_factors(base: Vec<usize>, offset: Vec<usize>) -> (Vec<usize>, usize) {
    assert_eq!(base.len(), offset.len());
    let len = base.len();
    let mut factor = vec![1; len];
    let mut value = vec![0; len];
    for i in 0..len {
        while offset[i] > factor[i] * base[i] {
            println!(
                "factor * base < offset: {} * {} < {}",
                factor[i], base[i], offset[i]
            );
            factor[i] += 1;
        }
        value[i] = factor[i] * base[i] - offset[i];
    }
    while !value.iter().all(|v| v == &value[0]) {
        // increase the factor of the smallest value and check again
        if let Some((i, _)) = value
            .iter()
            .enumerate()
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        {
            factor[i] += 1;
            value[i] = factor[i] * base[i] - offset[i];
        }
    }
    (factor, value[0])
}
fn find_factors_with_positive_offset(
    base0: usize,
    offset0: usize,
    base1: usize,
    offset1: usize,
    base2: usize,
    offset2: usize,
) -> (usize, usize, usize) {
    const LEN: usize = 3;
    let mut factor = [1; LEN];
    let base = [base0, base1, base2];
    let offset = [offset0, offset1, offset2];
    let mut value: [usize; LEN] = [0; LEN];
    for i in 0..LEN {
        value[i] = factor[i] * base[i] + offset[i];
    }
    while value[0] != value[1] || value[1] != value[2] {
        // increase the factor of the smallest value and check again
        if let Some((i, _)) = value
            .iter()
            .enumerate()
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        {
            factor[i] += 1;
            value[i] = factor[i] * base[i] + offset[i];
        }
    }
    (factor[0], factor[1], factor[2])
}

#[cfg(test)]
mod tests {
    use crate::{
        base_factors_for_offset, find_factors, find_factors_with_positive_offset, part1impl,
        part2impl,
    };
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn part1_example() {
        assert_eq!(part1impl(&read_str_to_lines(EXAMPLE)), 295);
    }

    #[test]
    fn part1() {
        assert_eq!(part1impl(&read_file_to_lines("input.txt")), 3269);
    }

    #[test]
    fn part2_base_factors_for_offset_with_example1() {
        assert_eq!(base_factors_for_offset(7, 13, 1), (11, 6));
        assert_eq!(base_factors_for_offset(7, 59, 4), (50, 6));
        assert_eq!(base_factors_for_offset(7, 31, 6), (8, 2));
        assert_eq!(base_factors_for_offset(7, 19, 7), (18, 7));
        //
        assert_eq!(base_factors_for_offset(13, 59, -39), (62, 13));
        assert_eq!(base_factors_for_offset(13, 31, 3), (26, 11));
        assert_eq!(base_factors_for_offset(13, 19, -7), (2, 1));
        //
        assert_eq!(base_factors_for_offset(59, 31, -42), (17, 31));
        assert_eq!(base_factors_for_offset(59, 19, -32), (16, 48));
        //
        assert_eq!(base_factors_for_offset(31, 19, 10), (15, 25));
    }

    #[test]
    fn part2_base_factors_for_offset_with_example2() {
        assert_eq!(base_factors_for_offset(17, 13, 2), (6, 8));
        assert_eq!(base_factors_for_offset(17, 19, 3), (11, 10));
        //
        assert_eq!(base_factors_for_offset(13, 19, -5), (15, 10));
        // Solve example 2 directly
        assert_eq!(
            find_factors(vec![17, 13, 19], vec![0, 2, 3]),
            (vec![201, 263, 180], 3417)
        );
    }

    #[test]
    fn part2_base_factors_for_offset_with_example3() {
        assert_eq!(base_factors_for_offset(67, 7, 1), (5, 48));
        assert_eq!(base_factors_for_offset(67, 59, 2), (44, 50));
        assert_eq!(base_factors_for_offset(67, 61, 3), (30, 33));
        //
        assert_eq!(base_factors_for_offset(7, 59, 5 - 44), (14, 1));
        assert_eq!(base_factors_for_offset(59, 61, 44 - 30), (7, 7));
        //
        assert_eq!(base_factors_for_offset(7, 61, 5 - 30), (21, 2));
        // solves second half of example 3
        assert_eq!(
            find_factors_with_positive_offset(7, 5, 59, 44, 61, 30),
            (1607, 190, 184)
        );
        // Solve example 3 directly
        assert_eq!(
            find_factors(vec![67, 7, 59, 61], vec![0, 1, 2, 3]),
            (vec![11254, 107717, 12780, 12361], 754018)
        );
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2impl(&read_str_to_lines(EXAMPLE)), 1068781);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(
            part2impl(&read_str_to_lines(
                "whatever
17,x,13,19"
            )),
            3417
        );
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(
            part2impl(&read_str_to_lines(
                "whatever
67,7,59,61"
            )),
            754018
        );
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(
            part2impl(&read_str_to_lines(
                "whatever
67,x,7,59,61"
            )),
            779210
        );
    }

    #[test]
    fn part2_example_5() {
        assert_eq!(
            part2impl(&read_str_to_lines(
                "whatever
67,7,x,59,61"
            )),
            1261476
        );
    }

    // very slow, takes 29s!
    #[test]
    fn part2_example_6() {
        assert_eq!(
            part2impl(&read_str_to_lines(
                "whatever
1789,37,47,1889"
            )),
            1202161486
        );
    }

    // exceedingly slow…
    #[test]
    fn part2() {
        assert_eq!(part2impl(&read_file_to_lines("input.txt"),), 24769);
    }
}
