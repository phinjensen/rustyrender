use num::Num;

#[derive(Debug)]
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
