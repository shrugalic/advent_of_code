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

/// find factors f[i] such that all values are the same, where value[i] = f[i] * base[i] - offset[i]
fn find_factors(base: Vec<usize>, offset: Vec<isize>) -> (Vec<usize>, usize) {
    assert_eq!(base.len(), offset.len());
    let len = base.len();
    let mut factor = vec![1; len];
    let mut value = vec![0; len];
    for i in 0..len {
        // Make sure value never becomes negative (stays usize)
        while offset[i] > (factor[i] * base[i]) as isize {
            // println!(
            //     "factor * base < offset: {} * {} < {}",
            //     factor[i], base[i], offset[i]
            // );
            factor[i] += 1;
        }
        value[i] = (((factor[i] * base[i]) as isize) - offset[i]) as usize;
    }
    while !value.iter().all(|v| v == &value[0]) {
        // increase the factor of the smallest value and check again
        let (index_of_min, _) = value
            .iter()
            .enumerate()
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        factor[index_of_min] += 1;
        value[index_of_min] = (((factor[index_of_min] * base[index_of_min]) as isize)
            - offset[index_of_min]) as usize;
    }
    (factor, value[0])
}
fn get_offsets_and_bases_as_separate_vecs(input: &[String]) -> (Vec<usize>, Vec<usize>) {
    input[1]
        .split(',')
        .enumerate()
        .filter_map(|(idx, id)| {
            // get rid of the 'x' entries
            if let Ok(id) = id.parse::<usize>() {
                Some((idx, id))
            } else {
                None
            }
        })
        .unzip()
}

/// find factors f[i] such that all values are the same, where value[i] = f[i] * base[i] + offset[i]
fn find_factors2(base: Vec<usize>, offset: Vec<usize>) -> (Vec<usize>, usize) {
    assert_eq!(base.len(), offset.len());
    let len = base.len();
    let mut factor = vec![1; len];
    let mut value = vec![0; len];
    for i in 0..len {
        value[i] = factor[i] * base[i] + offset[i];
    }
    while !value.iter().all(|v| v == &value[0]) {
        // increase the factor of the smallest value and check again
        let (index_of_min, _) = value
            .iter()
            .enumerate()
            .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();
        factor[index_of_min] += 1;
        value[index_of_min] = factor[index_of_min] * base[index_of_min] + offset[index_of_min];
    }
    (factor, value[0])
}

fn part2impl(input: &[String]) -> usize {
    let (offsets, freqs) = get_offsets_and_bases_as_separate_vecs(input);
    let mut bus_count = 2; // Start with two and add one more at a time
    let mut time = 0;
    while bus_count <= freqs.len() {
        time = find_meet_time(&freqs[0..bus_count], &offsets[0..bus_count], time);
        bus_count += 1;
    }
    time
}

/// Find integer factors f[i] for all `time + offset[i] = f[i] * freq[i]` equations
fn find_meet_time(freq: &[usize], offset: &[usize], start: usize) -> usize {
    assert_eq!(freq.len(), offset.len());
    let mut time = start;

    // We add elements one by one, and all but the last were already synced
    // If these weren't all primes, one could have to use their least common multiple
    let inc: usize = freq.iter().rev().skip(1).product();
    // Increase time until all equations hold
    while !freq
        .iter()
        .enumerate()
        .all(|(i, base)| (time + offset[i]) % base == 0)
    {
        time += inc;
    }
    println!(
        "Time {} = {} steps @ size {}",
        time,
        (time - start) / inc,
        inc
    );
    time
}

#[cfg(test)]
mod tests {
    use crate::{find_factors, find_factors2, find_meet_time, part1impl, part2impl};
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE1: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn part1_example() {
        assert_eq!(part1impl(&read_str_to_lines(EXAMPLE1)), 295);
    }

    #[test]
    fn part1() {
        assert_eq!(part1impl(&read_file_to_lines("input.txt")), 3269);
    }

    #[test]
    fn part2_example1_find_factors() {
        assert_eq!(find_factors(vec![7, 13], vec![0, 1]), (vec![11, 6], 77));
        assert_eq!(find_factors(vec!(7, 59), vec![0, 4]), (vec![50, 6], 350));
        assert_eq!(find_factors(vec!(7, 31), vec![0, 6]), (vec![8, 2], 56));
        assert_eq!(find_factors(vec!(7, 19), vec![0, 7]), (vec![18, 7], 126));
    }

    #[test]
    fn part2_example1_incremental_with_find_factors() {
        // Find the earliest time where the first two busses meet
        assert_eq!(find_meet_time(&[7, 13], &[0, 1], 0), 77);
        // From there, find where they meet the next bus. The step size is increased by the
        // common multiple of previous bus frequencies
        assert_eq!(find_meet_time(&[7, 13, 59], &[0, 1, 4], 77), 350);
        assert_eq!(find_meet_time(&[7, 13, 59, 31], &[0, 1, 4, 6], 350), 70147);
        assert_eq!(
            find_meet_time(&[7, 13, 59, 31, 19], &[0, 1, 4, 6, 7], 70147),
            1068781
        );
    }

