use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
