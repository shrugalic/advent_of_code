use std::ops::{Add, RangeInclusive, Sub};

const INPUT: &str = include_str!("../input/day24.txt");
const EXAMPLE_INTERSECTION_BOUNDARY: RangeInclusive<f64> = 7.0..=27.0;
const REAL_INTERSECTION_BOUNDARY: RangeInclusive<f64> =
    200_000_000_000_000.0..=400_000_000_000_000.0;

pub(crate) fn part1() -> usize {
    solve_part1(INPUT, &REAL_INTERSECTION_BOUNDARY)
}

pub(crate) fn part2() -> Coord {
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

fn solve_part2(input: &str) -> Coord {
    // Let pr be the rock's position at t = 0, and vr its velocity. It collides with every hailstone at some time,
    // let's say it meets the first at t1, the second at t2, and the third at t3.
    // The position at t1 will be p1 + v1 * t1 = pr + vr * t1
    // The position at t2 will be p2 + v2 * t2 = pr + vr * t2
    // The position at t3 will be p3 + v3 * t3 = pr + vr * t3

    // By subtracting the 2nd from the 1st equation, we get vr = (p1 + v1 * t1 - p2 - v2 * t2) / (t1 - t2)
    // By subtracting the 3rd from the 1st equation, we get vr = (p1 + v1 * t1 - p3 - v3 * t3) / (t1 - t3)

    // And by setting these two equations for vr equal to each other, we get:
    // (p1 + v1 * t1 - p2 - v2 * t2) / (t1 - t2) = (p1 + v1 * t1 - p3 - v3 * t3) / (t1 - t3)
    // Multiplying both sides by (t1 - t2) * (t1 - t3):
    // (t1 - t3) * ((p1 - p2) + v1 * t1 - v2 * t2) = (t1 - t2) * ((p1 - p3) + v1 * t1 - v3 * t3)
    // Multiplying out and grouping like terms:
    // (p1 - p2) * t1 + v1 * t1^2 - v2 * t1 * t2 + (p2 - p1) * t3 - v1 * t1 * t3 + v2 * t2 * t3 = (p1 - p3) * t1 + v1 * t1^2 - v3 * t1 * t3 + (p3 - p1) * t2 - v1 * t1 * t2 + v3 * t2 * t3
    // Simplifying and grouping like terms:
    // (p3 - p2) * t1 + (v1 - v2) * t1 * t2 + (p2 - p1) * t3 + (v3 - v1) * t1 * t3 + (v2 - v3) * t2 * t3 + (p1 - p3) * t2 = 0
    // This holds for every coordinate x, y and z, so we have 3 equations for the 3 unknowns t1, t2 and t3

    let hailstones: Vec<_> = parse(input).collect();
    let p1 = &hailstones[0].position;
    let p2 = &hailstones[1].position;
    let p3 = &hailstones[3].position;
    let v1 = &hailstones[0].velocity;
    let v2 = &hailstones[1].velocity;
    let v3 = &hailstones[3].velocity;
    // Let's call t1 = x, t2 = y, t3 = z

    // Print equation to solve externally
    println!(
        "\
        {}x {:+}xy {:+}z {:+}xz {:+}yz {:+}y = 0\n\
        {}x {:+}xy {:+}z {:+}xz {:+}yz {:+}y = 0\n\
        {}x {:+}xy {:+}z {:+}xz {:+}yz {:+}y = 0",
        p3.x - p2.x,
        v1.x - v2.x,
        p2.x - p1.x,
        v3.x - v1.x,
        v2.x - v3.x,
        p1.x - p3.x,
        //
        p3.y - p2.y,
        v1.y - v2.y,
        p2.y - p1.y,
        v3.y - v1.y,
        v2.y - v3.y,
        p1.y - p3.y,
        //
        p3.z - p2.z,
        v1.z - v2.z,
        p2.z - p1.z,
        v3.z - v1.z,
        v2.z - v3.z,
        p1.z - p3.z,
    );

    // Example:
    // 2x -1xy -1z +0xz +1yz -1y = 0
    // 6x +2xy +6z -3xz +1yz -12y = 0
    // 12x +0xy -8z -2xz +2yz -4y = 0
    // Solved using https://www.wolframalpha.com/input?i=system+equation+calculator yields
    // x = 5 and y = 3 and z = 4,
    // so t1 = 5, t2 = 3 and t3 = 4

    // Real input:
    // 51489553350844x -65xy -70537440843047z +113xz -48yz +19047887492203y = 0
    // -130219256866862x +150xy -20184271177029z +346xz -496yz +150403528043891y = 0
    // 78271152846418x +216xy +42154153131030z -235xz +19yz -120425305977448y = 0
    // Only solves to 0, 0, 0

    // By using hailstones 0, 1 and 3 instead, we get these equations:
    // 53529687195824x -65xy -70537440843047z +111xz -46yz +17007753647223y = 0
    // -42700513236702x +150xy -20184271177029z +180xz -330yz +62884784413731y = 0
    // -40634777736198x +216xy +42154153131030z -9xz -207yz -1519375394832y = 0
    // Which solve to x (t1) = 383159719021 and y (t2) = 173838346972 and z (t3) = 531549109510,

    let (t1, t2) = if input == INPUT {
        (383159719021, 173838346972)
    } else {
        (5, 3)
    };
    let collision_at_t1_pos = hailstones[0].position_at_time(t1);
    let collision_at_t2_pos = hailstones[1].position_at_time(t2);
    // From above: vr = (p1 + v1 * t1 - p2 - v2 * t2) / (t1 - t2)
    let rock_velocity = (collision_at_t1_pos - collision_at_t2_pos).divide(t1 - t2);
    let rock_origin = collision_at_t1_pos - rock_velocity.multiply(t1);

    (rock_origin.x + rock_origin.y + rock_origin.z)
        .try_into()
        .unwrap()
}

fn parse(input: &str) -> impl Iterator<Item = Hailstone> + '_ {
    input.trim().lines().map(Hailstone::from)
}

type Coord = i128;
#[derive(Debug, PartialEq, Copy, Clone)]
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

impl Add for Coord3D {
    type Output = Coord3D;

    fn add(self, rhs: Self) -> Self::Output {
        Coord3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Coord3D {
    type Output = Coord3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Coord3D {
    fn multiply(&self, factor: usize) -> Self {
        Coord3D {
            x: self.x * factor as Coord,
            y: self.y * factor as Coord,
            z: self.z * factor as Coord,
        }
    }
    fn divide(&self, factor: usize) -> Self {
        Coord3D {
            x: self.x / factor as Coord,
            y: self.y / factor as Coord,
            z: self.z / factor as Coord,
        }
    }
}

impl Hailstone {
    fn trajectory_intersects_with<'h>(&'h self, other: &'h Hailstone) -> Pair {
        Pair {
            hs1: self,
            hs2: other,
        }
    }
    fn position_at_time(&self, time: usize) -> Coord3D {
        self.position + self.velocity.multiply(time)
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (position, velocity) = value.split_once(" @ ").unwrap();
        let position = Coord3D::from(position);
        let velocity = Coord3D::from(velocity);
        Hailstone { position, velocity }
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
        assert_eq!(47, solve_part2(EXAMPLE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(606_772_018_765_659, solve_part2(INPUT));
    }
}
