const INPUT: &str = include_str!("../input/day08.txt");

pub(crate) fn day08_part1() -> usize {
    let grid = TreeHeights::from(INPUT);
    grid.count_trees_visible_from_outside()
}

pub(crate) fn day08_part2() -> usize {
    let grid = TreeHeights::from(INPUT);
    grid.max_view_distance()
}

struct TreeHeights {
    grid: Vec<Vec<u8>>,
}
impl From<&str> for TreeHeights {
    fn from(input: &str) -> Self {
        TreeHeights {
            grid: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).expect("a valid digit") as u8)
                        .collect()
                })
                .collect(),
        }
    }
}
impl TreeHeights {
    fn count_trees_visible_from_outside(&self) -> usize {
        let mut count = 0;
        for (y, line) in self.grid.iter().enumerate() {
            for (x, tree) in line.iter().enumerate() {
                if self.is_visible_from_any_direction(x, y, tree) {
                    count += 1;
                }
            }
        }
        count
    }
    fn is_visible_from_any_direction(&self, x: usize, y: usize, tree: &u8) -> bool {
        (0..x).all(|from_left| self.grid[y][from_left] < *tree)
            || (x + 1..self.width()).all(|from_right| self.grid[y][from_right] < *tree)
            || (0..y).all(|from_top| self.grid[from_top][x] < *tree)
            || (y + 1..self.height()).all(|from_bottom| self.grid[from_bottom][x] < *tree)
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn max_view_distance(&self) -> usize {
        let mut view_distances = Vec::with_capacity(self.width() * self.height());
        for (y, line) in self.grid.iter().enumerate() {
            for (x, tree) in line.iter().enumerate() {
                view_distances.push(self.view_distance_at(x, y, tree));
            }
        }
        *view_distances.iter().max().expect("at least one entry")
    }
    fn view_distance_at(&self, x: usize, y: usize, tree: &u8) -> usize {
        let left = x
            - (0..x)
                .rev()
                .find(|&l| self.grid[y][l] >= *tree)
                .unwrap_or(0);
        let right = (x + 1..self.width())
            .find(|&r| self.grid[y][r] >= *tree)
            .unwrap_or(self.width() - 1)
            - x;
        let up = y
            - (0..y)
                .rev()
                .find(|&t| self.grid[t][x] >= *tree)
                .unwrap_or(0);
        let down = (y + 1..self.height())
            .find(|&b| self.grid[b][x] >= *tree)
            .unwrap_or(self.height() - 1)
            - y;

        left * right * up * down
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn part1_example() {
        let input = TreeHeights::from(EXAMPLE);
        assert_eq!(21, input.count_trees_visible_from_outside());
    }

    #[test]
    fn part2_example() {
        let input = TreeHeights::from(EXAMPLE);
        assert_eq!(8, input.max_view_distance());
    }

    #[test]
    fn part1() {
        assert_eq!(1_832, day08_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(157_320, day08_part2());
    }
}
