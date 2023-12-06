const INPUT: &str = include_str!("../input/day06.txt");

pub(crate) fn part1() -> usize {
    product_of_ways_to_go_farther_in_the_same_time(INPUT)
}

pub(crate) fn part2() -> usize {
    count_ways_to_go_farther_in_the_same_time(INPUT)
}

fn product_of_ways_to_go_farther_in_the_same_time(input: &str) -> usize {
    parse(input)
        .iter()
        .map(Race::count_ways_to_go_farther_in_the_same_time)
        .product()
}

fn count_ways_to_go_farther_in_the_same_time(input: &str) -> usize {
    let races = parse(input);
    let concatenate =
        |left: usize, right: usize| left * 10usize.pow(right.to_string().len() as u32) + right;
    let race = races.into_iter().fold(Race::default(), |total, race| {
        let time = concatenate(total.time, race.time);
        let distance = concatenate(total.distance, race.distance);
        Race { time, distance }
    });
    race.count_ways_to_go_farther_in_the_same_time()
}

fn parse(input: &str) -> Vec<Race> {
    let (times, distances) = input.trim().split_once('\n').expect("2 lines");
    let times: Vec<_> = times
        .strip_prefix("Time:")
        .expect("'Time:' prefix")
        .trim()
        .split_ascii_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();
    let distances: Vec<_> = distances
        .strip_prefix("Distance:")
        .expect("'Distance:' prefix")
        .trim()
        .split_ascii_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

#[derive(PartialEq, Debug, Default)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn count_ways_to_go_farther_in_the_same_time(&self) -> usize {
        (1usize..self.time)
            .map(|time_and_speed| (self.time - time_and_speed) * time_and_speed)
            .filter(|distance| distance > &self.distance)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part1_example() {
        assert_eq!(288, product_of_ways_to_go_farther_in_the_same_time(EXAMPLE));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(71_503, count_ways_to_go_farther_in_the_same_time(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            1_108_800,
            product_of_ways_to_go_farther_in_the_same_time(INPUT)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(36_919_753, count_ways_to_go_farther_in_the_same_time(INPUT));
    }
}
