use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub(crate) fn day22_part1() -> RiskLevel {
    full_cave().risk_level()
}

pub(crate) fn day22_part2() -> usize {
    full_cave().shortest_path_len()
}

const DEPTH: usize = 3066;
const TARGET: (Coord, Coord) = (13, 726);

const X_MULTI: usize = 16807;
const Y_MULTI: usize = 48271;
const MODULO: usize = 20183;

type Coord = usize;
type X = Coord;
type Y = Coord;
type Depth = usize;
type GeologicIndex = usize;
type ErosionLevel = usize;
type RiskLevel = usize;
type Time = usize;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Loc {
    x: X,
    y: Y,
}

impl Loc {
    pub(crate) fn hamming_distance_to(&self, other: &Loc) -> usize {
        Loc::diff(self.x, other.x) + Loc::diff(self.y, other.y)
    }
    fn diff(a: usize, b: usize) -> usize {
        if a < b {
            b - a
        } else {
            a - b
        }
    }
}

impl Loc {
    fn new(x: X, y: Y) -> Self {
        Loc { x, y }
    }
    #[allow(dead_code)]
    fn equals(&self, x: X, y: Y) -> bool {
        self.x == x && self.y == y
    }
    fn offset_by(&self, x: isize, y: isize) -> Self {
        Loc::new(
            (self.x as isize + x) as usize,
            (self.y as isize + y) as usize,
        )
    }
    fn neighbors(&self) -> Vec<Loc> {
        let mut neighbors = vec![self.offset_by(1, 0), self.offset_by(0, 1)];
        if self.x > 0 {
            neighbors.push(self.offset_by(-1, 0));
        }
        if self.y > 0 {
            neighbors.push(self.offset_by(0, -1));
        }
        neighbors
    }
}
impl Debug for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum RegionType {
    Rocky,
    Narrow,
    Wet,
}

impl ToString for RegionType {
    fn to_string(&self) -> String {
        match self {
            RegionType::Rocky => ".",
            RegionType::Narrow => "|",
            RegionType::Wet => "=",
        }
        .to_string()
    }
}
impl From<ErosionLevel> for RegionType {
    fn from(erosion_level: ErosionLevel) -> Self {
        match erosion_level % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }
}

impl Debug for RegionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl RegionType {
    fn to_risk_level(self) -> RiskLevel {
        match self {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }

    fn is_incompatible_with(&self, tool: &Tool) -> bool {
        match self {
            RegionType::Rocky => tool == &Tool::Neither,
            RegionType::Narrow => tool == &Tool::ClimbingGear,
            RegionType::Wet => tool == &Tool::Torch,
        }
    }
}

pub(crate) struct Cave {
    erosion_levels: HashMap<Loc, ErosionLevel>,
    depth: Depth,
    target: Loc,
}

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn others(&self) -> Vec<Tool> {
        match self {
            Tool::Torch => vec![Tool::ClimbingGear, Tool::Neither],
            Tool::ClimbingGear => vec![Tool::Torch, Tool::Neither],
            Tool::Neither => vec![Tool::ClimbingGear, Tool::Torch],
        }
    }
}

