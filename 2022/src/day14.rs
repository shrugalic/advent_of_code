use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day14.txt");

pub(crate) fn day14_part1() -> usize {
    FallingSandCave::from(INPUT)
        .let_sand_fall_until_stable()
        .sand_grain_count()
}

pub(crate) fn day14_part2() -> usize {
    FallingSandCave::from(INPUT)
        .add_floor()
        .let_sand_fall_until_stable()
        .sand_grain_count()
}

type Coord = usize;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: Coord,
    y: Coord,
}
impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Pos {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Rock,   // #
    Sand,   // o
    Source, // +
    Air,    // .
}
impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Rock => '#',
            Tile::Sand => 'o',
            Tile::Source => '+',
            Tile::Air => '.',
        }
    }
}

#[derive(Debug)]
struct FallingSandCave {
    obstacles: HashMap<Pos, Tile>,
    x_range: RangeInclusive<Coord>,
    y_range: RangeInclusive<Coord>,
}
impl FallingSandCave {
    fn can_flow_to(&self, x: Coord, y: Coord) -> bool {
        [Tile::Air, Tile::Source].contains(self.tile_at(x, y))
    }
    fn tile_at(&self, x: Coord, y: Coord) -> &Tile {
        self.obstacles.get(&Pos { x, y }).unwrap_or(&Tile::Air)
    }
    fn sand_grain_count(&self) -> usize {
        self.obstacles
            .values()
            .filter(|&tile| tile == &Tile::Sand)
            .count()
    }
    fn let_sand_fall_until_stable(mut self) -> Self {
        'outer: loop {
            let mut x = 500;
            for y in self.y_range.clone() {
                match (
                    self.can_flow_to(x - 1, y + 1),
                    self.can_flow_to(x, y + 1),
                    self.can_flow_to(x + 1, y + 1),
                ) {
                    (_, true, _) => {}
                    (true, false, _) => {
                        x -= 1;
                    }
                    (false, false, true) => {
                        x += 1;
                    }
                    (false, false, false) => {
                        self.obstacles.insert(Pos { x, y }, Tile::Sand);
                        // println!("\n{}", self);
                        if x == 500 && y == 0 {
                            break 'outer;
                        }
                        continue 'outer;
                    }
                }
            }
            break 'outer;
        }
        self
    }

    fn add_floor(mut self) -> Self {
        let y = self.y_range.end() + 2;
        self.y_range = *self.y_range.start()..=y;
        self.x_range = (*self.x_range.start() - y)..=(*self.x_range.end() + y);
        for x in self.x_range.clone() {
            self.obstacles.insert(Pos { x, y }, Tile::Rock);
        }
        // println!("\n{}", self);
        self
    }
}
impl From<&str> for FallingSandCave {
    fn from(input: &str) -> Self {
        let mut obstacles = HashMap::new();
        let (mut min_x, mut max_x, mut max_y) = (usize::MAX, 0, 0);
        for points in input
            .lines()
            .map(|line| line.split(" -> ").map(Pos::from).collect::<Vec<_>>())
        {
            min_x = min(min_x, points[0].x);
            max_x = max(max_x, points[0].x);
            max_y = max(max_y, points[0].y);
            for pos in points.windows(2) {
                let a = &pos[0];
                let b = &pos[1];
                min_x = min(min_x, b.x);
                max_x = max(max_x, b.x);
                max_y = max(max_y, b.y);
                if a.x == b.x {
                    let x = a.x;
                    for y in min(a.y, b.y)..=max(a.y, b.y) {
                        obstacles.insert(Pos { x, y }, Tile::Rock);
                    }
                } else if a.y == b.y {
                    let y = a.y;
                    for x in min(a.x, b.x)..=max(a.x, b.x) {
                        obstacles.insert(Pos { x, y }, Tile::Rock);
                    }
                } else {
                    panic!("Input points not horizontally or vertically aligned");
                }
            }
        }
        obstacles.insert(Pos { x: 500, y: 0 }, Tile::Source);
        FallingSandCave {
            obstacles,
            x_range: (min_x - 1)..=(max_x + 1),
            y_range: 0..=max_y,
        }
    }
}
impl Display for FallingSandCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.y_range
                .clone()
                .into_iter()
                .map(|y| self
                    .x_range
                    .clone()
                    .into_iter()
                    .map(|x| self
                        .obstacles
                        .get(&Pos { x, y })
                        .unwrap_or(&Tile::Air)
                        .to_char())
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        let cave = FallingSandCave::from(EXAMPLE);
        // println!("{}", cave);
        let cave = cave.let_sand_fall_until_stable();
        // println!("\n{}", cave);
        assert_eq!(24, cave.sand_grain_count());
    }

    #[test]
    fn part1() {
        assert_eq!(913, day14_part1());
    }

    #[test]
    fn part2_example() {
        let cave = FallingSandCave::from(EXAMPLE).add_floor();
        // println!("{}", cave);
        let cave = cave.let_sand_fall_until_stable();
        // println!("\n{}", cave);
        assert_eq!(93, cave.sand_grain_count());
    }

    #[test]
    fn part2() {
        assert_eq!(30_762, day14_part2());
    }
}
