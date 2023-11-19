use std::cmp::max;
use std::ops::{AddAssign, RangeInclusive};

const INPUT: &str = include_str!("../input/day17.txt");

pub(crate) fn day17_part1() -> isize {
    Probe::from(INPUT).highest_point()
}

pub(crate) fn day17_part2() -> usize {
    Probe::from(INPUT).trajectory_count()
}

#[derive(Debug, PartialEq)]
struct Probe {
    target_area: RangeInclusive<Pair>,
}

impl Probe {
    fn highest_point(&self) -> isize {
        self.target_trajectory().0
    }
    fn trajectory_count(&self) -> usize {
        self.target_trajectory().1
    }
    fn target_trajectory(&self) -> (isize, usize) {
        let mut max_ys = vec![];
        let mut target_velocity_count = 0;

        let y_velocity_range = self.y_velocity_range();
        for x in self.x_velocity_range() {
            for y in y_velocity_range.clone() {
                if let Some(max_y) = self.simulate_trajectory(x, y) {
                    max_ys.push(max_y);
                    target_velocity_count += 1;
                }
            }
        }
        (*max_ys.iter().max().unwrap(), target_velocity_count)
    }
    fn simulate_trajectory(&self, x: isize, y: isize) -> Option<isize> {
        let mut position = Pair::new(0, 0);
        let mut velocity = Pair::new(x, y);

        let mut max_y = isize::MIN;
        while !(position.is_past(&self.target_area)
            || velocity.cannot_reach(&self.target_area, &position))
        {
            max_y = max(max_y, position.y);
            position += velocity;
            velocity.x = max(velocity.x - 1, 0);
            velocity.y -= 1;
            if position.is_within(&self.target_area) {
                return Some(max_y);
            }
        }
        None
    }
    fn x_velocity_range(&self) -> RangeInclusive<isize> {
        // During each step, the x-position increases by the x-velocity,
        // after which the x-velocity decreases by 1, to a minimum of 0.
        // So the furthest reachable x-position is bound by the sum
        // 0 + 1 + 2 + 3 + … + n-1 + n, which is equal to n * (n + 1) / 2.
        // The minimal x-velocity should reach the left border of target area:
        let mut min_x = 1;
        while Probe::reachable(min_x) < self.target_area.start().x {
            min_x += 1;
        }
        // The maximal x-velocity is bound by the right border of the target area,
        // which in this case would be reached in the first step
        let max_x = self.target_area.end().x;

        min_x..=max_x
    }
    fn reachable(x: isize) -> isize {
        x * (x + 1) / 2
    }
    fn y_velocity_range(&self) -> RangeInclusive<isize> {
        // Similarly to the x velocity range, the minimal y-velocity is bound by the
        // lower border of the target area, which would be reached in one step in this case.
        let min_y = self.target_area.start().y;

        // To reach the highest point possible, the y-velocity should be as large as possible.
        // Note: For positive y-velocities the distances above the x-axis cancel themselves out.
        // For example an initial y-velocity of 5 will become 4, 3, 2, 1 and 0, and will have
        // traveled to y = 5 + 4 + 3 + 2 + 1 = 15. After this the y-velocities will become
        // -1, -2, -3, -4, -5 and thus return to y = 0 after a number of steps that is equal to
        // twice the initial y-velocity. The next step is what is important here:
        // The upper bound for the initial y-velocity depends on the distance of the lower end
        // of the target area to the x-axis, because with an initial y-velocity equal to that,
        // a number of steps would have an y above 0, then fall back to 0, and finally fall
        // right onto the lower border in the next step.
        let max_y = self.target_area.start().y.abs();

        min_y..=max_y
    }
}
impl From<&str> for Probe {
    // Example input: target area: x=20..30, y=-10..-5
    fn from(input: &str) -> Self {
        let range_from = |range: &str| -> RangeInclusive<isize> {
            let (start, end) = range.split_once("..").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        };

        let (x, y) = input
            .trim()
            .trim_start_matches("target area: x=")
            .split_once(", y=")
            .map(|(x, y)| (range_from(x), range_from(y)))
            .unwrap();

        Probe {
            target_area: Pair::new(*x.start(), *y.start())..=Pair::new(*x.end(), *y.end()),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Pair {
    x: isize,
    y: isize,
}
impl Pair {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn is_past(&self, target_area: &RangeInclusive<Pair>) -> bool {
        self.x > target_area.end().x || self.y < target_area.start().y
    }
    fn is_within(&self, target_area: &RangeInclusive<Pair>) -> bool {
        (target_area.start().x..=target_area.end().x).contains(&self.x)
            && (target_area.start().y..=target_area.end().y).contains(&self.y)
    }
    fn cannot_reach(&self, target_area: &RangeInclusive<Pair>, position: &Pair) -> bool {
        self.x == 0 && position.x < target_area.start().x
    }
}
impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part1_example() {
        assert_eq!(45, Probe::from(EXAMPLE).highest_point());
    }

    #[test]
    fn part1() {
        assert_eq!(5565, day17_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(112, Probe::from(EXAMPLE).trajectory_count());
    }

    #[test]
    fn part2() {
        assert_eq!(2118, day17_part2());
    }
}
