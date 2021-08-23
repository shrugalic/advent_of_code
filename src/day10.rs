use std::cmp::Ordering;
use std::f64::consts::PI;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Point(pub(crate) usize, pub(crate) usize);
#[derive(Eq, PartialEq, Debug, Clone)]
struct Dir(i64, i64);

impl Dir {
    fn len(&self) -> f64 {
        ((self.0 as f64).powi(2) + (self.1 as f64).powi(2)).sqrt()
    }
    fn is_same_direction_as(&self, other: &Dir) -> bool {
        (self.angle() - other.angle()).abs() < f64::EPSILON
    }
    fn angle(&self) -> f64 {
        // In a typical coordinate system, the positive_angle starts at 0 in the east,
        // increases counter-clockwise, and is calculated as atan2(y, x).
        // Our angle however starts in the north and increases clockwise,
        // so we use -y as x and x as y to translate this.
        let y = self.0 as f64;
        let x = -1.0 * self.1 as f64;
        y.atan2(x)
    }
    /// Convert angle ranges from (-PI, +PI] to (0, 2PI] to simplify comparisons
    fn positive_angle(&self) -> f64 {
        let angle = self.angle();
        if angle < 0.0 {
            angle + 2.0 * PI
        } else {
            angle
        }
    }
}

/// Order by distance/length of direction vector
impl Ord for Dir {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_len = self.len();
        let other_len = other.len();
        if my_len < other_len {
            Ordering::Less
        } else if my_len > other_len {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Dir {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    fn dir_to(&self, other: &Point) -> Dir {
        Dir(
            other.0 as i64 - self.0 as i64,
            other.1 as i64 - self.1 as i64,
        )
    }
    fn detectable_others(&self, asteroids: &Vec<Point>) -> Vec<Point> {
        // Save a copy of the other asteroids
        let mut others: Vec<(Point, Dir)> = asteroids
            .iter()
            .filter_map(|point| {
                if self != point {
                    Some((point.clone(), self.dir_to(&point)))
                } else {
                    None
                }
            })
            //            .cloned()
            .collect();
        // Sort by distance to point, closest first (the Ordering of Dir is according to length)
        others.sort_by_key(|(_point, dir)| dir.clone());

        // Pick those that are not obscured by another asteroid
        let mut detectable_others: Vec<(Point, Dir)> = vec![];
        others.iter().for_each(|(candidate, c_dir)| {
            if !detectable_others
                .iter()
                .any(|(_detected, d_dir)| d_dir.is_same_direction_as(c_dir))
            {
                detectable_others.push((candidate.clone(), c_dir.clone()));
            }
        });

        // Remove direction, keep and return only the points
        detectable_others
            .into_iter()
            .map(|(point, _dir)| point)
            .collect()
    }
}

pub(crate) struct MonitoringStation {
    loc: Point,
    pub(crate) count: usize,
    asteroids: Vec<Point>,
}
impl From<&str> for MonitoringStation {
    fn from(input: &str) -> Self {
        let asteroids: Vec<Point> = input
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        // println!("({}, {}) = {}", x, y, c);
                        if c == '#' {
                            Some(Point(x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect();
        if let Some((count, loc)) = asteroids
            .iter()
            .map(|loc| (loc.detectable_others(&asteroids).len(), loc.clone()))
            .max_by_key(|(count, _loc)| *count)
        {
            MonitoringStation {
                loc,
                count,
                asteroids,
            }
        } else {
            panic!("There are no asteroids to serve as monitoring station");
        }
    }
}
impl MonitoringStation {
    pub(crate) fn vaporized(&mut self) -> Vec<Point> {
        let mut vaporized: Vec<Point> = vec![];
        // println!("loc = {:?}", self.loc);
        while self.asteroids.len() > 1 {
            let mut detectables = self.loc.detectable_others(&self.asteroids);
            // Sort by positive angle to get into vaporizing order
            detectables.sort_by(|a, b| {
                self.loc
                    .dir_to(a)
                    .positive_angle()
                    .partial_cmp(&self.loc.dir_to(b).positive_angle())
                    .unwrap()
            });
            // vaporize
            detectables.into_iter().for_each(|target| {
                if let Some(index) = self.asteroids.iter().position(|p| *p == target) {
                    self.asteroids.remove(index);
                }
                vaporized.push(target);
            });
        }
        vaporized
    }
}

pub(crate) fn day10_puzzle_input() -> &'static str {
    "#..#.#.#.######..#.#...##
##.#..#.#..##.#..######.#
.#.##.#..##..#.#.####.#..
.#..##.#.#..#.#...#...#.#
#...###.##.##..##...#..#.
##..#.#.#.###...#.##..#.#
###.###.#.##.##....#####.
.#####.#.#...#..#####..#.
.#.##...#.#...#####.##...
######.#..##.#..#.#.#....
###.##.#######....##.#..#
.####.##..#.##.#.#.##...#
##...##.######..##..#.###
...###...#..#...#.###..#.
.#####...##..#..#####.###
.#####..#.#######.###.##.
#...###.####.##.##.#.##.#
.#.#.#.#.#.##.#..#.#..###
##.#.####.###....###..##.
#..##.#....#..#..#.#..#.#
##..#..#...#..##..####..#
....#.....##..#.##.#...##
.##..#.#..##..##.#..##..#
.##..#####....#####.#.#.#
#..#..#..##...#..#.#.#.##"
}

#[cfg(test)]
mod tests {
    use super::{Dir, MonitoringStation, Point};
    use crate::day10::day10_puzzle_input;

    // positive_angle
    #[test]
    fn positive_angles() {
        assert!(Dir(0, -1).positive_angle() < Dir(1, -1).positive_angle());
        assert!(Dir(1, -1).positive_angle() < Dir(1, 0).positive_angle());
        assert!(Dir(1, 0).positive_angle() < Dir(1, 1).positive_angle());
        assert!(Dir(1, 1).positive_angle() < Dir(0, 1).positive_angle());
        assert!(Dir(0, 1).positive_angle() < Dir(-1, 1).positive_angle());
        assert!(Dir(-1, 1).positive_angle() < Dir(-1, 0).positive_angle());
        assert!(Dir(-1, 0).positive_angle() < Dir(-1, -1).positive_angle());
        // The last positive_angle is set to just slightly counter-clockwise of vertical,
        // because the positive_angle of vertical would be 0.0, whereas this is 1.999… * PI
        assert!(Dir(-1, -1).positive_angle() < Dir(-1, i64::MIN).positive_angle());
    }
    // is_same_direction_as
    #[test]
    fn non_null_opposite_direction() {
        assert!(!Dir(2, 1).is_same_direction_as(&Dir(-2, -1)));
    }
    // asteroids
    #[test]
    fn single_asteroid() {
        assert_eq!(MonitoringStation::from("#").asteroids, vec![Point(0, 0)]);
    }
    #[test]
    fn two_asteroids() {
        assert_eq!(
            MonitoringStation::from("##").asteroids,
            vec![Point(0, 0), Point(1, 0)]
        );
    }
    #[test]
    fn three_asteroids() {
        assert_eq!(
            MonitoringStation::from("###").asteroids,
            vec![Point(0, 0), Point(1, 0), Point(2, 0)]
        );
    }
    #[test]
    fn example_1_asteroids() {
        assert_eq!(
            MonitoringStation::from(example1()).asteroids,
            vec![
                Point(1, 0),
                Point(4, 0),
                Point(0, 2),
                Point(1, 2),
                Point(2, 2),
                Point(3, 2),
                Point(4, 2),
                Point(4, 3),
                Point(3, 4),
                Point(4, 4)
            ]
        );
    }
    fn example1() -> &'static str {
        ".#..#
.....
#####
....#
...##"
    }

    // detectable_others (from a point)
    #[test]
    fn count_1_detectable_asteroids_from_first_of_three_asteroids() {
        assert_eq!(
            Point(0, 0)
                .detectable_others(&MonitoringStation::from("###").asteroids)
                .len(),
            1
        );
    }
    #[test]
    fn count_2_detectable_asteroids_from_middle_of_three_asteroids() {
        assert_eq!(
            Point(1, 0)
                .detectable_others(&MonitoringStation::from("###").asteroids)
                .len(),
            2
        );
    }
    #[test]
    fn count_1_detectable_asteroids_from_last_of_three_asteroids() {
        assert_eq!(
            Point(2, 0)
                .detectable_others(&MonitoringStation::from("###").asteroids)
                .len(),
            1
        );
    }
    #[test]
    fn count_2_detectable_asteroids_with_four_asteroids() {
        assert_eq!(
            Point(0, 0)
                .detectable_others(
                    &MonitoringStation::from(
                        "###
#.."
                    )
                    .asteroids
                )
                .len(),
            2
        );
    }
    #[test]
    fn count_3_detectable_asteroids_with_four_asteroids() {
        assert_eq!(
            Point(0, 1)
                .detectable_others(
                    &MonitoringStation::from(
                        "###
#..",
                    )
                    .asteroids
                )
                .len(),
            3
        );
    }
    #[test]
    fn count_example1_asteroids_5() {
        assert_eq!(
            Point(4, 2)
                .detectable_others(&MonitoringStation::from(example1()).asteroids)
                .len(),
            5
        );
    }
    #[test]
    fn count_example1_asteroids_6() {
        assert_eq!(
            Point(0, 2)
                .detectable_others(&MonitoringStation::from(example1()).asteroids)
                .len(),
            6
        );
    }
    #[test]
    fn count_example1_asteroids_7() {
        assert_eq!(
            Point(4, 3)
                .detectable_others(&MonitoringStation::from(example1()).asteroids)
                .len(),
            7
        );
    }
    #[test]
    fn count_example1_asteroids_8() {
        assert_eq!(
            Point(3, 4)
                .detectable_others(&MonitoringStation::from(example1()).asteroids)
                .len(),
            8
        );
    }

    // max detectable asteroids
    #[test]
    fn no_detectable_asteroids_with_single_asteroid() {
        assert_eq!(MonitoringStation::from("#").count, 0);
    }
    #[test]
    fn max_1_detectable_asteroids_with_two_asteroids() {
        assert_eq!(MonitoringStation::from("##").count, 1);
    }
    #[test]
    fn max_1_detectable_asteroids_with_three_asteroids() {
        assert_eq!(MonitoringStation::from("###").count, 2);
    }
    #[test]
    fn max_3_detectable_asteroids_with_four_asteroids() {
        assert_eq!(
            MonitoringStation::from(
                "###
#.."
            )
            .count,
            3
        );
    }
    #[test]
    fn max_detectable_asteroids_example1() {
        assert_eq!(MonitoringStation::from(example1()).count, 8);
    }
    #[test]
    fn max_detectable_asteroids_larger_example1() {
        assert_eq!(
            MonitoringStation::from(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            )
            .count,
            33
        );
    }
    #[test]
    fn max_detectable_asteroids_larger_example2() {
        assert_eq!(
            MonitoringStation::from(
                "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            )
            .count,
            35
        );
    }
    #[test]
    fn max_detectable_asteroids_larger_example3() {
        assert_eq!(
            MonitoringStation::from(
                ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            )
            .count,
            41
        );
    }
    #[test]
    fn max_detectable_asteroids_larger_example4() {
        assert_eq!(MonitoringStation::from(large_example()).count, 210);
    }
    fn large_example() -> &'static str {
        ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
    }
    #[test]
    fn part2_example() {
        let mut station = MonitoringStation::from(
            ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##",
        );
        assert_eq!(
            station.vaporized(),
            vec![
                Point(8, 1),
                Point(9, 0),
                Point(9, 1),
                Point(10, 0),
                Point(9, 2),
                Point(11, 1),
                Point(12, 1),
                Point(11, 2),
                Point(15, 1),
                Point(12, 2),
                Point(13, 2),
                Point(14, 2),
                Point(15, 2),
                Point(12, 3),
                Point(16, 4),
                Point(15, 4),
                Point(10, 4),
                Point(4, 4),
                Point(2, 4),
                Point(2, 3),
                Point(0, 2),
                Point(1, 2),
                Point(0, 1),
                Point(1, 1),
                Point(5, 2),
                Point(1, 0),
                Point(5, 1),
                Point(6, 1),
                Point(6, 0),
                Point(7, 0),
                Point(8, 0),
                Point(10, 1),
                Point(14, 0),
                Point(16, 1),
                Point(13, 3),
                Point(14, 3)
            ]
        );
    }
    #[test]
    fn part2_large_example() {
        let mut station = MonitoringStation::from(large_example());
        let vaporized = station.vaporized();
        assert_eq!(vaporized[0], Point(11, 12));
        assert_eq!(vaporized[1], Point(12, 1));
        assert_eq!(vaporized[2], Point(12, 2));
        assert_eq!(vaporized[9], Point(12, 8));
        assert_eq!(vaporized[19], Point(16, 0));
        assert_eq!(vaporized[49], Point(16, 9));
        assert_eq!(vaporized[99], Point(10, 16));
        assert_eq!(vaporized[198], Point(9, 6));
        assert_eq!(vaporized[199], Point(8, 2));
        assert_eq!(vaporized[200], Point(10, 9));
        assert_eq!(vaporized[298], Point(11, 1));
    }
    #[test]
    fn part2() {
        let mut station = MonitoringStation::from(day10_puzzle_input());
        let vaporized = station.vaporized();
        assert_eq!(vaporized[199], Point(8, 15));
    }
    #[test]
    fn max_detectable_asteroids_part1() {
        assert_eq!(MonitoringStation::from(day10_puzzle_input()).count, 253);
    }
}
