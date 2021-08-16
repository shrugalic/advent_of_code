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
    fn offset_by(&self, x: isize, y: isize) -> Self {
        Loc::new(
            (self.x as isize + x) as usize,
            (self.y as isize + y) as usize,
        )
    }
    fn neighbors(&self) -> Vec<Loc> {
        let mut neighbors = vec![self.offset_by(1, 0), self.offset_by(0, 1)];
        if self.x > 0 {
            neighbors.push(self.offset_by(-1, 0));
        }
        if self.y > 0 {
            neighbors.push(self.offset_by(0, -1));
        }
        neighbors
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
    fn is_incompatible_with(&self, tool: &Tool) -> bool {
        match self {
            Type::Rocky => tool == &Tool::Neither,
            Type::Narrow => tool == &Tool::ClimbingGear,
            Type::Wet => tool == &Tool::Torch,
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
#[derive(PartialEq, Copy, Clone, Debug)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn others(&self) -> Vec<Tool> {
        match self {
            Tool::Torch => vec![Tool::ClimbingGear, Tool::Neither],
            Tool::ClimbingGear => vec![Tool::Torch, Tool::Neither],
            Tool::Neither => vec![Tool::ClimbingGear, Tool::Torch],
        }
    }
}

impl Tool {}

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

    fn shortest_path_len(&self) -> usize {
        let mut counter = 0;
        let mut shortest_path_len = vec![vec![usize::MAX; self.grid[0].len()]; self.grid.len()];
        shortest_path_len[0][0] = 0;
        let mut candidates = vec![(0, Loc::new(0, 0), Tool::Torch)];
        while let Some((curr_len, curr_loc, curr_tool)) = candidates.pop() {
            // while !candidates.is_empty() {
            //     let (fastest, _, _) = candidates.iter().min_by_key(|(len, _, _)| len).unwrap();
            //     let pos = candidates
            //         .iter()
            //         .position(|(len, _, _)| len == fastest)
            //         .unwrap();
            //     let (curr_len, curr_loc, curr_tool) = candidates.remove(pos);
            curr_loc.neighbors().iter().for_each(|next_loc| {
                if let Some(curr_shortest_path_to_next_loc) = shortest_path_len
                    .get(next_loc.y)
                    .and_then(|row| row.get(next_loc.x))
                {
                    let next_type = self.get(next_loc).unwrap();
                    let reached_target_and_need_torch_switch =
                        next_loc == &self.target && curr_tool != Tool::Torch;
                    let (next_len, next_tools) = if next_type.is_incompatible_with(&curr_tool)
                        || reached_target_and_need_torch_switch
                    {
                        (curr_len + 8, curr_tool.others())
                    } else {
                        (curr_len + 1, vec![curr_tool])
                    };
                    if next_len < *curr_shortest_path_to_next_loc {
                        shortest_path_len[next_loc.y][next_loc.x] = next_len;
                        next_tools.iter().for_each(|next_tool| {
                            candidates.push((next_len, *next_loc, *next_tool))
                        });
                    }
                } // else the next loc is outside the grid
            });
            counter += 1;

            // Keep only candidates that are better than the current best
            candidates = candidates
                .into_iter()
                .filter(|(len, loc, _)| *len <= shortest_path_len[loc.y][loc.x])
                .collect();

            // Prioritize candidates with shorter travel times
            candidates.sort_by_key(|(len, _loc, _tool)| -(*len as isize));

            // if counter >= 1000 {
            //     break;
            // }
        }
        println!("Finished in {} iterations", counter);

        if false {
            // print shortest paths
            shortest_path_len.iter().for_each(|row| {
                println!(
                    "{:?}",
                    row.iter()
                        .map(|len| if len == &usize::MAX {
                            "9999".to_string()
                        } else {
                            format!("{:4}", len)
                        })
                        .collect::<Vec<_>>()
                );
            });
        }

        shortest_path_len[self.target.y][self.target.x]
    }
}

pub(crate) fn full_cave() -> Cave<Type> {
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
        let cave = full_cave();
        assert_eq!(10115, cave.risk_level());
    }

    #[test]
    fn example_shortest_path_len() {
        let cave = Cave::new(510, Loc::new(10, 10));
        assert_eq!(45, cave.shortest_path_len());
    }

    #[test]
    fn part2_shortest_path_len() {
        let cave = full_cave();
        // 1040 is too high
        assert_eq!(1039, cave.shortest_path_len());
    }
}
