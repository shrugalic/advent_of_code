use line_reader::read_file_to_lines;

pub(crate) fn day15_part1() -> usize {
    let input = read_file_to_lines("input/day15.txt");
    let discs = discs_from(input);
    earliest_start_time_with_full_alignment(discs)
}

pub(crate) fn day15_part2() -> usize {
    let input = read_file_to_lines("input/day15.txt");
    let mut discs = discs_from(input);
    let period = 11;
    let t0pos = 0;
    discs.push(Disc {
        period,
        t0pos,
        time: Disc::first_pass_through_start_time(discs.len() + 1, t0pos, period),
    });
    earliest_start_time_with_full_alignment(discs)
}

fn earliest_start_time_with_full_alignment(discs: Vec<Disc>) -> usize {
    // The disc with the most positions takes the longest to re-align,
    // it's pointless to try more often than this slowest rotation period
    let (mut slowest, mut others) = split(discs);
    while !slowest.is_aligned_with(&others) {
        slowest.rotate_once();
        for disc in others.iter_mut() {
            while disc.time < slowest.time {
                disc.rotate_once();
            }
        }
    }
    slowest.time
}

fn split(mut discs: Vec<Disc>) -> (Disc, Vec<Disc>) {
    discs.sort_unstable_by_key(|disc| disc.period);
    let slowest = discs.pop().unwrap();
    (slowest, discs)
}

fn discs_from(input: Vec<String>) -> Vec<Disc> {
    input.into_iter().map(Disc::from).collect()
}

#[derive(Debug)]
struct Disc {
    period: usize,
    t0pos: usize,
    time: usize,
}
impl From<String> for Disc {
    fn from(s: String) -> Self {
        // Disc #1 has 13 positions; at time=0, it is at position 1.
        let s: Vec<_> = s.split(|c| c == ' ' || c == '#' || c == '.').collect();
        let number = s[2].parse().unwrap();
        let period = s[4].parse().unwrap();
        let t0pos = s[12].parse().unwrap();
        let time = Disc::first_pass_through_start_time(number, t0pos, period);
        Disc {
            period,
            t0pos,
            time,
        }
    }
}
impl Disc {
    fn first_pass_through_start_time(number: usize, t0pos: usize, period: usize) -> usize {
        // It takes ${number} of seconds to reach this disc
        let mut t = number;
        // After this, a capsule can pass through every ${period} seconds
        while (t0pos + t) % period != 0 {
            t += 1;
        }
        // Subtract ${number} seconds to get the start time at the top
        t - number
    }
    fn rotate_once(&mut self) {
        self.time += self.period;
    }
    fn is_aligned_with(&self, others: &[Disc]) -> bool {
        others.iter().all(|disc| disc.time == self.time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn part1_example() {
        let input = read_str_to_lines(EXAMPLE);
        let discs = discs_from(input);
        assert_eq!(5, earliest_start_time_with_full_alignment(discs));
    }

    #[test]
    fn part1() {
        assert_eq!(376_777, day15_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(3_903_937, day15_part2());
    }
}
