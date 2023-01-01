use std::ops::{Add, Neg};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point2<T> {
    pub x: T,
    pub y: T
}

impl<T> Point2<T> {
    pub fn from_xy(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T> Add for Point2<T> where T: Add<T, Output = T> {
    type Output = Point2<T>;
    fn add(self, rhs: Point2<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<&Self> for Point2<T> where T: Add<T, Output = T> + Copy {
    type Output = Point2<T>;
    fn add(self, rhs: &Point2<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<Point2<T>> for &Point2<T> where T: Add<T, Output = T> + Copy {
    type Output = Point2<T>;
    fn add(self, rhs: Point2<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Add<Self> for &Point2<T> where T: Add<T, Output = T> + Copy {
    type Output = Point2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Neg for &Point2<T> where T: Neg<Output = T> + Copy {
    type Output = Point2<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Neg for Point2<T> where T: Neg<Output = T> {
    type Output = Point2<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    pub fn from_xyz(x: T, y: T, z:T) -> Self {
        Self {x, y, z}
    }
}

impl<T> Add for Point3<T> where T: Add<T, Output = T> {
    type Output = Point3<T>;
    fn add(self, rhs: Point3<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<&Self> for Point3<T> where T: Add<T, Output = T> + Copy {
    type Output = Point3<T>;
    fn add(self, rhs: &Point3<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<Point3<T>> for &Point3<T> where T: Add<T, Output = T> + Copy {
    type Output = Point3<T>;
    fn add(self, rhs: Point3<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<Self> for &Point3<T> where T: Add<T, Output = T> + Copy {
    type Output = Point3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Neg for &Point3<T> where T: Neg<Output = T> + Copy {
    type Output = Point3<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Neg for Point3<T> where T: Neg<Output = T> {
    type Output = Point3<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_point2() {
        let a = Point2 {x: 1, y: 1};
        let b = Point2 {x: -1, y: 1};
        let c = &a + &b;
        assert_eq!(c, a.clone() + &b);
        assert_eq!(c, &a + b.clone());
        assert_eq!(c, a.clone() + b.clone());
    }

    #[test]
    fn test_add_point3() {
        let a = Point3 {x: 1, y: 1, z: 1};
        let b = Point3 {x: -1, y: 1, z: 2};
        let c = &a + &b;
        assert_eq!(c, a.clone() + &b);
        assert_eq!(c, &a + b.clone());
        assert_eq!(c, a.clone() + b.clone());
    }
}