use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Vec2D {
    pub(crate) x: isize,
    pub(crate) y: isize,
}
impl Vec2D {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Vec2D {
            x: x as isize,
            y: y as isize,
        }
    }
    pub(crate) fn crosswise_neighbors(&self) -> impl Iterator<Item = Vec2D> + use<'_> {
        [
            Vec2D { x: 1, y: 0 },
            Vec2D { x: -1, y: 0 },
            Vec2D { x: 0, y: 1 },
            Vec2D { x: 0, y: -1 },
        ]
        .into_iter()
        .map(|dir| dir + *self)
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

impl Debug for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
