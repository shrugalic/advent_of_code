use std::fmt::{Debug, Formatter};

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

#[derive(Clone, Copy, PartialEq)]
struct Loc {
    x: X,
    y: Y,
}
impl Loc {
    fn new(x: X, y: Y) -> Self {
        Loc { x, y }
    }
    fn equals(&self, x: X, y: Y) -> bool {
        self.x == x && self.y == y
    }
}
impl Debug for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Type {
    Rocky,
    Narrow,
    Wet,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Rocky => ".",
            Type::Narrow => "|",
            Type::Wet => "=",
        }
        .to_string()
    }
}
impl From<ErosionLevel> for Type {
    fn from(erosion_level: ErosionLevel) -> Self {
        match erosion_level % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => unreachable!(),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Type {
    fn to_risk_level(self) -> RiskLevel {
        match self {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2,
        }
    }
}

pub(crate) struct Cave<T> {
    grid: Vec<Vec<T>>,
    depth: Depth,
    target: Loc,
}
impl<T: ToString> ToString for Cave<T> {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, t)| {
                        if self.target.equals(x, y) {
                            "T".to_string()
                        } else if Cave::is_mouth(x, y) {
                            "M".to_string()
                        } else {
                            t.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
impl Cave<Type> {
    fn new(depth: Depth, target: Loc) -> Self {
        // Let the size be 60% larger than the target:
        // - this works out for the example (10, 10) target's grid size of 16 x 16
        // - hopefully is enough to find a path in part 2
        let width = (target.x as f64 * 1.6) as usize;
        let height = (target.y as f64 * 1.6) as usize;
        let mut cave = Cave {
            grid: vec![vec![Type::Rocky; width]; height],
            depth,
            target,
        };

        let mut levels = vec![vec![0; width]; height];

        // init y = 0
        let y = 0;
        (1..width).into_iter().for_each(|x| {
            let erosion_level = cave.erosion_level(x * X_MULTI);
            levels[y][x] = erosion_level;
            cave.set(&Loc::new(x, y), Type::from(erosion_level));
        });

        // init x = 0
        let x = 0;
        (1..height).into_iter().for_each(|y| {
            let erosion_level = cave.erosion_level(y * Y_MULTI);
            levels[y][x] = erosion_level;
            cave.set(&Loc::new(x, y), Type::from(erosion_level));
        });

        // fill in the rest diagonally
        let mut x = 1;
        let mut y = 1;
        let mut sum = x + y;
        while sum <= height + width - 2 {
            let erosion_level = cave.erosion_level_at(&mut levels, x, y);
            levels[y][x] = erosion_level;
            cave.set(&Loc::new(x, y), Type::from(erosion_level));

            if y == 1 || x == width - 1 {
                sum += 1;
                x = if sum > height { sum - height } else { 0 };
            }
            x += 1;
            y = sum - x;
        }

        cave
    }

    fn erosion_level_at(&self, levels: &mut Vec<Vec<ErosionLevel>>, x: X, y: Y) -> ErosionLevel {
        let geo_index = if self.is_target(x, y) {
            0
        } else {
            levels[y][x - 1] * levels[y - 1][x]
        };
        self.erosion_level(geo_index)
    }

    fn erosion_level(&self, geo_index: GeologicIndex) -> ErosionLevel {
        (geo_index + self.depth) % MODULO
    }

    fn is_target_or_mouth(&self, x: X, y: Y) -> bool {
        self.is_target(x, y) || Cave::is_mouth(x, y)
    }

    fn is_mouth(x: usize, y: usize) -> bool {
        x == 0 && y == 0
    }
    fn is_target(&self, x: X, y: Y) -> bool {
        x == self.target.x && y == self.target.y
    }
    pub(crate) fn risk_level(&self) -> RiskLevel {
        (0..=self.target.y)
            .into_iter()
            .flat_map(|y| {
                (0..=self.target.x)
                    .into_iter()
                    .map(move |x| self.risk_level_at(x, y))
            })
            .sum()
    }
    fn risk_level_at(&self, x: usize, y: usize) -> RiskLevel {
        let tile_type = if self.is_target_or_mouth(x, y) {
            Type::from(self.erosion_level(0))
        } else {
            *self.get(&Loc::new(x, y)).unwrap()
        };
        tile_type.to_risk_level()
    }

    fn get(&self, loc: &Loc) -> Option<&Type> {
        self.grid
            .get(loc.y as usize)
            .and_then(|line| line.get(loc.x as usize))
    }
    fn set(&mut self, loc: &Loc, t: Type) {
        self.grid[loc.y as usize][loc.x as usize] = t;
    }
}

pub(crate) fn part_1_cave() -> Cave<Type> {
    Cave::new(DEPTH, Loc::new(TARGET.0, TARGET.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_to_string() {
        let cave = Cave::new(510, Loc::new(10, 10));
        assert_eq!(
            cave.to_string(),
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
        let cave = Cave::new(510, Loc::new(10, 10));
        assert_eq!(114, cave.risk_level());
    }

    #[test]
    fn part1_risk_level() {
        let cave = part_1_cave();
        assert_eq!(10115, cave.risk_level());
    }
}
