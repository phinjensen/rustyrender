use std::cmp::{max, min};
use std::mem::swap;

use crate::geometry::{Vec2, Vec3};
use crate::tga::{ColorSpace, Image};

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

fn bounding_box(
    points: &[Vec2<isize>],
    c_min: Vec2<isize>,
    c_max: Vec2<isize>,
) -> (Vec2<isize>, Vec2<isize>) {
    (
        Vec2 {
            x: points
                .iter()
                .chain(&[c_max])
                .map(|&p| max(c_min.x, p.x))
                .min()
                .unwrap_or(0),
            y: points
                .iter()
                .chain(&[c_max])
                .map(|&p| max(c_min.y, p.y))
                .min()
                .unwrap_or(0),
        },
        Vec2 {
            x: points
                .iter()
                .chain(&[c_min])
                .map(|&p| min(c_max.x, p.x))
                .max()
                .unwrap_or(0),
            y: points
                .iter()
                .chain(&[c_min])
                .map(|&p| min(c_max.y, p.y))
                .max()
                .unwrap_or(0),
        },
    )
}

fn barycentric(t0: Vec2<isize>, t1: Vec2<isize>, t2: Vec2<isize>, p: Vec2<isize>) -> Vec3<f32> {
    let a = Vec3 {
        x: t2.x - t0.x,
        y: t1.x - t0.x,
        z: t0.x - p.x,
    };
    let u = a.cross_product(Vec3 {
        x: t2.y - t0.y,
        y: t1.y - t0.y,
        z: t0.y - p.y,
    });
    if (u.z).abs() < 1 {
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        }
    } else {
        Vec3 {
            x: 1.0 - (u.x + u.y) as f32 / u.z as f32,
            y: u.y as f32 / u.z as f32,
            z: u.x as f32 / u.z as f32,
        }
    }
}

pub fn triangle<T: ColorSpace + Copy>(
    t0: Vec2<isize>,
    t1: Vec2<isize>,
    t2: Vec2<isize>,
    image: &mut Image<T>,
    color: T,
) {
    let bbox = bounding_box(
        &[t0, t1, t2],
        Vec2 { x: 0, y: 0 },
        Vec2 {
            x: (image.width() - 1) as isize,
            y: (image.height() - 1) as isize,
        },
    );

    let mut p = Vec2::<isize> { x: 0, y: 0 };
    for x in bbox.0.x..=bbox.1.x {
        p.x = x;
        for y in bbox.0.y..=bbox.1.y {
            p.y = y;
            let bc_screen = barycentric(t0, t1, t2, p);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set(p.x as usize, p.y as usize, color).unwrap();
        }
    }
}
