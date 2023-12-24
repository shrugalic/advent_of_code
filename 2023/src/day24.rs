use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day24.txt");
const EXAMPLE_INTERSECTION_BOUNDARY: RangeInclusive<f64> = 7.0..=27.0;
const REAL_INTERSECTION_BOUNDARY: RangeInclusive<f64> =
    200_000_000_000_000.0..=400_000_000_000_000.0;

pub(crate) fn part1() -> usize {
    solve_part1(INPUT, &REAL_INTERSECTION_BOUNDARY)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str, boundary: &RangeInclusive<f64>) -> usize {
    let hailstones: Vec<_> = parse(input).collect();
    hailstones
        .iter()
        .enumerate()
        .map(|(i, hs1)| {
            hailstones[i + 1..]
                .iter()
                .filter(|hs2| {
                    hs1.trajectory_intersects_with(hs2)
                        .within_xy_boundaries(boundary)
                })
                .count()
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let hailstones: Vec<_> = parse(input).collect();
    0
}

fn parse(input: &str) -> impl Iterator<Item = Hailstone> + '_ {
    input.trim().lines().map(Hailstone::from)
}

type Coord = i64;
#[derive(Debug, PartialEq)]
struct Coord3D {
    x: Coord,
    y: Coord,
    z: Coord,
}

#[derive(Debug, PartialEq)]
struct Hailstone {
    position: Coord3D,
    velocity: Coord3D,
}

struct Pair<'h> {
    hs1: &'h Hailstone,
    hs2: &'h Hailstone,
}

impl Pair<'_> {
    fn within_xy_boundaries(&self, boundary: &RangeInclusive<f64>) -> bool {
        let px1 = self.hs1.position.x as f64;
        let vx1 = self.hs1.velocity.x as f64;
        let py1 = self.hs1.position.y as f64;
        let vy1 = self.hs1.velocity.y as f64;

        let px2 = self.hs2.position.x as f64;
        let vx2 = self.hs2.velocity.x as f64;
        let py2 = self.hs2.position.y as f64;
        let vy2 = self.hs2.velocity.y as f64;

        let x1 = |t: f64| px1 + t * vx1;
        let x2 = |t: f64| px2 + t * vx2;
        let y1 = |t: f64| py1 + t * vy1;
        let y2 = |t: f64| py2 + t * vy2;

        // x = px1 + vx1 * t1 = px2 + vx2 * t2
        // y = py1 + vy1 * t1 = py2 + vy2 * t2
        //
        // vx1 * t1 = px2 + vx2 * t2 - px1
        // t1 = (px2 + vx2 * t2 - px1) / vx1
        //
        // vy2 * t2 = py1 + vy1 * t1 - py2
        // t2 = (py1 + vy1 * t1 - py2) / vy2
        //
        // t1 = (px2 + vx2 * t2 - px1) / vx1
        // t1 = (px2 + vx2 * ((py1 + vy1 * t1 - py2) / vy2) - px1) / vx1
        // t1 * vx1 = px2 + vx2 * ((py1 + vy1 * t1 - py2) / vy2) - px1
        // t1 * vx1 = px2 - px1 + vx2 * ((py1 + vy1 * t1 - py2) / vy2)
        // t1 * vx1 = px2 - px1 + vx2 * py1 / vy2 + vx2 * vy1 * t1 / vy2 - vx2 * py2 / vy2
        // t1 * vx1 - vx2 * vy1 * t1 / vy2 = px2 - px1 + vx2 * py1 / vy2 - vx2 * py2 / vy2
        // t1 * (vx1 - vx2 * vy1 / vy2) = px2 - px1 + vx2 * py1 / vy2 - vx2 * py2 / vy2
        // t1 = (px2 - px1 + vx2 * py1 / vy2 - vx2 * py2 / vy2) / (vx1 - vx2 * vy1 / vy2)
        let (t1, t2) = if (vx1 - vx2 * vy1 / vy2) != 0.0 {
            let t1 = (px2 - px1 + vx2 * py1 / vy2 - vx2 * py2 / vy2) / (vx1 - vx2 * vy1 / vy2);
            let t2_of = |t1| (py1 + vy1 * t1 - py2) / vy2;
            let t2 = t2_of(t1);
            (t1, t2)
        } else if (vx2 - vx1 * vy2 / vy1) != 0.0 {
            let t2 = (px1 - px2 + vx1 * py2 / vy1 - vx1 * py1 / vy1) / (vx2 - vx1 * vy2 / vy1);
            let t1_of = |t2| (px2 + vx2 * t2 - px1) / vy2;
            let t1 = t1_of(t2);
            (t1, t2)
        } else {
            // parallel
            return false;
            unreachable!()
        };
        // println!("t1 {t1}");
        // println!("t2 {t2}");
        // println!("x1(t1) {}", x1(t1));
        // println!("y1(t1) {}", y1(t1));
        // println!("x2(t2) {}", x2(t2));
        // println!("y2(t2) {}", y2(t2));

        boundary.contains(&x1(t1))
            && boundary.contains(&y1(t1))
            && boundary.contains(&x2(t2))
            && boundary.contains(&y2(t2))
            && t1 > 0.0
            && t2 > 0.0
    }
}

impl Hailstone {
    fn trajectory_intersects_with<'h>(&'h self, other: &'h Hailstone) -> Pair {
        Pair {
            hs1: self,
            hs2: other,
        }
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (position, velocity) = value.split_once(" @ ").unwrap();
        let position = Coord3D::from(position);
        let velocity = Coord3D::from(velocity);
        Hailstone {
            position: position,
            velocity,
        }
    }
}

impl From<&str> for Coord3D {
    fn from(value: &str) -> Self {
        let mut v = value.split(", ").map(|n| n.trim().parse().unwrap());
        let x = v.next().unwrap();
        let y = v.next().unwrap();
        let z = v.next().unwrap();
        Coord3D { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_parsing_example() {
        let hailstone = Hailstone::from("20, 19, 15 @  1, -5, -3");
        assert_eq!(
            hailstone,
            Hailstone {
                position: Coord3D {
                    x: 20,
                    y: 19,
                    z: 15
                },
                velocity: Coord3D { x: 1, y: -5, z: -3 }
            }
        );
    }

    #[test]
    fn test_trajectory_intersection() {
        let hs: Vec<Hailstone> = parse(EXAMPLE).collect();
        let intersect = |a: &Hailstone, b: &Hailstone| -> bool {
            a.trajectory_intersects_with(b)
                .within_xy_boundaries(&EXAMPLE_INTERSECTION_BOUNDARY)
        };
        assert!(intersect(&hs[0], &hs[1]));
        assert!(intersect(&hs[0], &hs[2]));
        assert!(!intersect(&hs[0], &hs[3]));
        assert!(!intersect(&hs[0], &hs[4]));
        assert!(!intersect(&hs[1], &hs[2]));
        assert!(!intersect(&hs[1], &hs[3]));
        assert!(!intersect(&hs[1], &hs[4]));
        assert!(!intersect(&hs[2], &hs[3]));
        assert!(!intersect(&hs[2], &hs[4]));
        assert!(!intersect(&hs[3], &hs[4]));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(2, solve_part1(EXAMPLE, &EXAMPLE_INTERSECTION_BOUNDARY));
    }

    #[test]
    fn test_part1() {
        assert_eq!(16_727, solve_part1(INPUT, &REAL_INTERSECTION_BOUNDARY));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(1, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, solve_part2(INPUT));
    }
}
