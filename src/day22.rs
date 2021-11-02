use line_reader::read_file_to_lines;
use std::fmt::{Debug, Formatter};

pub(crate) fn day22_part1() -> usize {
    let lines = read_file_to_lines("input/day22.txt");
    let pairs = parse_pairs(lines);
    let stats = pairs.into_iter().map(|n| n.stats).collect();
    viable_pair_count(stats)
}

pub(crate) fn day22_part2() -> usize {
    let lines = read_file_to_lines("input/day22.txt");
    let pairs = parse_pairs(lines);
    count_steps_to_move_goal_data_to_origin(pairs)
}

fn parse_pairs(lines: Vec<String>) -> Vec<Pair> {
    lines
        .iter()
        .skip(2)
        .map(|s| Pair::from(s.as_str()))
        .collect()
}

fn viable_pair_count(mut stats: Vec<Stats>) -> usize {
    let mut count = 0;
    while let Some(stat) = stats.pop() {
        count += stats
            .iter()
            .filter(|&other| stat.viably_pairs_with(other))
            .count();
    }
    count
}

fn count_steps_to_move_goal_data_to_origin(pairs: Vec<Pair>) -> usize {
    let mut cluster = Cluster::from(pairs);
    // println!("{:?}\n", cluster);

    let mut count = 0;
    let mut empty = cluster.pos_of_empty_node();
    count += cluster.move_empty_node_to_the_top(&mut empty);
    // println!("{:?}\n", cluster);
    count += cluster.move_empty_node_to_the_right_until_swapped_with_goal_node(&mut empty);
    // println!("{:?}\n", cluster);
    count += cluster.move_goal_node_left_until_it_reaches_the_origin(&mut empty);
    // println!("{:?}\n", cluster);

    count
}

type Coord = usize;
type FileSize = u16;

#[derive(PartialEq, Clone)]
enum Node {
    Goal,
    Empty,
    Normal,
    Full,
}
impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::Goal => "G",
            Node::Empty => "_",
            Node::Normal => ".",
            Node::Full => "#",
        }
        .to_string()
    }
}

