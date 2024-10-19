use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        v1.z() * v2.x() - v1.x() * v2.z(),
        v1.x() * v2.y() - v1.y() * v2.x(),
    )
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        Vec3::new(
            self.x / rhs as f32,
            self.y / rhs as f32,
            self.z / rhs as f32,
        )
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: i32) -> Self::Output {
        Vec3::new(
            self.x * rhs as f32,
            self.y * rhs as f32,
            self.y * rhs as f32,
        )
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

mod test_vec3 {
    use super::*;

    #[test]
    fn test_div() {
        let v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = v1.clone();
        let result = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(v1 / 2.0, result);
        assert_eq!(v1 / v2, result);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = v1.clone();
        let result = Vec3::new(4.0, 4.0, 4.0);

        assert_eq!(v1 * 2.0, result);
        assert_eq!(v1 * v2, result);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = v1.clone();
        let result = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(v1 - v2, result);
    }
    #[test]
    fn test_add() {
        let v1 = Vec3::new(2.0, 2.0, 2.0);
        let v2 = v1.clone();
        let result = Vec3::new(4.0, 4.0, 4.0);

        assert_eq!(v1 + v2, result);
    }
}
