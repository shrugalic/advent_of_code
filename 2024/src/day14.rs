use crate::vec_2d::Vec2D;
use std::cmp::Ordering;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../2024/input/day14.txt");

const GRID_SIZE: Vec2D = Vec2D { x: 101, y: 103 };

pub(crate) fn part1() -> usize {
    solve_part1(INPUT, &GRID_SIZE)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT, &GRID_SIZE)
}

fn solve_part1(input: &str, grid_size: &Vec2D) -> usize {
    let mut robots: Vec<_> = parse(input).collect();
    // Set to false to check if single steps also work (needed for part two)
    let all_steps_at_once = true;
    if all_steps_at_once {
        for robot in robots.iter_mut() {
            robot.step_many(100, grid_size);
        }
    } else {
        for _ in 1..=100 {
            for robot in robots.iter_mut() {
                robot.step_once(grid_size);
            }
        }
    }
    let (tl, tr, bl, br) = robots_per_quadrant(&robots, grid_size);
    tl * tr * bl * br
}

/// count robots in each quadrant, ignoring the ones exactly on a middle line
fn robots_per_quadrant(robots: &[Robot], grid_size: &Vec2D) -> (usize, usize, usize, usize) {
    let middle = Vec2D {
        x: grid_size.x / 2,
        y: grid_size.y / 2,
    };
    robots
        .iter()
        .filter_map(|r| match (r.p.x.cmp(&middle.x), r.p.y.cmp(&middle.y)) {
            (Ordering::Less, Ordering::Less) => Some((1, 0, 0, 0)), // top left
            (Ordering::Greater, Ordering::Less) => Some((0, 1, 0, 0)), // top right
            (Ordering::Less, Ordering::Greater) => Some((0, 0, 1, 0)), // bottom left
            (Ordering::Greater, Ordering::Greater) => Some((0, 0, 0, 1)), // bottom right
            (_, _) => None,
        })
        .reduce(|(a1, a2, a3, a4), (e1, e2, e3, e4)| (a1 + e1, a2 + e2, a3 + e3, a4 + e4))
        .unwrap()
}

fn solve_part2(input: &str, grid_size: &Vec2D) -> usize {
    let mut robots: Vec<_> = parse(input).collect();
    // 10'403 happens to be the period for my input, originally used higher max_steps
    let periods = robots
        .iter()
        .filter_map(|r| Robot::determine_period(r, 10_403, grid_size))
        .collect::<HashSet<_>>();
    assert_eq!(periods.len(), 1);
    let period = periods.into_iter().next().unwrap();
    let middle = Vec2D {
        x: grid_size.x / 2,
        y: grid_size.y / 2,
    };
    for i in 1..=period {
        // For a Christmas tree shape, I'd expect the
        // top left and right corners to be mostly empty
        let mut top_left = 0;
        let mut top_right = 0;
        for robot in robots.iter_mut() {
            robot.step_once(grid_size);

            if robot.p.x + robot.p.y < middle.x {
                top_left += 1;
            } else if grid_size.x - robot.p.x + robot.p.y < middle.x {
                top_right += 1;
            }
        }

        // This magic number works for my input, where there is one
        // "frame" with 21 robots in the top left and 14 in the top right
        if top_left + top_right <= 35 {
            return i;
        }
    }
    unreachable!()
}

#[expect(unused)]
fn to_string(robots: &[Robot], grid_size: &Vec2D) -> String {
    let mut chars = vec![vec![' '; grid_size.x as usize]; grid_size.y as usize];
    for robot in robots.iter() {
        chars[robot.p.y as usize][robot.p.x as usize] = '#';
    }
    chars
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Robot {
    p: Vec2D,
    v: Vec2D,
}

impl Robot {
    fn step_once(&mut self, grid_size: &Vec2D) {
        self.p.constrained_add(&self.v, grid_size);
    }

    fn step_many(&mut self, step_count: usize, grid_size: &Vec2D) {
        self.p += self.v * Vec2D::new(step_count, step_count);
        self.p.constrain_to(grid_size);
    }

    fn determine_period(&self, max_steps: usize, grid_size: &Vec2D) -> Option<usize> {
        let start_pos = self.p;
        let mut p = start_pos;
        for step in 1..=max_steps {
            p.constrained_add(&self.v, grid_size);
            if p == start_pos {
                return Some(step);
            }
        }
        panic!("Unable to determine period for {self:?} in {max_steps} steps");
    }
}

impl From<&str> for Robot {
    fn from(line: &str) -> Self {
        let nums: Vec<isize> = line
            .split(&['=', ',', ' '])
            .filter_map(|s| s.parse().ok())
            .collect();
        Robot {
            p: Vec2D {
                x: nums[0],
                y: nums[1],
            },
            v: Vec2D {
                x: nums[2],
                y: nums[3],
            },
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
    input.trim().lines().map(Robot::from)
}

/// Operations constrained to a grid with modulo arithmetic
trait ConstrainedGridOps {
    fn constrain_to(&mut self, grid_size: &Vec2D);
    fn constrained_add(&mut self, addend: &Vec2D, grid_size: &Vec2D);
}

impl ConstrainedGridOps for Vec2D {
    fn constrain_to(&mut self, grid_size: &Vec2D) {
        self.x = self.x.rem_euclid(grid_size.x);
        self.y = self.y.rem_euclid(grid_size.y);
    }

    fn constrained_add(&mut self, v: &Vec2D, grid_size: &Vec2D) {
        *self += *v;
        self.constrain_to(grid_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_GRID_SIZE: Vec2D = Vec2D { x: 11, y: 7 };

    const EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_part1_example() {
        assert_eq!(12, solve_part1(EXAMPLE, &EXAMPLE_GRID_SIZE));
    }

    #[test]
    fn test_part1() {
        assert_eq!(228_410_028, solve_part1(INPUT, &GRID_SIZE));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8_258, solve_part2(INPUT, &GRID_SIZE));
    }
}
