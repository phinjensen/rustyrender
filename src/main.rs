use std::isize;
use std::mem::swap;

use rustyrender::{
    model::Model,
    tga::{ColorSpace, Image, RGB},
};

const WHITE: RGB = RGB {
    r: 255,
    g: 255,
    b: 255,
};

fn line<T: ColorSpace + Copy>(
    mut x0: isize,
    mut y0: isize,
    mut x1: isize,
    mut y1: isize,
    image: &mut Image<T>,
    color: T,
) {
    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
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
    while x <= x1 {
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
