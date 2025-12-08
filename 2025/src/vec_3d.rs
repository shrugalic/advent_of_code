use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub(crate) struct Vec3D {
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) z: isize,
}

impl Vec3D {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Vec3D {
            x: x as isize,
            y: y as isize,
            z: z as isize,
        }
    }

    pub fn distance_to(&self, other: &Vec3D) -> f64 {
        let diff = *self - *other;
        f64::sqrt((diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f64)
    }
}

impl AddAssign for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl SubAssign for Vec3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Debug for Vec3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
