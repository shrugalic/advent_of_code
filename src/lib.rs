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

#[cfg(test)]
mod tests {
    use crate::part1impl;
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
}
