use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub(crate) struct Vec2D {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl Vec2D {
    pub(crate) const WEST: Vec2D = Vec2D { x: -1, y: 0 };
    pub(crate) const EAST: Vec2D = Vec2D { x: 1, y: 0 };
    pub(crate) const NORTH: Vec2D = Vec2D { x: 0, y: -1 };
    pub(crate) const SOUTH: Vec2D = Vec2D { x: 0, y: 1 };

    pub fn new(x: usize, y: usize) -> Self {
        Vec2D {
            x: x as isize,
            y: y as isize,
        }
    }
    pub fn all_neighbors(&self) -> impl Iterator<Item = Vec2D> + use<'_> {
        [
            Vec2D { x: -1, y: -1 },
            Vec2D { x: -1, y: 0 },
            Vec2D { x: -1, y: 1 },
            Vec2D { x: 0, y: 1 },
            Vec2D { x: 0, y: -1 },
            Vec2D { x: 1, y: -1 },
            Vec2D { x: 1, y: 0 },
            Vec2D { x: 1, y: 1 },
        ]
        .into_iter()
        .map(|dir| dir + *self)
    }
    pub fn crosswise_neighbors(&self) -> impl Iterator<Item = Vec2D> + use<'_> {
        [
            Vec2D { x: 1, y: 0 },
            Vec2D { x: -1, y: 0 },
            Vec2D { x: 0, y: 1 },
            Vec2D { x: 0, y: -1 },
        ]
        .into_iter()
        .map(|dir| dir + *self)
    }
    pub fn left_neighbor(&self) -> Self {
        Vec2D {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right_neighbor(&self) -> Self {
        Vec2D {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn above_neighbor(&self) -> Self {
        Vec2D {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn below_neighbor(&self) -> Self {
        Vec2D {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left_above_neighbor(&self) -> Self {
        Vec2D {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    pub fn right_above_neighbor(&self) -> Self {
        Vec2D {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    pub fn left_below_neighbor(&self) -> Self {
        Vec2D {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    pub fn right_below_neighbor(&self) -> Self {
        Vec2D {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    pub fn turn_cw(&mut self) {
        (self.x, self.y) = (self.y, -self.x);
    }
    pub fn turn_ccw(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }
    pub fn x_and_y_increments(&self) -> (Vec<Vec2D>, Vec<Vec2D>) {
        let x_parts = match self.x.cmp(&0) {
            Ordering::Greater => vec![Vec2D::EAST; self.x.unsigned_abs()],
            Ordering::Less => vec![Vec2D::WEST; self.x.unsigned_abs()],
            Ordering::Equal => vec![],
        };
        let y_parts = match self.y.cmp(&0) {
            Ordering::Greater => vec![Vec2D::SOUTH; self.y.unsigned_abs()],
            Ordering::Less => vec![Vec2D::NORTH; self.y.unsigned_abs()],
            Ordering::Equal => vec![],
        };
        (x_parts, y_parts)
    }
}
impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Add for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl SubAssign for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl Sub for Vec2D {
    type Output = Vec2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