    #[test]
    fn part2_example2_direct_solve() {
        assert_eq!(
            find_factors(vec![17, 13, 19], vec![0, 2, 3]),
            (vec![201, 263, 180], 3417)
        );
    }
    #[test]
    fn part2_example2_two_step_solve_with_find_factors() {
        // Find base factors for all combinations with first bus
        assert_eq!(find_factors(vec![17, 13], vec![0, 2]), (vec![6, 8], 102));
        assert_eq!(find_factors(vec![17, 19], vec![0, 3]), (vec![11, 10], 187));
        // Solve resulting equations
        assert_eq!(
            find_factors(vec![13, 19], vec![-6, -11]),
            (vec![15, 10], 201) // 201 * 17 = 3_417, the solution
        );
        // Same but with positive offsets using find_factors2
        assert_eq!(
            find_factors2(vec![13, 19], vec![6, 11]),
            (vec![15, 10], 201) // 201 * 17 = 3_417, the solution
        );
    }
    #[test]
    fn part2_example2_two_step_solve_with_find_factors2() {
        // Same, but starting from last bus, to have positive offsets, so find_factors2 can be used.
        // The resulting time will be off by the offset of the last bus.
        // Find base factors for all combinations with last bus
        assert_eq!(find_factors2(vec![19, 17], vec![0, 3]), (vec![10, 11], 190));
        assert_eq!(find_factors2(vec![19, 13], vec![0, 1]), (vec![11, 16], 209));
        // Solve resulting equations
        assert_eq!(
            find_factors2(vec![17, 13], vec![10, 11]),
            (vec![10, 13], 180) // 180 * 19 − 3 = 3_417, the solution
        );
    }

    #[test]
    fn part2_example3_direct_solve() {
        // Solve example 3 directly
        assert_eq!(
            find_factors(vec![67, 7, 59, 61], vec![0, 1, 2, 3]),
            (vec![11254, 107717, 12780, 12361], 754_018)
        );
    }
    #[test]
    fn part2_example3_two_step_solve_with_find_factors() {
        // Find base factors for all combinations with first bus
        assert_eq!(find_factors(vec![67, 7], vec![0, 1]), (vec![5, 48], 335));
        assert_eq!(find_factors(vec![67, 59], vec![0, 2]), (vec![44, 50], 2948));
        assert_eq!(find_factors(vec![67, 61], vec![0, 3]), (vec![30, 33], 2010));
        // Solve resulting equations
        assert_eq!(
            find_factors(vec![7, 59, 61], vec![-5, -44, -30]),
            (vec![1607, 190, 184], 11254) // 11_254 * 67 = 754_018, the solution
        );
        // Same but with positive offsets using find_factors2
        assert_eq!(
            find_factors2(vec![7, 59, 61], vec![5, 44, 30]),
            (vec![1607, 190, 184], 11254) // 11_254 * 67 = 754_018, the solution
        );
    }
    #[test]
    fn part2_example3_two_step_solve_with_find_factors2() {
        // Find base factors for all combinations with first bus
        assert_eq!(
            find_factors2(vec![61, 67], vec![0, 3]),
            (vec![33, 30], 2013)
        );
        assert_eq!(find_factors2(vec![61, 7], vec![0, 2]), (vec![6, 52], 366));
        assert_eq!(
            find_factors2(vec![61, 59], vec![0, 1]),
            (vec![30, 31], 1830)
        );
        // Solve resulting equations
        assert_eq!(
            find_factors2(vec![67, 7, 59], vec![33, 6, 30]),
            (vec![184, 1765, 209], 12361) // 12361 * 61 - 3 = 754_018, the solution
        );
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2impl(&read_str_to_lines(EXAMPLE1)), 1068781);
    }

    const EXAMPLE2: &str = "whatever
17,x,13,19";
    #[test]
    fn part2_example_2() {
        assert_eq!(part2impl(&read_str_to_lines(EXAMPLE2)), 3417);
    }

    const EXAMPLE3: &str = "whatever
67,7,59,61";
    #[test]
    fn part2_example_3() {
        assert_eq!(part2impl(&read_str_to_lines(EXAMPLE3)), 754018);
    }

    const EXAMPLE4: &str = "whatever
67,x,7,59,61";
    #[test]
    fn part2_example_4() {
        assert_eq!(part2impl(&read_str_to_lines(EXAMPLE4)), 779210);
    }

    const EXAMPLE5: &str = "whatever
67,7,x,59,61";
    #[test]
    fn part2_example_5() {
        assert_eq!(part2impl(&read_str_to_lines(EXAMPLE5)), 1261476);
    }

    const EXAMPLE6: &str = "whatever
1789,37,47,1889";
    // takes 29s! with part2impl
    // takes 15ms with part2impl2
    #[test]
    fn part2_example_6() {
        assert_eq!(part2impl(&read_str_to_lines(EXAMPLE6)), 1202161486);
    }

    // exceedingly slow…
    #[test]
    fn part2() {
        assert_eq!(
            part2impl(&read_file_to_lines("input.txt"),),
            672754131923874
        );
    }
}
