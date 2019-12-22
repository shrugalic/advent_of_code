use regex::Regex;
use std::cmp::Ordering;
use std::fmt;

// For example: <x=-1, y=0, z=2>
const PATTERN: &'static str = r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>";

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Vector(i64, i64, i64);
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.0, self.1, self.2)
    }
}
impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl Vector {
    fn acceleration_from(&self, other: &Vector) -> Vector {
        Vector(
            Vector::acceleration(&self.0, &other.0),
            Vector::acceleration(&self.1, &other.1),
            Vector::acceleration(&self.2, &other.2),
        )
    }
    fn acceleration(a: &i64, b: &i64) -> i64 {
        match a.cmp(b) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }
    fn abs_sum_parts(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}
#[derive(Default, Debug, PartialEq)]
struct Moon {
    pos: Vector,
    vel: Vector,
}
impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}
impl From<&str> for Moon {
    fn from(input: &str) -> Self {
        let re = Regex::new(PATTERN).unwrap();
        if let Some(caps) = re.captures(input) {
            Moon {
                pos: Vector(
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                ),
                vel: Vector::default(),
            }
        } else {
            panic!("Unable to parse location from '{}'", input);
        }
    }
}
impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Moon {
            pos: Vector(x, y, z),
            vel: Vector::default(),
        }
    }
    fn acceleration_from(&self, moons: &[Moon]) -> Vector {
        moons
            .iter()
            .filter(|&moon| self != moon) // optional optimization: others only
            .fold(Vector::default(), |sum, moon| {
                sum + self.pos.acceleration_from(&moon.pos)
            })
    }
    fn apply_gravity(&mut self, gravity: Vector) {
        self.vel += gravity;
    }
    fn apply_velocity(&mut self) {
        self.pos += self.vel;
    }
    fn potential_energy(&self) -> usize {
        self.pos.abs_sum_parts()
    }
    fn kinetic_energy(&self) -> usize {
        self.vel.abs_sum_parts()
    }
    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
}
#[derive(Default, Debug, PartialEq)]
struct Jupiter {
    moons: Vec<Moon>,
}
impl fmt::Display for Jupiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}",
            self.moons[0], self.moons[1], self.moons[2], self.moons[3]
        )
    }
}
impl From<&str> for Jupiter {
    fn from(input: &str) -> Self {
        let moons: Vec<Moon> = input.split('\n').map(|line| Moon::from(line)).collect();
        Jupiter { moons }
    }
}
impl Jupiter {
    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocities();
    }
    fn steps(&mut self, steps: usize) {
        for _step in 0..steps {
            self.apply_gravity();
            self.apply_velocities();
        }
    }
    fn accelerations(&self) -> Vec<Vector> {
        self.moons
            .iter()
            .map(|moon| moon.acceleration_from(&self.moons))
            .collect()
    }
    fn apply_gravity(&mut self) {
        let gravities = self.accelerations();
        self.moons.iter_mut().enumerate().for_each(|(i, m)| {
            if let Some(&gravity) = gravities.get(i) {
                m.apply_gravity(gravity);
            }
        });
    }
    fn apply_velocities(&mut self) {
        self.moons.iter_mut().for_each(|m| {
            m.apply_velocity();
        });
    }
    fn total_energy(&self) -> usize {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Jupiter, Moon, Vector};

    #[test]
    fn moon_from_input() {
        assert_eq!(Moon::from("<x=-1, y=0, z=2>"), Moon::new(-1, 0, 2));
    }
    #[test]
    fn jupiter_from_input() {
        assert_eq!(
            Jupiter::from(example_1_input()),
            Jupiter {
                moons: vec![
                    Moon::new(-1, 0, 2),
                    Moon::new(2, -10, -7),
                    Moon::new(4, -8, 8),
                    Moon::new(3, 5, -1)
                ]
            }
        );
    }
    #[test]
    fn jupiter_initial_output() {
        assert_eq!(
            Jupiter::from(example_1_input()).to_string(),
            "pos=<x=-1, y=0, z=2>, vel=<x=0, y=0, z=0>
pos=<x=2, y=-10, z=-7>, vel=<x=0, y=0, z=0>
pos=<x=4, y=-8, z=8>, vel=<x=0, y=0, z=0>
pos=<x=3, y=5, z=-1>, vel=<x=0, y=0, z=0>"
        );
    }
    #[test]
    fn gravity() {
        let left = Moon::new(3, 0, 0);
        let middle = Moon::new(4, 0, 0);
        let right = Moon::new(5, 0, 0);
        let moons = [left, middle, right];

        let left_acc = moons[0].acceleration_from(&moons);
        assert_eq!(left_acc, Vector(2, 0, 0));

        let middle_acc = moons[1].acceleration_from(&moons);
        assert_eq!(middle_acc, Vector(0, 0, 0));

        let right_acc = moons[2].acceleration_from(&moons);
        assert_eq!(right_acc, Vector(-2, 0, 0));
    }
    #[test]
    fn jupiter_step() {
        let mut jupiter = Jupiter::from(example_1_input());
        jupiter.step();
        assert_eq!(
            jupiter.to_string(),
            "pos=<x=2, y=-1, z=1>, vel=<x=3, y=-1, z=-1>
pos=<x=3, y=-7, z=-4>, vel=<x=1, y=3, z=3>
pos=<x=1, y=-7, z=5>, vel=<x=-3, y=1, z=-3>
pos=<x=2, y=2, z=0>, vel=<x=-1, y=-3, z=1>"
        );
    }
    #[test]
    fn jupiter_2_steps() {
        let mut jupiter = Jupiter::from(example_1_input());
        jupiter.steps(2);
        assert_eq!(
            jupiter.to_string(),
            "pos=<x=5, y=-3, z=-1>, vel=<x=3, y=-2, z=-2>
pos=<x=1, y=-2, z=2>, vel=<x=-2, y=5, z=6>
pos=<x=1, y=-4, z=-1>, vel=<x=0, y=3, z=-6>
pos=<x=1, y=-4, z=2>, vel=<x=-1, y=-6, z=2>"
        );
    }
    #[test]
    fn example1_10_steps() {
        let mut jupiter = Jupiter::from(example_1_input());
        jupiter.steps(10);
        assert_eq!(
            jupiter.to_string(),
            "pos=<x=2, y=1, z=-3>, vel=<x=-3, y=-2, z=1>
pos=<x=1, y=-8, z=0>, vel=<x=-1, y=1, z=3>
pos=<x=3, y=-6, z=1>, vel=<x=3, y=2, z=-3>
pos=<x=2, y=0, z=4>, vel=<x=1, y=-1, z=-1>"
        );
        assert_eq!(jupiter.total_energy(), 179);
    }
    #[test]
    fn potential_energy() {
        assert_eq!(Moon::new(1, 10, -5).potential_energy(), 16);
    }
    #[test]
    fn kinetic_energy() {
        assert_eq!(
            Moon {
                pos: Vector::default(),
                vel: Vector(2, 7, -3)
            }
            .kinetic_energy(),
            12
        );
    }
    #[test]
    fn total_energy() {
        assert_eq!(
            Moon {
                pos: Vector(1, 10, -5),
                vel: Vector(2, 7, -3)
            }
            .total_energy(),
            192
        );
    }
    #[test]
    fn example2_100_steps() {
        let mut jupiter = Jupiter::from(example_2_input());
        jupiter.steps(100);
        assert_eq!(
            jupiter.to_string(),
            "pos=<x=8, y=-12, z=-9>, vel=<x=-7, y=3, z=0>
pos=<x=13, y=16, z=-3>, vel=<x=3, y=-11, z=-5>
pos=<x=-29, y=-11, z=-1>, vel=<x=-3, y=7, z=4>
pos=<x=16, y=-13, z=23>, vel=<x=7, y=1, z=1>"
        );
        assert_eq!(jupiter.total_energy(), 1940);
    }
    fn example_1_input() -> &'static str {
        "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"
    }
    fn example_2_input() -> &'static str {
        "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"
    }
    #[test]
    fn part1() {
        let mut jupiter = Jupiter::from(puzzle_input());
        jupiter.steps(1000);
        assert_eq!(jupiter.total_energy(), 14907);
    }
    fn puzzle_input() -> &'static str {
        "<x=-6, y=2, z=-9>
<x=12, y=-14, z=-4>
<x=9, y=5, z=-6>
<x=-1, y=-4, z=9>"
    }
}
