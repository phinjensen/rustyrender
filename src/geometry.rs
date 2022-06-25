use num::{Num, ToPrimitive};
use std::{
    ops::{Add, Mul, Sub},
    result,
};

pub type Result<T> = result::Result<T, &'static str>;
const FLOAT_CONVERSION_MESSAGE: &str = "Error, couldn't convert value into f64";

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: Num + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Num + Copy> Vec2<T> {
    pub fn new() -> Self {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn from(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Vec2 {
            x: slice[0],
            y: slice[1],
        }
    }
}

impl<T: Num + Copy> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Num + Copy> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f64> for Vec2<isize> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: (self.x as f64 * rhs) as isize,
            y: (self.y as f64 * rhs) as isize,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T: Num + ToPrimitive + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + ToPrimitive + Copy> Vec3<T> {
    pub fn new() -> Self {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn from(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    pub fn cross_product(&self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn norm(&self) -> Result<f64> {
        Ok((self.x * self.x + self.y * self.y + self.z * self.z)
            .to_f64()
            .ok_or(FLOAT_CONVERSION_MESSAGE)?
            .sqrt())
    }

    pub fn normalize(&self) -> Result<Vec3<f64>> {
        *self * (1.0 / self.norm()?)
    }
}

impl<T: Num + ToPrimitive + Copy> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Num + ToPrimitive + Copy> Mul for Vec3<T> {
    type Output = Result<f64>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Ok((self.x * rhs.x + self.y * rhs.y + self.z * rhs.z)
            .to_f64()
            .ok_or(FLOAT_CONVERSION_MESSAGE)?)
    }
}

impl<T: Num + ToPrimitive + Copy> Mul<f64> for Vec3<T> {
    type Output = Result<Vec3<f64>>;

    fn mul(self, rhs: f64) -> Self::Output {
        Ok(Vec3 {
            x: (self.x.to_f64().ok_or(FLOAT_CONVERSION_MESSAGE)? * rhs),
            y: (self.y.to_f64().ok_or(FLOAT_CONVERSION_MESSAGE)? * rhs),
            z: (self.z.to_f64().ok_or(FLOAT_CONVERSION_MESSAGE)? * rhs),
        })
    }
}
