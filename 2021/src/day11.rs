use std::collections::HashSet;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day11.txt");

pub(crate) fn day11_part1() -> usize {
    let mut octopuses = Octopuses::from(INPUT);
    octopuses.count_flashes_for_step_count(100)
}

pub(crate) fn day11_part2() -> usize {
    let mut octopuses = Octopuses::from(INPUT);
    octopuses.count_steps_until_all_flash_simultaneously()
}

type EnergyLevel = u8;
struct Octopuses {
    grid: Vec<Vec<EnergyLevel>>,
}
impl Octopuses {
    fn count_flashes_for_step_count(&mut self, n: usize) -> usize {
        (0..n).into_iter().map(|_| self.flash_count_in_step()).sum()
    }

    fn count_steps_until_all_flash_simultaneously(&mut self) -> usize {
        let octopus_count = self.grid[0].len() * self.grid.len();
        let mut step_count = 1;
        while self.flash_count_in_step() < octopus_count {
            step_count += 1;
        }
        step_count
    }

    fn flash_count_in_step(&mut self) -> usize {
        let mut flashed_pos = HashSet::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                self.grid[y][x] += 1;
                self.flash(x, y, &mut flashed_pos);
            }
        }
        let flash_count = flashed_pos.len();
        flashed_pos.drain().for_each(|(x, y)| self.grid[y][x] = 0);
        flash_count
    }

    fn flash(&mut self, x: usize, y: usize, flashed_pos: &mut HashSet<(usize, usize)>) {
        if self.grid[y][x] > 9 && flashed_pos.insert((x, y)) {
            // propagate
            for (nx, ny) in self.neighbors_of(x, y) {
                self.grid[ny][nx] += 1;
                self.flash(nx, ny, flashed_pos);
            }
        }
    }

    fn neighbors_of(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            // (0, 0) center
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(|(x, y)| self.contains(x, y))
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
    }

    fn contains(&self, x: &isize, y: &isize) -> bool {
        (0..self.grid[0].len() as isize).contains(x) && (0..self.grid.len() as isize).contains(y)
    }
}
impl From<&str> for Octopuses {
    fn from(input: &str) -> Self {
        Octopuses {
            grid: input
                .trim()
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as EnergyLevel)
                        .collect()
                })
                .collect::<Vec<_>>(),
        }
    }
}
impl Display for Octopuses {
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

    const SMALL_EXAMPLE: &str = "\
11111
19991
19191
19991
11111";

    const EXAMPLE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_neighbors_of() {
        let octopuses = Octopuses::from(EXAMPLE);
        assert_eq!(
            octopuses.neighbors_of(1, 1),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                // (1, 1) center
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ],
        );
    }

    #[test]
    fn part1_small_example() {
        let mut octopuses = Octopuses::from(SMALL_EXAMPLE);
        assert_eq!(9, octopuses.count_flashes_for_step_count(3));
    }

    #[test]
    fn part1_example() {
        let mut octopuses = Octopuses::from(EXAMPLE);
        assert_eq!(1656, octopuses.count_flashes_for_step_count(100));
    }

    #[test]
    fn part1() {
        assert_eq!(1721, day11_part1());
    }

    #[test]
    fn part2_example() {
        let mut octopuses = Octopuses::from(EXAMPLE);
        assert_eq!(195, octopuses.count_steps_until_all_flash_simultaneously());
    }

    #[test]
    fn part2() {
        assert_eq!(298, day11_part2());
    }
}
