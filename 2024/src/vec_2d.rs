use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
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
    pub(crate) fn left_neighbor(&self) -> Self {
        Vec2D {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub(crate) fn right_neighbor(&self) -> Self {
        Vec2D {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub(crate) fn above_neighbor(&self) -> Self {
        Vec2D {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub(crate) fn below_neighbor(&self) -> Self {
        Vec2D {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub(crate) fn left_above_neighbor(&self) -> Self {
        Vec2D {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    pub(crate) fn right_above_neighbor(&self) -> Self {
        Vec2D {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    pub(crate) fn left_below_neighbor(&self) -> Self {
        Vec2D {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    pub(crate) fn right_below_neighbor(&self) -> Self {
        Vec2D {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    #[expect(unused)]
    pub(crate) fn left(&self) -> Self {
        Vec2D { x: -1, y: 0 }
    }
    #[expect(unused)]
    pub(crate) fn right() -> Self {
        Vec2D { x: 1, y: 0 }
    }
    #[expect(unused)]
    pub(crate) fn up() -> Self {
        Vec2D { x: 0, y: -1 }
    }
    #[expect(unused)]
    pub(crate) fn down() -> Self {
        Vec2D { x: 0, y: 1 }
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

impl Debug for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
