use core::f32;

use crate::geometry::Vec2;
use crate::tga::{ColorSpace, Image, RGB};

pub fn line<T: ColorSpace + Copy>(
    a: &Vec2<isize>,
    b: &Vec2<isize>,
    image: &mut Image<T>,
    color: T,
) {
    let mut x0 = a.x;
    let mut y0 = a.y;
    let mut x1 = b.x;
    let mut y1 = b.y;
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    let mut x = x0;
    loop {
        if x > x1 {
            break;
        }

        if steep {
            image.set(y as usize, x as usize, color).ok();
        } else {
            image.set(x as usize, y as usize, color).ok();
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
        x += 1;
    }
}

const RED: RGB = RGB { r: 255, g: 0, b: 0 };
const GREEN: RGB = RGB { r: 0, g: 255, b: 0 };

pub fn triangle(
    //<T: ColorSpace + Copy>(
    t0: Vec2<isize>,
    t1: Vec2<isize>,
    t2: Vec2<isize>,
    image: &mut Image<RGB>,
    color: RGB,
) {
    let mut t0 = t0;
    let mut t1 = t1;
    let mut t2 = t2;
    if t0.y > t1.y {
        (t0, t1) = (t1, t0);
    }
    if t0.y > t2.y {
        (t0, t2) = (t2, t0);
    }
    if t1.y > t2.y {
        (t1, t2) = (t2, t1);
    }
    let height = (t2.y - t0.y) as f32;
    let segment_height = (t1.y - t0.y) as f32;
    for y in t0.y..t1.y {
        let alpha = (y - t0.y) as f32 / height;
        let beta = (y - t0.y) as f32 / segment_height;
        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = t0 + (t1 - t0) * beta;
        if a.x > b.x {
            (a, b) = (b, a);
        }
        for x in a.x..=b.x {
            image.set(x as usize, y as usize, color).unwrap();
        }
    }
    let segment_height = (t2.y - t1.y) as f32;
    for y in t1.y..=t2.y {
        let alpha = (y - t0.y) as f32 / height;
        let beta = (y - t1.y) as f32 / segment_height;
        let mut a = t0 + (t2 - t0) * alpha;
        let mut b = t1 + (t2 - t1) * beta;
        if a.x > b.x {
            (a, b) = (b, a);
        }
        for x in a.x..=b.x {
            image.set(x as usize, y as usize, color).unwrap();
        }
    }
}
