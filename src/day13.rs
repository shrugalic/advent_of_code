use line_reader::read_file_to_lines;
use std::collections::VecDeque;

pub(crate) fn day13_part1() -> usize {
    trip_severity(read_file_to_lines("input/day13.txt"))
}

pub(crate) fn day13_part2() -> usize {
    delay_to_get_through_safely(read_file_to_lines("input/day13.txt"))
}

#[derive(Clone, Debug)]
struct Firewall {
    range: usize,
    scanner: usize,
    is_scanning_down: bool,
}

impl Firewall {
    fn is_scanning_top(&self) -> bool {
        self.scanner == 0
    }
}

fn trip_severity(input: Vec<String>) -> usize {
    let mut firewalls = parse_firewalls(input);

    let mut severity = 0;
    for depth in 0..firewalls.len() {
        // Move packet along the top layer, and sum up severity if it's scanned
        // by the scanner of the optional firewall at the current depth
        if let Some(firewall) = firewalls.get(depth).unwrap() {
            if firewall.is_scanning_top() {
                severity += depth * firewall.range;
            }
        }
        firewalls
            .iter_mut()
            .filter_map(|f| f.as_mut())
            .for_each(|firewall| {
                if firewall.is_scanning_down {
                    firewall.scanner += 1;
                    if firewall.scanner == firewall.range - 1 {
                        firewall.is_scanning_down = false;
                    }
                } else {
                    firewall.scanner -= 1;
                    if firewall.scanner == 0 {
                        firewall.is_scanning_down = true;
                    }
                }
            });
    }

    severity
}

fn delay_to_get_through_safely(input: Vec<String>) -> usize {
    // We only care about firewalls when they're scanning the top layer, where packets travel
    // These are the periods where their scanner is at the top
    let firewall_periods: Vec<Option<usize>> = parse_firewalls(input)
        .into_iter()
        .map(|fw| fw.map(|firewall| (firewall.range - 1) * 2))
        .collect();

    let mut packet_depths: VecDeque<Option<usize>> = VecDeque::new();
    for starting_delay in 0..usize::MAX {
        // Add a new packet, which also moves the traveling packets along
        packet_depths.push_front(Some(starting_delay));

        // Check if a packet made it to the other side
        if let Some(Some(winners_delay)) = packet_depths.get(firewall_periods.len()) {
            return *winners_delay;
        }

        // Remove any packets scanned by a firewall
        for (depth, delay) in packet_depths
            .iter_mut()
            .enumerate()
            .filter(|(_depth, delay)| delay.is_some())
        {
            if let Some(period) = firewall_periods[depth] {
                if starting_delay % period == 0 {
                    *delay = None;
                }
            }
        }

        // Shorten the packet VecDeque as much as possible, so its size doesn't get out of hand.
        // This is still considerably faster than using a delay-by-index HashMap
        while let Some(None) = packet_depths.iter().last() {
            packet_depths.pop_back();
        }
    }

    unreachable!()
}

fn parse_firewalls(input: Vec<String>) -> Vec<Option<Firewall>> {
    let layer_count = parse_firewall(input.last().unwrap()).0 + 1;
    let mut firewalls = vec![None; layer_count];
    input
        .iter()
        .map(|s| parse_firewall(s))
        .for_each(|(depth, firewall)| firewalls[depth] = Some(firewall));

    firewalls
}

fn parse_firewall(line: &str) -> (usize, Firewall) {
    let (depth, range) = line.split_once(": ").unwrap();
    (
        depth.parse().unwrap(),
        Firewall {
            range: range.parse().unwrap(),
            scanner: 0,
            is_scanning_down: true,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE1: &str = "\
0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn part1_example() {
        assert_eq!(24, trip_severity(read_str_to_lines(EXAMPLE1)));
    }
    #[test]
    fn part1_full() {
        assert_eq!(748, day13_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(10, delay_to_get_through_safely(read_str_to_lines(EXAMPLE1)));
    }

    #[test]
    fn part2_full() {
        assert_eq!(3873662, day13_part2());
    }
}
