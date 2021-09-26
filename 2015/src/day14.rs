use line_reader::read_file_to_lines;

pub(crate) fn day14_part1() -> usize {
    let input = read_file_to_lines("input/day14.txt");
    max_distance_after(2503, input)
}

pub(crate) fn day14_part2() -> usize {
    let input = read_file_to_lines("input/day14.txt");
    max_points_after(2503, input)
}

fn max_distance_after(duration: usize, input: Vec<String>) -> usize {
    parse_reindeer(input)
        .into_iter()
        .map(|r| r.distance_after(duration))
        .max()
        .unwrap()
}

fn max_points_after(duration: usize, input: Vec<String>) -> usize {
    let reindeer = parse_reindeer(input);
    let mut time = 0;
    let mut distances = vec![0; reindeer.len()];
    let mut points = vec![0; reindeer.len()];
    while time < duration {
        for (idx, reindeer) in reindeer.iter().enumerate() {
            if time % reindeer.cycle_duration() < reindeer.fly_duration {
                distances[idx] += reindeer.speed;
            }
        }
        let max_distance = distances.iter().max().unwrap();
        distances
            .iter()
            .enumerate()
            .filter(|(_, dist)| dist == &max_distance)
            .for_each(|(idx, _)| points[idx] += 1);
        time += 1;
    }
    *points.iter().max().unwrap()
}

fn parse_reindeer(input: Vec<String>) -> Vec<Reindeer> {
    let reindeer: Vec<_> = input.iter().map(Reindeer::from).collect();
    reindeer
}

#[derive(Debug)]
struct Reindeer {
    speed: usize,
    fly_duration: usize,
    rest_duration: usize,
}
impl<T: AsRef<str>> From<T> for Reindeer {
    fn from(s: T) -> Self {
        let parts: Vec<_> = s.as_ref().split_ascii_whitespace().collect();
        let speed = parts[3].parse().unwrap();
        let fly_duration = parts[6].parse().unwrap();
        let rest_duration = parts[13].parse().unwrap();
        Reindeer {
            speed,
            fly_duration,
            rest_duration,
        }
    }
}

impl Reindeer {
    fn distance_after(&self, duration: usize) -> usize {
        let cycle_duration = self.cycle_duration();
        let mut time = 0;
        let mut distance = 0;
        while time < duration {
            if time % cycle_duration < self.fly_duration {
                distance += self.speed;
            }
            time += 1;
        }
        distance
    }

    fn cycle_duration(&self) -> usize {
        self.fly_duration + self.rest_duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn part1_example() {
        let input = read_str_to_lines(EXAMPLE);
        assert_eq!(1120, max_distance_after(1000, input));
    }
    #[test]
    fn part1() {
        assert_eq!(2655, day14_part1());
    }

    #[test]
    fn part2_example() {
        let input = read_str_to_lines(EXAMPLE);
        assert_eq!(689, max_points_after(1000, input));
    }

    #[test]
    fn part2() {
        assert_eq!(1059, day14_part2());
    }
}
