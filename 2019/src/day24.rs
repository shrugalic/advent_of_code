use crate::parse;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../input/day24.txt");

pub(crate) fn day24_part1() -> usize {
    let mut grid = Grid::from(parse(INPUT));
    grid.iterate_until_pattern_repeats();
    grid.biodiversity_rating()
}

pub(crate) fn day24_part2() -> usize {
    let mut grids = Grids::from(parse(INPUT));
    grids.iterate(200);
    grids.total_bug_count()
}


#[derive(Debug, PartialEq, Clone)]
struct Grids {
    center: Grid,
    inner: Vec<Grid>,
    outer: Vec<Grid>,
}
impl Grids {
    fn iterate(&mut self, minutes: usize) {
        for _ in 0..minutes {
            self.step();
        }
    }
    fn step(&mut self) {
        if self.inner.is_empty() || self.inner[self.inner.len() - 1].has_bugs_around_center() {
            self.inner.push(Grid::default());
        }
        if self.outer.is_empty() || self.outer[self.outer.len() - 1].has_bugs_outer_outer_edge() {
            self.outer.push(Grid::default());
        }
        let previous = self.clone();
        let level_range = -(self.outer.len() as isize)..=(self.inner.len() as isize);

        let dummy = Grid::default();
        for level in level_range {
            let center = previous.grid_at(level).unwrap_or(&dummy);
            let new = self.mut_grid_at(level);
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let bug_count = previous.bugs_to_the_left(x, y, level)
                        + previous.bugs_above(x, y, level)
                        + previous.bugs_to_the_right(x, y, level)
                        + previous.bugs_below(x, y, level);
                    new.grid[y][x] = bug_count == 1 || !center.has_bug_at(x, y) && bug_count == 2;
                }
            }
        }
    }
    fn grids_at(&self, level: isize) -> (Option<&Grid>, Option<&Grid>, Option<&Grid>) {
        (
            self.grid_at(level),
            self.grid_at(level + 1),
            self.grid_at(level - 1),
        )
    }
    fn grid_at(&self, level: isize) -> Option<&Grid> {
        match level {
            0 => Some(&self.center),
            1..=isize::MAX => self.inner.get(level as usize - 1),
            isize::MIN..=-1 => self.outer.get(level.abs() as usize - 1),
            _ => unreachable!(),
        }
    }
    fn mut_grid_at(&mut self, level: isize) -> &mut Grid {
        match level {
            0 => &mut self.center,
            1..=isize::MAX => &mut self.inner[level as usize - 1],
            isize::MIN..=-1 => &mut self.outer[level.abs() as usize - 1],
            _ => unreachable!(),
        }
    }
    fn bugs_to_the_left(&self, x: usize, y: usize, level: isize) -> usize {
        let (center, inner, outer) = self.grids_at(level);
        match (x, y) {
            (0, _) => outer.map(Grid::left_of_center_bug_count).unwrap_or(0),
            (3, 2) => inner.map(Grid::right_col_bug_count).unwrap_or(0),
            (_, _) => center.unwrap().bug_count_at(x - 1, y),
        }
    }
    fn bugs_to_the_right(&self, x: usize, y: usize, level: isize) -> usize {
        let (center, inner, outer) = self.grids_at(level);
        match (x, y) {
            (4, _) => outer.map(Grid::right_of_center_bug_count).unwrap_or(0),
            (1, 2) => inner.map(Grid::left_col_bug_count).unwrap_or(0),
            (_, _) => center.unwrap().bug_count_at(x + 1, y),
        }
    }
    fn bugs_above(&self, x: usize, y: usize, level: isize) -> usize {
        let (center, inner, outer) = self.grids_at(level);
        match (x, y) {
            (_, 0) => outer.map(Grid::above_center_bug_count).unwrap_or(0),
            (2, 3) => inner.map(Grid::bottom_row_bug_count).unwrap_or(0),
            (_, _) => center.unwrap().bug_count_at(x, y - 1),
        }
    }
    fn bugs_below(&self, x: usize, y: usize, level: isize) -> usize {
        let (center, inner, outer) = self.grids_at(level);
        match (x, y) {
            (_, 4) => outer.map(Grid::below_center_bug_count).unwrap_or(0),
            (2, 1) => inner.map(Grid::top_row_bug_count).unwrap_or(0),
            (_, _) => center.unwrap().bug_count_at(x, y + 1),
        }
    }
    fn total_bug_count(&self) -> usize {
        self.center.total_bug_count()
            + self.inner.iter().map(Grid::total_bug_count).sum::<usize>()
            + self.outer.iter().map(Grid::total_bug_count).sum::<usize>()
    }
}