impl Cave {
    fn new(depth: Depth, target: Loc) -> Self {
        Cave {
            erosion_levels: HashMap::new(),
            depth,
            target,
        }
    }
    #[allow(dead_code)]
    fn as_string(&mut self, width: usize, height: usize) -> String {
        (0..height)
            .into_iter()
            .map(|y| {
                (0..width)
                    .into_iter()
                    .map(|x| {
                        if self.target.equals(x, y) {
                            "T".to_string()
                        } else if Cave::is_mouth(x, y) {
                            "M".to_string()
                        } else {
                            self.region_type_at(&x, &y).to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn erosion_level_at(&mut self, x: &X, y: &Y) -> ErosionLevel {
        let loc = Loc::new(*x, *y);
        if let Some(erosion_level) = self.erosion_levels.get(&loc) {
            *erosion_level
        } else {
            let geo_index = match (x, y) {
                (0, 0) => 0,
                (0, y) => y * Y_MULTI,
                (x, 0) => x * X_MULTI,
                (x, y) => {
                    if loc == self.target {
                        0
                    } else {
                        let left = self.erosion_level_at(&(x - 1), y);
                        let above = self.erosion_level_at(x, &(y - 1));
                        left * above
                    }
                }
            };
            let erosion_level = self.erosion_level(geo_index);
            self.erosion_levels.insert(loc, erosion_level);
            erosion_level
        }
    }

    fn erosion_level(&self, geo_index: GeologicIndex) -> ErosionLevel {
        (geo_index + self.depth) % MODULO
    }

    #[allow(dead_code)]
    fn is_mouth(x: usize, y: usize) -> bool {
        x == 0 && y == 0
    }

    pub(crate) fn risk_level(&mut self) -> RiskLevel {
        (0..=self.target.y)
            .into_iter()
            .map(|y| {
                (0..=self.target.x)
                    .into_iter()
                    .map(|x| self.risk_level_at(&x, &y))
                    .sum::<usize>()
            })
            .sum()
    }

    fn risk_level_at(&mut self, x: &X, y: &Y) -> RiskLevel {
        self.region_type_at(x, y).to_risk_level()
    }

    fn region_type_at_loc(&mut self, loc: &Loc) -> RegionType {
        self.region_type_at(&loc.x, &loc.y)
    }

    fn region_type_at(&mut self, x: &X, y: &Y) -> RegionType {
        RegionType::from(self.erosion_level_at(x, y))
    }

    pub(crate) fn shortest_path_len(&mut self) -> usize {
        let mut visited: HashMap<(Loc, Tool), Time> = HashMap::new();

        let mut queue = BinaryHeap::new();
        let origin = Loc::new(0, 0);
        queue.push(State {
            time: 0,
            loc: origin,
            tool: Tool::Torch,
            dist: self.target.hamming_distance_to(&origin),
        });

        while let Some(curr) = queue.pop() {
            if self
                .region_type_at_loc(&curr.loc)
                .is_incompatible_with(&curr.tool)
            {
                continue;
            }

            // Only redo a location if we got here faster than previously
            let prev_time = visited.entry((curr.loc, curr.tool)).or_insert(usize::MAX);
            if curr.time < *prev_time {
                *prev_time = curr.time;
            } else {
                continue;
            }

            // Check if we found the target
            if self.target == curr.loc && curr.tool == Tool::Torch {
                // println!("Reached target in iteration {}", counter);
                // Cave::print_shortest_paths(&visited);
                return curr.time;
            }

            // Try from neighboring positions with the same tool
            curr.loc.neighbors().into_iter().for_each(|loc| {
                let dist = self.target.hamming_distance_to(&loc);
                queue.push(State::new(curr.time + 1, loc, curr.tool, dist));
            });

            // Try from here with other tools
            let dist = self.target.hamming_distance_to(&curr.loc);
            curr.tool.others().into_iter().for_each(|tool| {
                queue.push(State::new(curr.time + 7, curr.loc, tool, dist));
            });
        }
        unreachable!()
    }

    #[allow(dead_code)]
    fn print_shortest_paths(times: &HashMap<(Loc, Tool), Time>) {
        let width = times.keys().map(|(loc, _)| loc.x).max().unwrap();
        let height = times.keys().map(|(loc, _)| loc.y).max().unwrap();
        println!("w x h = {} x {}", width, height);
        let mut grid: Vec<Vec<Option<Time>>> = vec![vec![None; width + 1]; height + 1];
        // Move the best time for each square into the grid
        [Tool::Neither, Tool::Torch, Tool::ClimbingGear]
            .iter()
            .for_each(|tool| {
                (0..height).into_iter().for_each(|y| {
                    (0..width).into_iter().for_each(|x| {
                        match (times.get(&(Loc::new(x, y), *tool)), grid[y][x]) {
                            (Some(time), Some(other)) => grid[y][x] = Some(*time.min(&other)),
                            (Some(time), None) => grid[y][x] = Some(*time),
                            (None, _) => {}
                        }
                    })
                })
            });
        // Then print the grid
        let max_value_width = times.values().max().unwrap().to_string().len();
        grid.iter().for_each(|row| {
            println!(
                "{:?}",
                row.iter()
                    .map(|time| match time {
                        Some(time) => format!("{:width$}", time, width = max_value_width),
                        None => " ".repeat(max_value_width),
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        });
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct State {
    time: Time,
    loc: Loc,
    tool: Tool,
    dist: usize,
}

impl State {
    fn new(time: Time, loc: Loc, tool: Tool, dist: usize) -> Self {
        State {
            time,
            loc,
            tool,
            dist,
        }
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.time + self.dist)
            .cmp(&(other.time + other.dist))
            .reverse()
    }
}

pub(crate) fn full_cave() -> Cave {
    Cave::new(DEPTH, Loc::new(TARGET.0, TARGET.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_to_string() {
        let mut cave = Cave::new(510, Loc::new(10, 10));
        assert_eq!(
            cave.as_string(16, 16),
            "\
M=.|=.|.|=.|=|=.
.|=|=|||..|.=...
.==|....||=..|==
=.|....|.==.|==.
=|..==...=.|==..
=||.=.=||=|=..|=
|.=.===|||..=..|
|..==||=.|==|===
.=..===..=|.|||.
.======|||=|=.|=
.===|=|===T===||
=|||...|==..|=.|
=.=|=.=..=.||==|
||=|=...|==.=|==
|=.=||===.|||===
||.|==.|.|.||=||"
        )
    }
    #[test]
    fn example_risk_level() {
        let mut cave = Cave::new(510, Loc::new(10, 10));
        assert_eq!(114, cave.risk_level());
    }

    #[test]
    fn part1() {
        assert_eq!(10_115, day22_part1());
    }

    #[test]
    fn example_shortest_path_len() {
        let mut cave = Cave::new(510, Loc::new(10, 10));
        assert_eq!(45, cave.shortest_path_len());
    }

    #[test]
    fn part2() {
        assert_eq!(990, day22_part2());
    }

    #[test]
    fn binary_heap_ordering() {
        let tool = Tool::Torch;
        let loc = Loc::new(0, 0);

        let one = State::new(0, loc, tool, 10);
        let three = State::new(2, loc, tool, 6);
        let two = State::new(1, loc, tool, 8);

        let mut queue = BinaryHeap::new();
        queue.push(one);
        queue.push(three.clone());
        queue.push(two);

        // ordering is by lowest (dist + time)
        assert_eq!(queue.peek(), Some(&three));
    }
}
