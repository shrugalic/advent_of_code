use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day15.txt");

pub(crate) fn day15_part1() -> usize {
    let mut scan = ScanResult::from(INPUT);
    scan.no_beacon_pos_count_at_y(2_000_000)
}

pub(crate) fn day15_part2() -> isize {
    let mut scan = ScanResult::from(INPUT);
    scan.tuning_frequency_of_distress_beacon(4_000_000)
}

type Coord = isize;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: Coord,
    y: Coord,
}
impl Pos {
    fn distance_to(&self, other: &Pos) -> usize {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Signal {
    sensor: Pos,
    beacon: Pos,
}
impl Signal {
    fn distance_to_beacon(&self) -> usize {
        self.sensor.distance_to(&self.beacon)
    }
    fn no_beacon_bounds_at_y(&self, y: Coord) -> Option<RangeInclusive<Coord>> {
        let bounds = self.no_other_beacon_boundaries();
        let lo = bounds.start();
        let hi = bounds.end();
        if (lo.y..=hi.y).contains(&y) {
            let d = (self.sensor.y - y).abs();
            Some((lo.x + d)..=(hi.x - d))
        } else {
            None
        }
    }
    fn no_other_beacon_boundaries(&self) -> RangeInclusive<Pos> {
        let d = self.distance_to_beacon() as isize;
        let x = self.sensor.x;
        let y = self.sensor.y;
        RangeInclusive::new(Pos { x: x - d, y: y - d }, Pos { x: x + d, y: y + d })
    }
}

#[derive(Debug)]
struct ScanResult {
    signals: Vec<Signal>,
    x_range: RangeInclusive<Coord>,
    y_range: RangeInclusive<Coord>,
}
impl ScanResult {
    fn no_beacon_pos_count_at_y(&mut self, y: isize) -> usize {
        let x_ranges = self.no_beacon_x_ranges_at_y(y);
        let x_min = *x_ranges.iter().map(|r| r.start()).min().unwrap();
        let x_max = *x_ranges.iter().map(|r| r.end()).max().unwrap();
        let not_in_any_range_count = (x_min..=x_max)
            .into_iter()
            .filter(|x| !x_ranges.iter().any(|range| range.contains(x)))
            .count();
        (x_max - x_min) as usize - not_in_any_range_count
    }
    fn no_beacon_x_ranges_at_y(&mut self, y: isize) -> Vec<RangeInclusive<Coord>> {
        self.signals
            .iter()
            .filter_map(|sig| sig.no_beacon_bounds_at_y(y))
            .collect()
    }
    fn tuning_frequency_of_distress_beacon(&mut self, xy_max: Coord) -> isize {
        'outer: for y in 0..=xy_max {
            let mut x_ranges = self.no_beacon_x_ranges_at_y(y);
            x_ranges.sort_unstable_by_key(|r| *r.start());
            let mut candidates = x_ranges.as_slice();
            let mut x = 0;
            while x <= xy_max {
                if let Some(idx) = candidates.iter().position(|r| r.start() > &x) {
                    let lower_ranges = &candidates[..idx];
                    candidates = &candidates[idx..];
                    if let Some(&max) = lower_ranges.iter().map(|r| r.end()).max() {
                        if max >= xy_max {
                            continue 'outer;
                        }
                        x = max + 1;
                        if !candidates.is_empty() && !candidates.iter().any(|r| r.contains(&x)) {
                            return x * 4_000_000 + y;
                        }
                    }
                } else {
                    continue 'outer;
                }
            }
        }
        unreachable!()
    }
}
impl From<&str> for ScanResult {
    fn from(input: &str) -> Self {
        let mut signals = vec![];
        let (mut min_x, mut max_x, mut min_y, mut max_y) =
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
        for (sensor, beacon) in input.lines().map(|line| {
            let line = line.strip_prefix("Sensor at x=").unwrap();
            let (sensor, beacon) = line.split_once(": closest beacon is at x=").unwrap();
            let (x, y) = sensor.split_once(", y=").unwrap();
            let sensor = Pos {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };
            let (x, y) = beacon.split_once(", y=").unwrap();
            let beacon = Pos {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };
            (sensor, beacon)
        }) {
            min_x = min_x.min(sensor.x);
            max_x = max_x.max(sensor.x);
            min_y = min_y.min(sensor.y);
            max_y = max_y.max(sensor.y);
            min_x = min_x.min(beacon.x);
            max_x = max_x.max(beacon.x);
            min_y = min_y.min(beacon.y);
            max_y = max_y.max(beacon.y);
            signals.push(Signal { sensor, beacon });
        }
        ScanResult {
            signals,
            x_range: (min_x - 1)..=(max_x + 1),
            y_range: 0..=max_y,
        }
    }
}
impl Display for ScanResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.y_range
                .clone()
                .into_iter()
                .map(|y| self
                    .x_range
                    .clone()
                    .into_iter()
                    .map(|x| self
                        .signals
                        .iter()
                        .find_map(|s| {
                            let pos = Pos { x, y };
                            if s.sensor == pos {
                                Some('S')
                            } else if s.beacon == pos {
                                Some('B')
                            } else {
                                None
                            }
                        })
                        .unwrap_or('.'))
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_example() {
        let mut scan = ScanResult::from(EXAMPLE);
        // println!("{}", scan);
        assert_eq!(26, scan.no_beacon_pos_count_at_y(10));
    }

    #[test]
    fn part1() {
        assert_eq!(5_256_611, day15_part1()); // 5256610 is too low
    }

    #[test]
    fn part2_example() {
        let mut scan = ScanResult::from(EXAMPLE);
        assert_eq!(56_000_011, scan.tuning_frequency_of_distress_beacon(20));
    }

    // Slow at 14s in debug (< 0.7s in release)
    #[test]
    fn part2() {
        assert_eq!(13_337_919_186_981, day15_part2());
    }
}
