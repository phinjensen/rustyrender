use core::f32;
use std::mem::swap;

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
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
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
    let (mut t0, mut t1, mut t2) = (t0, t1, t2);
    if t0.y > t1.y {
        swap(&mut t0, &mut t1);
    }
    if t0.y > t2.y {
        swap(&mut t0, &mut t2);
    }
    if t1.y > t2.y {
        swap(&mut t1, &mut t2);
    }

    let height = (t2.y - t0.y) as f32;
    for i in 0..height as isize {
        let second_half = i > (t1.y - t0.y) || t1.y == t0.y;
        let segment_height = match second_half {
            true => t2.y - t1.y,
            false => t1.y - t0.y,
        } as f32;

        let alpha = i as f32 / height;
        let beta = (i - if second_half { t1.y - t0.y } else { 0 }) as f32 / segment_height;
        let mut a = t0 + (t2 - t0) * alpha;
        #[rustfmt::skip]
        let mut b = if second_half { t1 + (t2 - t1) * beta }
                              else { t0 + (t1 - t0) * beta };

        if a.x > b.x {
            swap(&mut a, &mut b)
        }
        for x in a.x..=b.x {
            image.set(x as usize, (t0.y + i) as usize, color).unwrap();
        }
    }
}
