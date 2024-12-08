use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Position {
    pub(crate) x: isize,
    pub(crate) y: isize,
}
impl Position {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
