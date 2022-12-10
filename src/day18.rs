use crate::parse;

const INPUT: &str = include_str!("../input/day18.txt");

pub(crate) fn day18_part1() -> usize {
    let input = parse(INPUT);
    let mut grid = Grid::from(input);
    for _ in 0..100 {
        grid.iterate_part1()
    }
    grid.turned_on_lights_count()
}

pub(crate) fn day18_part2() -> usize {
    let input = parse(INPUT);
    let mut grid = Grid::from(input);
    for _ in 0..100 {
        grid.iterate_part2()
    }
    grid.turned_on_lights_count()
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<bool>>,
    on_count: Vec<Vec<u8>>, // Keeps track of the neighbor-on-count for each cell
}
impl From<Vec<&str>> for Grid {
    fn from(input: Vec<&str>) -> Grid {
        let grid: Vec<Vec<bool>> = input
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        let on_count = (0..grid.len())
            .into_iter()
            .map(|y| {
                (0..grid[0].len())
                    .into_iter()
                    .map(|x| {
                        let neighbors = Grid::safe_neighbors(x, y, grid.len(), grid[0].len());
                        Grid::count_turned_on_locations(neighbors, &grid)
                    })
                    .collect()
            })
            .collect();
        Grid { grid, on_count }
    }
}
impl Grid {
    fn iterate_part1(&mut self) {
        self.iterate_internal(false);
    }
    fn iterate_part2(&mut self) {
        self.iterate_internal(true);
    }
    fn iterate_internal(&mut self, corners_always_on: bool) {
        let previous = self.clone();
        for y in 0..previous.grid.len() {
            for x in 0..previous.grid[0].len() {
                match (previous.grid[y][x], previous.on_count[y][x]) {
                    (true, 0..=1 | 4..=8) => self.turn_off(x, y),
                    (false, 3) => self.turn_on(x, y),
                    (_, _) => {}
                }
            }
        }
        if corners_always_on {
            self.turn_on_corners();
        }
    }

    fn turn_on_corners(&mut self) {
        let max_y = self.grid.len() - 1;
        let max_x = self.grid[0].len() - 1;
        self.turn_on(0, 0);
        self.turn_on(0, max_y);
        self.turn_on(max_x, 0);
        self.turn_on(max_x, max_y);
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        self.set_to(true, x, y);
    }

    fn turn_off(&mut self, x: usize, y: usize) {
        self.set_to(false, x, y)
    }

    fn set_to(&mut self, new_state: bool, x: usize, y: usize) {
        if self.grid[y][x] == new_state {
            return;
        }
        self.neighbors_of(x, y).iter().for_each(|(x, y)| {
            if new_state {
                self.on_count[*y][*x] += 1
            } else {
                self.on_count[*y][*x] -= 1
            }
        });
        self.grid[y][x] = new_state
    }

    fn turned_on_lights_count(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&on| *on).count())
            .sum()
    }

    fn count_turned_on_locations(locs: Vec<(usize, usize)>, grid: &[Vec<bool>]) -> u8 {
        locs.iter().filter(|(x, y)| grid[*y][*x]).count() as u8
    }

    fn neighbors_of(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        Grid::safe_neighbors(x, y, self.grid.len(), self.grid[0].len())
    }

    fn safe_neighbors(x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
        (-1..=1)
            .into_iter()
            .flat_map(|dy| {
                (-1..=1)
                    .into_iter()
                    .filter(move |dx| dy != 0 || dx != &0)
                    .map(move |dx| ((x as isize + dx) as usize, (y as isize + dy) as usize))
            })
            .filter(|(x, y)| (0..height).contains(y) && (0..width).contains(x))
            .collect()
    }
}
impl ToString for Grid {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&on| if on { '#' } else { '.' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn part1_example() {
        let input = parse(EXAMPLE1[0]);
        let mut grid = Grid::from(input);
        for expected in EXAMPLE1 {
            assert_eq!(grid.to_string(), expected);
            grid.iterate_part1();
        }
    }
    #[test]
    fn part1() {
        assert_eq!(768, day18_part1());
    }

    #[test]
    fn part2_example() {
        let input = parse(EXAMPLE2[0]);
        let mut grid = Grid::from(input);
        for expected in EXAMPLE2 {
            assert_eq!(grid.to_string(), expected);
            grid.iterate_part2();
        }
    }
    #[test]
    fn part2() {
        assert_eq!(781, day18_part2());
    }

    const EXAMPLE1: [&str; 5] = [
        "\
.#.#.#
...##.
#....#
..#...
#.#..#
####..",
        "\
..##..
..##.#
...##.
......
#.....
#.##..",
        "\
..###.
......
..###.
......
.#....
.#....",
        "\
...#..
......
...#..
..##..
......
......",
        "\
......
......
..##..
..##..
......
......",
    ];

    const EXAMPLE2: [&str; 6] = [
        "\
##.#.#
...##.
#....#
..#...
#.#..#
####.#",
        "\
#.##.#
####.#
...##.
......
#...#.
#.####",
        "\
#..#.#
#....#
.#.##.
...##.
.#..##
##.###",
        "\
#...##
####.#
..##.#
......
##....
####.#",
        "\
#.####
#....#
...#..
.##...
#.....
#.#..#",
        "\
##.###
.##..#
.##...
.##...
#.#...
##...#",
    ];
}