impl From<Vec<&str>> for Grids {
    fn from(s: Vec<&str>) -> Self {
        let center = Grid::from(s);
        Grids {
            center,
            inner: vec![],
            outer: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    grid: [[bool; 5]; 5],
}
impl Default for Grid {
    fn default() -> Self {
        let grid = [[false; 5]; 5];
        Grid { grid }
    }
}
impl From<Vec<&str>> for Grid {
    fn from(s: Vec<&str>) -> Self {
        let mut grid = Grid::default();
        s.iter().enumerate().for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| grid.grid[y][x] = c == '#')
        });
        grid
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row
                    .iter()
                    .map(|v| if *v { '#' } else { '.' })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
impl Grid {
    fn around_inner_center() -> [(usize, usize); 8] {
        [
            // above center
            (1, 1),
            (2, 1),
            (3, 1),
            // left of center
            (1, 2),
            // right of center
            (3, 2),
            // below center
            (1, 3),
            (2, 3),
            (3, 3),
        ]
    }
    fn around_outer_edge() -> [(usize, usize); 16] {
        [
            // top
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            // bottom
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (4, 4),
            // left
            (0, 1),
            (0, 2),
            (0, 3),
            // right
            (4, 1),
            (4, 2),
            (4, 3),
        ]
    }
    fn below_center_bug_count(&self) -> usize {
        self.bug_count_at(2, 3)
    }
    fn above_center_bug_count(&self) -> usize {
        self.bug_count_at(2, 1)
    }
    fn right_of_center_bug_count(&self) -> usize {
        self.bug_count_at(3, 2)
    }
    fn left_of_center_bug_count(&self) -> usize {
        self.bug_count_at(1, 2)
    }
    fn total_bug_count(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum()
    }
    fn iterate_until_pattern_repeats(&mut self) {
        let mut ids = HashSet::new();
        while !ids.contains(&self.biodiversity_rating()) {
            ids.insert(self.biodiversity_rating());
            self.iterate();
        }
    }
    fn iterate(&mut self) {
        let prev = self.clone();
        for x in 0..5 {
            for y in 0..5 {
                let count = prev.get_neighboring_bug_count(x, y);
                self.grid[y][x] = count == 1 || !prev.has_bug_at(x, y) && count == 2;
            }
        }
    }
    fn get_neighboring_bug_count(&self, x: usize, y: usize) -> usize {
        let conditions = [
            x > 0 && self.has_bug_at(x - 1, y),
            y > 0 && self.has_bug_at(x, y - 1),
            self.has_bug_at(x + 1, y),
            self.has_bug_at(x, y + 1),
        ];
        conditions.iter().filter(|&&has_bug| has_bug).count()
    }
    fn has_bug_at(&self, x: usize, y: usize) -> bool {
        *self
            .grid
            .get(y)
            .and_then(|row| row.get(x))
            .unwrap_or(&false)
    }
    fn bug_count_at(&self, x: usize, y: usize) -> usize {
        if self.has_bug_at(x, y) {
            1
        } else {
            0
        }
    }
    fn right_col_bug_count(&self) -> usize {
        self.count_bugs_at(&[(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)])
    }
    fn left_col_bug_count(&self) -> usize {
        self.count_bugs_at(&[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)])
    }
    fn top_row_bug_count(&self) -> usize {
        self.count_bugs_at(&[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)])
    }
    fn bottom_row_bug_count(&self) -> usize {
        self.count_bugs_at(&[(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)])
    }
    fn has_bugs_around_center(&self) -> bool {
        self.count_bugs_at(&Grid::around_inner_center()) > 0
    }
    fn has_bugs_outer_outer_edge(&self) -> bool {
        self.count_bugs_at(&Grid::around_outer_edge()) > 0
    }
    fn count_bugs_at(&self, pos: &[(usize, usize)]) -> usize {
        pos.iter()
            .map(|(x, y)| if self.grid[*y][*x] { 1 } else { 0 })
            .sum()
    }
    fn biodiversity_rating(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, v)| {
                        if *v {
                            2_usize.pow((5 * y + x) as u32)
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE_STABLE: &str = "\
.....
.....
.....
#....
.#...";

    const EXAMPLE_INITIAL: &str = "\
....#
#..#.
#..##
..#..
#....";

    #[test]
    fn test_biodiversity_rating() {
        let area = Grid::from(parse(EXAMPLE_STABLE));
        assert_eq!(2129920, area.biodiversity_rating());
    }

    #[test]
    fn test_iterate_until_pattern_repeats() {
        let mut area = Grid::from(parse(EXAMPLE_INITIAL));
        area.iterate_until_pattern_repeats();
        println!("{}", area);
        assert_eq!(2129920, area.biodiversity_rating());
    }

    #[test]
    fn test_iterate() {
        let mut area = Grid::from(parse(EXAMPLE_INITIAL));
        area.iterate();
        assert_eq!(
            area,
            Grid::from(parse(
                "\
#..#.
####.
###.#
##.##
.##.."
            ))
        );
        area.iterate();
        assert_eq!(
            area,
            Grid::from(parse(
                "\
#####
....#
....#
...#.
#.###"
            ))
        );
        area.iterate();
        assert_eq!(
            area,
            Grid::from(parse(
                "\
#....
####.
...##
#.##.
.##.#"
            ))
        );
        area.iterate();
        assert_eq!(
            area,
            Grid::from(parse(
                "\
####.
....#
##..#
.....
##..."
            ))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(27777901, day24_part1());
    }

    #[test]
    fn part2_example_step() {
        let mut grids = Grids::from(parse(EXAMPLE_INITIAL));
        grids.step();
        assert_eq!(
            /*center*/ 15 + /*outer*/3 +  /*inner*/9,
            grids.total_bug_count()
        );
    }

    #[test]
    fn part2_example_iterate() {
        let mut grids = Grids::from(parse(EXAMPLE_INITIAL));
        grids.iterate(10);
        assert_eq!(99, grids.total_bug_count());
    }

    #[test]
    fn part2() {
        assert_eq!(2047, day24_part2());
    }
}
