use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day09.txt");

pub(crate) fn day09_part1() -> usize {
    let map = HeightMap::from(INPUT);
    map.sum_of_risk_levels()
}

pub(crate) fn day09_part2() -> usize {
    let map = HeightMap::from(INPUT);
    map.product_of_basin_sizes()
}

type Height = u8;
type Pos = (usize, usize);

struct HeightMap {
    grid: Vec<Vec<Height>>,
}
impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        HeightMap {
            grid: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as Height)
                        .collect()
                })
                .collect::<Vec<_>>(),
        }
    }
}
impl HeightMap {
    fn sum_of_risk_levels(&self) -> usize {
        self.low_points()
            .into_iter()
            .map(|(_, height)| 1 + height as usize)
            .sum::<usize>()
    }

    fn low_points(&self) -> Vec<(Pos, Height)> {
        let mut low_points = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                if self.is_lower_than_neighbors(x, y) {
                    low_points.push(((x, y), *height));
                }
            }
        }
        low_points
    }

    fn is_lower_than_neighbors(&self, x: usize, y: usize) -> bool {
        let height = self.grid[y][x];
        self.neighbors_of(x, y)
            .into_iter()
            .all(|(x2, y2)| height < self.grid[y2][x2])
    }

    fn neighbors_of(&self, x: usize, y: usize) -> Vec<Pos> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < self.grid[0].len() {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y + 1 < self.grid.len() {
            neighbors.push((x, y + 1));
        }
        neighbors
    }

    fn product_of_basin_sizes(&self) -> usize {
        let low_points = self.low_points();
        let mut basin_sizes = vec![];
        for ((x, y), _) in low_points {
            let mut visited = HashSet::new();
            let mut candidates = vec![(x, y)];
            while let Some((x, y)) = candidates.pop() {
                visited.insert((x, y));
                candidates.extend(
                    self.neighbors_of(x, y)
                        .into_iter()
                        .filter(|(x, y)| !visited.contains(&(*x, *y)) && self.grid[*y][*x] < 9),
                );
            }
            basin_sizes.push(visited.len());
        }
        basin_sizes.sort_unstable();
        basin_sizes.into_iter().rev().take(3).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_example() {
        let map = HeightMap::from(EXAMPLE);
        assert_eq!(15, map.sum_of_risk_levels());
    }
    #[test]
    fn part1() {
        assert_eq!(564, day09_part1());
    }

    #[test]
    fn part2_example() {
        let map = HeightMap::from(EXAMPLE);
        assert_eq!(1134, map.product_of_basin_sizes());
    }

    #[test]
    fn part2() {
        assert_eq!(1_038_240, day09_part2());
    }
}