#[derive(PartialEq)]
struct Cluster {
    grid: Vec<Vec<Node>>,
}
impl Cluster {
    fn pos_of_empty_node(&self) -> Pos {
        self.grid
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, n)| n == &&Node::Empty)
                    .map(|(x, _)| Pos { x, y })
            })
            .next()
            .unwrap()
    }
    fn move_empty_node_to_the_top(&mut self, empty: &mut Pos) -> usize {
        let mut count = 0;
        while empty.y > 0 {
            match self.grid[empty.y - 1][empty.x] {
                Node::Normal => {
                    self.grid[empty.y][empty.x] = self.grid[empty.y - 1][empty.x].clone();
                    empty.y -= 1;
                    self.grid[empty.y][empty.x] = Node::Empty;
                    count += 1;
                }
                Node::Goal => {
                    return count + 1;
                }
                Node::Full => {
                    self.grid[empty.y][empty.x] = self.grid[empty.y][empty.x - 1].clone();
                    empty.x -= 1;
                    self.grid[empty.y][empty.x] = Node::Empty;
                    count += 1;
                }
                Node::Empty => unreachable!(),
            }
        }
        count
    }
    fn move_empty_node_to_the_right_until_swapped_with_goal_node(
        &mut self,
        empty: &mut Pos,
    ) -> usize {
        let mut count = 0;
        while empty.x < self.grid[0].len() - 1 {
            self.grid[empty.y][empty.x] = self.grid[empty.y][empty.x + 1].clone();
            empty.x += 1;
            self.grid[empty.y][empty.x] = Node::Empty;
            count += 1;
        }
        count
    }

    fn move_goal_node_left_until_it_reaches_the_origin(&mut self, empty: &mut Pos) -> usize {
        let mut count = 0;
        while self.grid[0][1] != Node::Empty {
            count += self.move_goal_node_left_once(empty);
        }
        count
    }

    fn move_goal_node_left_once(&mut self, empty: &mut Pos) -> usize {
        self.grid[empty.y][empty.x] = self.grid[empty.y + 1][empty.x].clone();
        empty.y += 1;

        self.grid[empty.y][empty.x] = self.grid[empty.y][empty.x - 1].clone();
        empty.x -= 1;

        self.grid[empty.y][empty.x] = self.grid[empty.y][empty.x - 1].clone();
        empty.x -= 1;

        self.grid[empty.y][empty.x] = self.grid[empty.y - 1][empty.x].clone();
        empty.y -= 1;

        self.grid[empty.y][empty.x] = self.grid[empty.y][empty.x + 1].clone();
        empty.x += 1;

        self.grid[empty.y][empty.x] = Node::Empty;

        5
    }
}
impl From<Vec<Pair>> for Cluster {
    fn from(pairs: Vec<Pair>) -> Self {
        let max = pairs.iter().map(|n| &n.pos).max().unwrap();

        // Init grid with normal nodes and mark the goal node
        let mut grid = vec![vec![Node::Normal; max.x + 1]; max.y + 1];
        grid[0][max.x] = Node::Goal;

        // Find the empty node and mark nodes that wouldn't fit into it as full
        let empty_node = pairs.iter().find(|pair| pair.stats.is_empty()).unwrap();
        grid[empty_node.pos.y][empty_node.pos.x] = Node::Empty;
        pairs
            .iter()
            .filter(|pair| pair.stats.used > empty_node.stats.available)
            .for_each(|pair| grid[pair.pos.y][pair.pos.x] = Node::Full);

        Cluster { grid }
    }
}
impl ToString for Cluster {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
impl Debug for Cluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, PartialEq)]
struct Pair {
    pos: Pos,
    stats: Stats,
}
impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once(|c| c == ' ').unwrap();
        Pair {
            pos: Pos::from(left),
            stats: Stats::from(right),
        }
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq)]
struct Pos {
    x: Coord,
    y: Coord,
}
impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        // Example: /dev/grid/node-x0-y0
        let (left, right) = s
            .trim_start_matches("/dev/grid/node-x")
            .split_once(|c| c == '-')
            .unwrap();
        let x = left.parse().unwrap();
        let y = right.trim_start_matches('y').parse().unwrap();
        Pos { x, y }
    }
}

#[derive(PartialEq)]
struct Stats {
    used: FileSize,
    available: FileSize,
}
impl From<&str> for Stats {
    fn from(s: &str) -> Self {
        // Example:     92T   72T    20T   78%
        let parts: Vec<_> = s.trim().split_ascii_whitespace().collect();
        let used = parts[1].trim_end_matches('T').parse().unwrap();
        let available = parts[2].trim_end_matches('T').parse().unwrap();
        Stats { used, available }
    }
}
impl ToString for Stats {
    fn to_string(&self) -> String {
        format!("{:3}/{:3}", self.used, self.used + self.available)
    }
}
impl Stats {
    fn is_empty(&self) -> bool {
        self.used == 0
    }
    fn would_fit_in(&self, other: &Stats) -> bool {
        self.used <= other.available
    }
    fn viably_pairs_with(&self, other: &Stats) -> bool {
        !self.is_empty() && self.would_fit_in(other)
            || !other.is_empty() && other.would_fit_in(self)
    }
}
impl Debug for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    #[test]
    fn parse_node() {
        let actual = Pair::from("/dev/grid/node-x0-y0     92T   72T    20T   78%");
        let expected = Pair {
            pos: Pos { x: 0, y: 0 },
            stats: Stats::new(72, 20),
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1() {
        assert_eq!(937, day22_part1());
    }

    const EXAMPLE: &str = "\
root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";
    #[test]
    fn part2_example() {
        let nodes = parse_pairs(read_str_to_lines(EXAMPLE));

        assert_eq!(7, count_steps_to_move_goal_data_to_origin(nodes));
    }

    #[test]
    fn part2() {
        assert_eq!(188, day22_part2());
    }
}
