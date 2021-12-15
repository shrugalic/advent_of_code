use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day15.txt");

pub(crate) fn day15_part1() -> usize {
    let mut cavern = Cavern::from(INPUT);
    cavern.risk_level_sum_of_lowest_risk_path()
}

pub(crate) fn day15_part2() -> usize {
    let cavern = Cavern::from(INPUT);
    cavern.enlarge().risk_level_sum_of_lowest_risk_path()
}

type RiskLevel = u8;
type Pos = (usize, usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    risk_level_sum: usize,
    pos: Pos,
    end: Pos,
}
impl State {
    fn new(end: Pos) -> Self {
        State {
            risk_level_sum: 0,
            pos: (0, 0),
            end,
        }
    }
    fn reached_end(&self) -> bool {
        self.pos == self.end
    }
    fn moved_to(&self, pos: Pos, risk: RiskLevel) -> Self {
        State {
            risk_level_sum: self.risk_level_sum + risk as usize,
            pos,
            end: self.end,
        }
    }
    fn neighbors(&self) -> Vec<(usize, usize)> {
        [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .into_iter()
            .map(|(dx, dy)| (self.pos.0 as isize + dx, self.pos.1 as isize + dy))
            .filter(|(x, y)| self.contains(x, y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect()
    }
    fn contains(&self, x: &isize, y: &isize) -> bool {
        (0..=self.end.0 as isize).contains(x) && (0..=self.end.1 as isize).contains(y)
    }
}

struct Cavern {
    grid: Vec<Vec<RiskLevel>>,
}
impl Cavern {
    fn risk_level_sum_of_lowest_risk_path(&mut self) -> usize {
        let mut candidates = BinaryHeap::new();
        candidates.push(Reverse(State::new(self.end())));
        let mut visited = HashSet::new();
        while let Some(Reverse(curr)) = candidates.pop() {
            if curr.reached_end() {
                return curr.risk_level_sum;
            }
            for next_pos in curr.neighbors() {
                if visited.insert(next_pos) {
                    let risk = self.risk_level(next_pos);
                    candidates.push(Reverse(curr.moved_to(next_pos, risk)))
                }
            }
        }
        unreachable!()
    }
    fn enlarge(self) -> Self {
        let w = self.grid[0].len();
        let h = self.grid.len();
        const FACTOR: usize = 5;

        let mut grid = vec![vec![0; w * FACTOR]; h * FACTOR];
        for tile_y in 0..FACTOR {
            let base_y = tile_y * h;
            for tile_x in 0..FACTOR {
                let base_x = tile_x * w;
                for (y, row) in self.grid.iter().enumerate() {
                    for (x, risk) in row.iter().enumerate() {
                        grid[base_y + y][base_x + x] = (risk + (tile_x + tile_y) as u8 - 1) % 9 + 1;
                    }
                }
            }
        }
        Cavern { grid }
    }
    fn end(&self) -> Pos {
        (self.grid[0].len() - 1, self.grid.len() - 1)
    }
    fn risk_level(&self, pos: Pos) -> RiskLevel {
        self.grid[pos.1][pos.0]
    }
}
impl From<&str> for Cavern {
    fn from(input: &str) -> Self {
        Cavern {
            grid: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as RiskLevel)
                        .collect()
                })
                .collect::<Vec<_>>(),
        }
    }
}
impl Display for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(u8::to_string).collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part1_example() {
        let mut cavern = Cavern::from(EXAMPLE);
        assert_eq!(40, cavern.risk_level_sum_of_lowest_risk_path());
    }

    #[test]
    fn part1() {
        assert_eq!(745, day15_part1());
    }

    #[test]
    fn test_enlarge() {
        let cavern = Cavern::from("8");
        assert_eq!(
            vec![
                vec![8, 9, 1, 2, 3],
                vec![9, 1, 2, 3, 4],
                vec![1, 2, 3, 4, 5],
                vec![2, 3, 4, 5, 6],
                vec![3, 4, 5, 6, 7]
            ],
            cavern.enlarge().grid
        );
    }

    #[test]
    fn part2_example() {
        let cavern = Cavern::from(EXAMPLE);
        assert_eq!(315, cavern.enlarge().risk_level_sum_of_lowest_risk_path());
    }

    #[test]
    fn part2() {
        assert_eq!(3002, day15_part2());
    }
}
