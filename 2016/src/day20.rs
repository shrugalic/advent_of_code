use line_reader::read_file_to_lines;
use std::collections::VecDeque;
use std::ops::RangeInclusive;

const MAX_IP: usize = 4_294_967_295;

pub(crate) fn day20_part1() -> usize {
    let blacklist: Vec<IpRange> = parse_rules(read_file_to_lines("input/day20.txt"));
    lowest_valued_non_forbidden_ip(blacklist)
}

pub(crate) fn day20_part2() -> usize {
    let blacklist: Vec<IpRange> = parse_rules(read_file_to_lines("input/day20.txt"));
    number_of_allowed_ips(blacklist)
}

type IpRange = RangeInclusive<usize>;
fn lowest_valued_non_forbidden_ip(blacklist: Vec<IpRange>) -> usize {
    let mut candidates: Vec<_> = blacklist.iter().map(|r| r.end() + 1).collect();
    candidates.sort_unstable();
    for candidate in candidates {
        if blacklist.iter().all(|range| !range.contains(&candidate)) {
            return candidate;
        }
    }
    unreachable!()
}

fn parse_rules(input: Vec<String>) -> Vec<IpRange> {
    input
        .into_iter()
        .map(|s| {
            let (from, to) = s.split_once('-').unwrap();
            from.parse().unwrap()..=to.parse().unwrap()
        })
        .collect()
}

fn number_of_allowed_ips(blacklist: Vec<IpRange>) -> usize {
    MAX_IP + 1 - blocked_ip_count(blacklist)
}

fn blocked_ip_count(blacklist: Vec<IpRange>) -> usize {
    let non_overlapping_ranges = merge_overlapping_ranges(blacklist);
    non_overlapping_ranges
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

fn merge_overlapping_ranges(mut blacklist: Vec<IpRange>) -> Vec<IpRange> {
    blacklist.sort_unstable_by_key(|r| *r.start());
    let mut blacklist: VecDeque<IpRange> = VecDeque::from(blacklist);
    let mut merged = vec![];
    while let Some(mut range) = blacklist.pop_front() {
        while let Some(pos) = blacklist
            .iter()
            .position(|other| other.start() <= range.end())
        {
            let other = blacklist.remove(pos).unwrap();
            if other.end() > range.end() {
                range = *range.start()..=*other.end();
            }
        }
        merged.push(range);
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
5-8
0-2
4-7";

    #[test]
    fn part1_example() {
        let blacklist: Vec<IpRange> = parse_rules(read_str_to_lines(EXAMPLE));
        assert_eq!(3, lowest_valued_non_forbidden_ip(blacklist));
    }

    #[test]
    fn part1() {
        assert_eq!(22_887_907, day20_part1());
    }

    #[test]
    fn part2_example() {
        let blacklist: Vec<IpRange> = parse_rules(read_str_to_lines(EXAMPLE));
        assert_eq!(MAX_IP + 1 - 8, number_of_allowed_ips(blacklist));
    }

    #[test]
    fn part2() {
        assert_eq!(109, day20_part2());
    }
}
