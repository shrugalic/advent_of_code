use crate::parse;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::{Add, Div, RangeInclusive};

const INPUT: &str = include_str!("../input/day23.txt");

pub(crate) fn day23_part1() -> usize {
    count_nanobots_in_signal_range(parse(INPUT))
}

pub(crate) fn day23_part2() -> usize {
    distance_to_origin_from_point_within_range_of_most_nanobots(parse(INPUT))
}

type Coord = isize;
type Radius = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Loc {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Add for Loc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Loc::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Div<Coord> for Loc {
    type Output = Loc;

    fn div(self, div: Coord) -> Self::Output {
        Loc::new(self.x / div, self.y / div, self.z / div)
    }
}

impl Loc {
    fn new(x: Coord, y: Coord, z: Coord) -> Self {
        Loc { x, y, z }
    }
    fn distance_to(&self, other: &Loc) -> usize {
        Loc::diff(self.x, other.x) + Loc::diff(self.y, other.y) + Loc::diff(self.z, other.z)
    }
    fn diff(a: Coord, b: Coord) -> usize {
        (a - b).abs() as usize
    }
    fn origin() -> Self {
        Loc { x: 0, y: 0, z: 0 }
    }
    fn offset_all(&self, offset: isize) -> Loc {
        Loc::new(self.x + offset, self.y + offset, self.z + offset)
    }
    fn offset(&self, x: Coord, y: Coord, z: Coord) -> Loc {
        Loc::new(self.x + x, self.y + y, self.z + z)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Nanobot {
    loc: Loc,
    radius: Radius,
}

impl Nanobot {
    fn reaches_loc(&self, loc: &Loc) -> bool {
        self.loc.distance_to(loc) <= self.radius
    }

    fn is_within_reach_of(&self, bound: &BoundingBox) -> bool {
        // The -1 is to not overlap the neighboring box
        let box_max = &bound.pos.offset_all(bound.size as isize - 1);
        // The code below works out the same as doing the following for each coordinate and summing up:
        // if bot > hi { bot - hi /* bot is above */ }
        // else if bot < lo { lo - bot /* bot is below */ }
        // else { 0 /* bot is within */ }
        let distance_from_bot_to_box = (self.loc.distance_to(box_max)
            + self.loc.distance_to(&bound.pos)
            - box_max.distance_to(&bound.pos))
            / 2;
        distance_from_bot_to_box <= self.radius
    }
}

impl From<&str> for Nanobot {
    fn from(s: &str) -> Self {
        let (pos, r) = s.split_once(">, r=").unwrap();
        let pos: Vec<Coord> = pos
            .trim_start_matches("pos=<")
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let loc = Loc::new(pos[0], pos[1], pos[2]);
        let radius = r.parse().unwrap();
        Nanobot { loc, radius }
    }
}

pub(crate) fn count_nanobots_in_signal_range(input: Vec<&str>) -> usize {
    let nanobots: Vec<Nanobot> = input.into_iter().map(Nanobot::from).collect();
    let signal = nanobots.iter().max_by_key(|n| n.radius).unwrap();

    nanobots
        .iter()
        .filter(|bot| signal.reaches_loc(&bot.loc))
        .count()
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct BoundingBox {
    pos: Loc,
    size: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Candidate {
    bound: BoundingBox,
    bot_count: usize,
}

impl Candidate {
    fn distance_to_origin(&self) -> usize {
        self.bound.pos.distance_to(&Loc::origin())
    }
}
impl PartialOrd<Self> for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by larger bot count…
        match self.bot_count.cmp(&other.bot_count) {
            // then by larger bounding box…
            Ordering::Equal => match self.bound.size.cmp(&other.bound.size) {
                // then by closeness to origin
                Ordering::Equal => self
                    .distance_to_origin()
                    .cmp(&other.distance_to_origin())
                    .reverse(),
                large_size => large_size,
            },
            large_bot_count => large_bot_count,
        }
    }
}

pub(crate) fn distance_to_origin_from_point_within_range_of_most_nanobots(
    input: Vec<&str>,
) -> usize {
    let bots: Vec<Nanobot> = input.into_iter().map(Nanobot::from).collect();

    let bound = initial_box_around_all_bots(&bots);
    let bot_count = count_bots_in_range_of_box(&bots, &bound);

    let mut queue = BinaryHeap::new();
    queue.push(Candidate { bound, bot_count });

    while let Some(candidate) = queue.pop() {
        if candidate.bound.size == 1 {
            // Because the heap is ordered by maximum number of bots, then bounding box size,
            // then closeness to origin, we're done once we reach this smallest size
            return candidate.bound.pos.distance_to(&Loc::origin());
        }

        let size = candidate.bound.size / 2;
        octants(&candidate.bound.pos, size as isize)
            .into_iter()
            .for_each(|pos| {
                let bound = BoundingBox { pos, size };
                let bot_count = count_bots_in_range_of_box(&bots, &bound);
                queue.push(Candidate { bound, bot_count });
            });
    }
    unreachable!()
}

// Positions of the 8 smaller cubes dividing the previous larger cube (2 * 2 * 2)
fn octants(pos: &Loc, size: isize) -> Vec<Loc> {
    [
        (0, 0, 0),
        (0, 0, 1),
        (0, 1, 0),
        (0, 1, 1),
        (1, 0, 0),
        (1, 0, 1),
        (1, 1, 0),
        (1, 1, 1),
    ]
    .iter()
    .map(|(x, y, z)| pos.offset(*x * size, *y * size, *z * size))
    .collect::<Vec<_>>()
}

fn initial_box_around_all_bots(bots: &[Nanobot]) -> BoundingBox {
    let (box_lo, box_hi) = min_max(bots);
    let longest_side = longest_side(box_lo, box_hi);
    let size = 2usize.pow((longest_side as f64).log2().ceil() as u32);
    BoundingBox { pos: box_lo, size }
}

fn longest_side(min: Loc, max: Loc) -> isize {
    let w = (max.x - min.x).abs() + 1;
    let h = (max.y - min.y).abs() + 1;
    let d = (max.z - min.z).abs() + 1;
    // println!("Size: w * h * d = {} * {} * {}", w, h, d);
    *[w, h, d].iter().max().unwrap()
}

fn min_max(bots: &[Nanobot]) -> (Loc, Loc) {
    let x = range_of_center(bots, &|bot| bot.loc.x);
    let y = range_of_center(bots, &|bot| bot.loc.y);
    let z = range_of_center(bots, &|bot| bot.loc.z);
    let min = Loc::new(*x.start(), *y.start(), *z.start());
    let max = Loc::new(*x.end(), *y.end(), *z.end());
    (min, max)
}

fn count_bots_in_range_of_box(bots: &[Nanobot], bound: &BoundingBox) -> usize {
    bots.iter()
        .filter(|bot| bot.is_within_reach_of(bound))
        .count()
}

fn range_of_center(
    bots: &[Nanobot],
    center_coord_choice: &dyn Fn(&Nanobot) -> Coord,
) -> RangeInclusive<isize> {
    bots.iter().map(|b| center_coord_choice(b)).min().unwrap()
        ..=bots.iter().map(|b| center_coord_choice(b)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE_1: &str = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";

    const EXAMPLE_2: &str = "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";

    #[test]
    fn part1_example_count_nanobots_in_signal_range() {
        assert_eq!(7, count_nanobots_in_signal_range(parse(EXAMPLE_1)));
    }

    #[test]
    fn part1_count_nanobots_in_signal_range() {
        assert_eq!(417, day23_part1());
    }

    #[test]
    fn part2_example_distance_to_origin_from_point_withing_range_of_most_nanobots() {
        assert_eq!(
            36,
            distance_to_origin_from_point_within_range_of_most_nanobots(parse(EXAMPLE_2))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(112_997_634, day23_part2());
    }
}
