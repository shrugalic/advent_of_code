use std::cmp::Ordering;

const INPUT: &str = include_str!("../input/day22.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    let (bricks, _) = parse_and_settle(input);
    let redundant_bricks: Vec<_> = bricks
        .iter()
        .filter(|brick| {
            // all supported bricks (if any) are also supported by another brick
            brick
                .supported_bricks(&bricks)
                .iter()
                .all(|supported_by_brick| supported_by_brick.supporting_brick_count(&bricks) > 1)
        })
        .collect();
    redundant_bricks.len()
}

fn solve_part2(input: &str) -> usize {
    let (bricks, _) = parse_and_settle(input);
    let single_supporting_bricks: Vec<_> = bricks
        .iter()
        .filter(|brick| {
            !brick
                .supported_bricks(&bricks)
                .iter()
                .all(|above| above.supporting_brick_count(&bricks) > 1)
        })
        .collect();

    let mut total_moved_bricks_count = 0;
    for support in single_supporting_bricks {
        let mut with_support_removed = bricks.clone();
        let index = bricks.iter().position(|brick| brick == support).unwrap();
        with_support_removed.remove(index);
        let (_, moved_bricks_count) = settle(with_support_removed.clone());
        total_moved_bricks_count += moved_bricks_count;
    }
    total_moved_bricks_count
}

fn parse_and_settle(input: &str) -> (Vec<Brick>, usize) {
    let bricks = parse(input);
    settle(bricks)
}

fn settle(mut bricks: Vec<Brick>) -> (Vec<Brick>, usize) {
    // sort highest to lowest, so pop() yields lower first
    bricks.sort_unstable_by(|a, b| b.cmp(a));

    let floor = bricks.iter().fold(Brick::default(), |mut acc, brick| {
        acc.top.x = acc.top.x.max(brick.bottom.x.max(brick.top.x));
        acc.top.y = acc.top.y.max(brick.bottom.y.max(brick.top.y));
        acc.bottom.x = acc.bottom.x.min(brick.bottom.x.min(brick.top.x));
        acc.bottom.y = acc.bottom.y.min(brick.bottom.y.min(brick.top.y));
        acc
    });

    // add temporary floor to simplify logic
    let mut settled = vec![floor];

    let mut moved_bricks_count = 0;

    // Let them settle starting from the bottom
    while let Some(mut brick) = bricks.pop() {
        if let Some(lowest_possible_z) = settled
            .iter()
            .filter(|other| other.top.z < brick.bottom.z)
            .filter(|lower| brick.xy_intersect(lower))
            .map(|intersecting_lower| intersecting_lower.top.z + 1)
            .max()
        {
            let delta = brick.bottom.z - lowest_possible_z;
            if delta > 0 {
                brick.bottom.z -= delta;
                brick.top.z -= delta;
                moved_bricks_count += 1;
            }
        }
        settled.push(brick);
    }

    // remove floor
    settled.remove(0);
    (settled, moved_bricks_count)
}

fn parse(input: &str) -> Vec<Brick> {
    input.trim().lines().map(Brick::from).collect()
}

impl Brick {
    fn single(x: Coord, y: Coord, z: Coord) -> Self {
        Brick {
            top: Position { x, y, z },
            bottom: Position { x, y, z },
        }
    }
    fn new(x1: Coord, y1: Coord, z1: Coord, x2: Coord, y2: Coord, z2: Coord) -> Self {
        if x1 == x2 && y1 == y2 || x1 == x2 && z1 == z2 || y1 == y2 && z1 == z2 {
            let p1 = Position::new(x1, y1, z1);
            let p2 = Position::new(x2, y2, z2);
            let (top, bottom) = match p1.z > p2.z {
                true => (p1, p2),
                false => (p2, p1),
            };
            Brick { top, bottom }
        } else {
            panic!("Bricks must be aligned to axes")
        }
    }
    fn xy_intersect(&self, other: &Brick) -> bool {
        let xs = self.bottom.x..=self.top.x;
        let ys = self.bottom.y..=self.top.y;
        let other_xs = other.bottom.x..=other.top.x;
        let other_ys = other.bottom.y..=other.top.y;
        other_xs.start() <= xs.end()
            && xs.start() <= other_xs.end()
            && other_ys.start() <= ys.end()
            && ys.start() <= other_ys.end()
    }
    fn supported_bricks<'a>(self: &'a Brick, bricks: &'a [Brick]) -> Vec<&'a Brick> {
        bricks
            .iter()
            .filter(|other| other.bottom.z == self.top.z + 1)
            .filter(|other| self.xy_intersect(other))
            .collect()
    }
    fn supporting_brick_count(&self, bricks: &[Brick]) -> usize {
        bricks
            .iter()
            .filter(|other| other.top.z + 1 == self.bottom.z)
            .filter(|other| self.xy_intersect(other))
            .count()
    }
}

