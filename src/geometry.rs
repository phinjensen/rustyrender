use num::Num;
use std::ops::{Add, Mul, Sub};

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

impl Mul<f32> for Vec2<isize> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: (self.x as f32 * rhs) as isize,
            y: (self.y as f32 * rhs) as isize,
        }
    }
}

#[derive(Debug)]
pub struct Vec3<T: Num + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Copy> Vec3<T> {
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
}
