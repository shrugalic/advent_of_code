use crate::day16::Map;
use crate::vec_2d::Vec2D;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("../../2024/input/day20.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT, 100)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT, 100)
}

fn solve_part1(input: &str, min_time_saved: usize) -> usize {
    let path = Map::from(input).find_shortest_path();
    count_shortcuts(path, min_time_saved, 2)
}

fn solve_part2(input: &str, min_time_saved: usize) -> usize {
    let path = Map::from(input).find_shortest_path();
    count_shortcuts(path, min_time_saved, 20)
}

fn count_shortcuts(path: Path, min_time_saved: usize, max_cheat_time: usize) -> usize {
    let mut shortcuts_by_time_saved: HashMap<usize, HashSet<(&Vec2D, &Vec2D)>> = HashMap::new();
    for (t_start, start) in path.0.iter().enumerate() {
        for (t_end, end) in path.0.iter().enumerate().skip(t_start + 2) {
            let normal_time = t_end - t_start;
            let cheat_time = ((end.x - start.x).abs() + (end.y - start.y).abs()) as usize;
            if cheat_time > max_cheat_time {
                continue;
            }
            let time_saved = normal_time - cheat_time;
            if time_saved > 0 {
                shortcuts_by_time_saved
                    .entry(time_saved)
                    .or_default()
                    .insert((start, end));
            }
        }
    }
    shortcuts_by_time_saved
        .into_iter()
        .filter(|(time_saved, _)| time_saved >= &min_time_saved)
        .map(|(_, shortcuts)| shortcuts.len())
        .sum()
}

/// This uses the Map and Tile structs from day 16
impl Map {
    fn find_shortest_path(&self) -> Path {
        let mut queue: BinaryHeap<Path> = [Path::new(self.start_pos)].into_iter().collect();
        while let Some(path) = queue.pop() {
            let curr_pos = path.0.last().unwrap();
            if curr_pos == &self.end_pos {
                return path;
            }
            queue.extend(
                curr_pos
                    .crosswise_neighbors()
                    .filter(|next_pos| {
                        self.tile_is_empty_at(next_pos) && !path.0.contains(next_pos)
                    })
                    .map(|next_pos| {
                        let mut next_path = path.clone();
                        next_path.0.push(next_pos);
                        next_path
                    }),
            );
        }
        unreachable!()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Path(Vec<Vec2D>);

impl Path {
    fn new(start: Vec2D) -> Self {
        Path(vec![start])
    }
}
/// BinaryHeap returns the greatest element, but since the question asks
/// for the lowest cost, let's order the path by lowest length instead
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.len().cmp(&other.0.len()).reverse()
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1,
            solve_part1(EXAMPLE, 2)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(1289, solve_part1(INPUT, 100));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
            solve_part2(EXAMPLE, 50)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(982_425, solve_part2(INPUT, 100));
    }
}