type Coord = u16;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Default, Clone, Copy)]
struct Position {
    x: Coord,
    y: Coord,
    z: Coord,
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
struct Brick {
    bottom: Position,
    top: Position,
}

impl PartialOrd<Self> for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bottom
            .z
            .cmp(&other.bottom.z)
            .then_with(|| self.top.z.cmp(&other.top.z))
            .then_with(|| self.bottom.x.cmp(&other.bottom.x))
            .then_with(|| self.top.x.cmp(&other.top.x))
            .then_with(|| self.bottom.y.cmp(&other.bottom.y))
            .then_with(|| self.top.y.cmp(&other.top.y))
    }
}

impl Position {
    fn new(x: Coord, y: Coord, z: Coord) -> Self {
        Position { x, y, z }
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        // 1,0,1~1,2,1
        let (a, b) = value.split_once('~').unwrap();
        let a = Position::from(a);
        let b = Position::from(b);
        let (top, bottom) = match a.cmp(&b) {
            Ordering::Greater => (a, b),
            Ordering::Less | Ordering::Equal => (b, a),
        };
        Brick { top, bottom }
    }
}
impl From<&str> for Position {
    fn from(value: &str) -> Self {
        // 1,1,9
        let coords: Vec<_> = value.split(',').map(|n| n.parse().unwrap()).collect();
        Position {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_parsed_bricks_top_is_above_bottom() {
        assert_eq!(
            Brick::from("1,1,8~1,1,9"),
            Brick {
                top: Position { x: 1, y: 1, z: 9 },
                bottom: Position::new(1, 1, 8)
            }
        );
        assert_eq!(
            Brick::from("1,1,9~1,1,8"),
            Brick {
                top: Position { x: 1, y: 1, z: 9 },
                bottom: Position { x: 1, y: 1, z: 8 }
            }
        );
    }

    #[test]
    fn test_sorting_puts_lower_bottom_first() {
        let mut bricks = vec![
            Brick::new(0, 0, 2, 2, 0, 2),
            Brick::single(1, 1, 1),
            Brick::new(0, 2, 2, 2, 2, 2),
            Brick::new(0, 2, 3, 2, 2, 3),
            Brick::new(1, 1, 5, 1, 1, 4),
            Brick::new(0, 0, 4, 0, 0, 5),
        ];
        bricks.sort_unstable();

        assert_eq!(
            bricks,
            vec![
                Brick::single(1, 1, 1),
                Brick::new(0, 0, 2, 2, 0, 2),
                Brick::new(0, 2, 2, 2, 2, 2),
                Brick::new(0, 2, 3, 2, 2, 3),
                Brick::new(0, 0, 4, 0, 0, 5),
                Brick::new(1, 1, 4, 1, 1, 5),
            ]
        );
    }

    #[test]
    fn test_settle() {
        let (bricks, _) = parse_and_settle(EXAMPLE);
        assert_eq!(
            bricks,
            vec![
                Brick::new(1, 0, 1, 1, 2, 1), // A
                Brick::new(0, 0, 2, 2, 0, 2), // B
                Brick::new(0, 2, 2, 2, 2, 2), // C
                Brick::new(0, 0, 3, 0, 2, 3), // D
                Brick::new(2, 0, 3, 2, 2, 3), // E
                Brick::new(0, 1, 4, 2, 1, 4), // F
                Brick::new(1, 1, 5, 1, 1, 6), // G
            ]
        );
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(5, solve_part1(EXAMPLE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(386, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(7, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(39_933, solve_part2(INPUT));
    }
}
