use std::isize;

use rustyrender::drawing::line;
use rustyrender::geometry::Vec2;
use rustyrender::model::Model;
use rustyrender::tga::{Image, RGB};

const WHITE: RGB = RGB {
    r: 255,
    g: 255,
    b: 255,
};

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
            let a = Vec2 {
                x: ((v0.x + 1.) * (width as f32) / 2.) as isize,
                y: ((v0.y + 1.) * (height as f32) / 2.) as isize,
            };
            let b = Vec2 {
                x: ((v1.x + 1.) * (width as f32) / 2.) as isize,
                y: ((v1.y + 1.) * (height as f32) / 2.) as isize,
            };
            line(a, b, &mut image, WHITE)
        }
    }

    image.write_to_file("output.tga", true, true).unwrap();
}
