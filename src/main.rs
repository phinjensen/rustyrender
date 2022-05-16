use std::isize;

use rustyrender::model::Model;
use rustyrender::tga::{ColorSpace, Image, RGB};

const RED: RGB = RGB { r: 255, g: 0, b: 0 };
const WHITE: RGB = RGB {
    r: 255,
    g: 255,
    b: 255,
};

fn line<T: ColorSpace + Copy + Eq>(
    mut x0: isize,
    mut y0: isize,
    mut x1: isize,
    mut y1: isize,
    image: &mut Image<T>,
    color: T,
) {
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

fn main() {
    let width = 800;
    let height = 800;
    let mut image: Image<RGB> = Image::new(width, height);
    let model = Model::from("obj/african_head.obj").unwrap();

    for i in 0..model.num_faces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vertex(face[j]);
            let v1 = model.vertex(face[(j + 1) % 3]);
            let x0 = ((v0.x + 1.) * (width as f32) / 2.) as isize;
            let y0 = ((v0.y + 1.) * (height as f32) / 2.) as isize;
            let x1 = ((v1.x + 1.) * (width as f32) / 2.) as isize;
            let y1 = ((v1.y + 1.) * (height as f32) / 2.) as isize;
            line(x0, y0, x1, y1, &mut image, WHITE)
        }
    }

    image.write_to_file("output.tga", true, true).unwrap();
}
