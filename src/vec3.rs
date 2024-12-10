#![allow(clippy::cast_precision_loss)]
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::util::rand;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }
    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.z
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x < s && self.y < s && self.z < s
    }
}

#[inline]
pub fn random_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(rand(min, max), rand(min, max), rand(min, max))
}

#[inline]
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * *n;
    r_out_perp + r_out_parallel
}

#[inline]
pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

#[inline]
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        v1.z() * v2.x() - v1.x() * v2.z(),
        v1.x() * v2.y() - v1.y() * v2.x(),
    )
}

#[inline]
pub fn rand_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand(-1.0, 1.0), rand(-1.0, 1.0), 0.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}

#[inline]
pub fn random_vec() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
        let lenth_squared = p.length_squared();
        if 1e-160 < lenth_squared && lenth_squared <= 1.0 {
            return p / lenth_squared.sqrt();
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        Vec3::new(
            self.x / rhs as f32,
            self.y / rhs as f32,
            self.z / rhs as f32,
        )
    }
}
impl Div<u32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: u32) -> Self::Output {
        Vec3::new(
            self.x / rhs as f32,
            self.y / rhs as f32,
            self.z / rhs as f32,
        )
    }
}
impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
impl Div for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;
    #[inline]
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

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
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
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[test]
fn test_div() {
    let v1 = Vec3::new(2.0, 2.0, 2.0);
    let v2 = v1;
    let result = Vec3::new(1.0, 1.0, 1.0);

    assert_eq!(v1 / 2.0, result);
    assert_eq!(v1 / v2, result);
}

#[test]
fn test_mul() {
    let v1 = Vec3::new(2.0, 2.0, 2.0);
    let v2 = v1;
    let result = Vec3::new(4.0, 4.0, 4.0);

    assert_eq!(v1 * 2.0, result);
    assert_eq!(v1 * v2, result);
}

#[test]
fn test_sub() {
    let v1 = Vec3::new(2.0, 2.0, 2.0);
    let v2 = v1;
    let result = Vec3::new(0.0, 0.0, 0.0);

    assert_eq!(v1 - v2, result);
}
#[test]
fn test_add() {
    let v1 = Vec3::new(2.0, 2.0, 2.0);
    let v2 = v1;
    let result = Vec3::new(4.0, 4.0, 4.0);

    assert_eq!(v1 + v2, result);
}
