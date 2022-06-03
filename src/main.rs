use std::isize;

use rustyrender::drawing::triangle;
use rustyrender::geometry::Vec2;
use rustyrender::tga::{Image, RGB};

const GREEN: RGB = RGB { r: 0, g: 255, b: 0 };
const WHITE: RGB = RGB {
    r: 255,
    g: 255,
    b: 255,
};
const RED: RGB = RGB { r: 255, g: 0, b: 0 };

fn main() {
    let width = 200;
    let height = 200;
    let mut image: Image<RGB> = Image::new(width, height);

    let t0: Vec<Vec2<isize>> = vec![Vec2::from(10, 70), Vec2::from(50, 160), Vec2::from(70, 80)];
    let t1: Vec<Vec2<isize>> = vec![Vec2::from(180, 50), Vec2::from(150, 1), Vec2::from(70, 180)];
    let t2: Vec<Vec2<isize>> = vec![
        Vec2::from(180, 150),
        Vec2::from(120, 160),
        Vec2::from(130, 180),
    ];

    triangle(t0[0], t0[1], t0[2], &mut image, GREEN);
    triangle(t1[0], t1[1], t1[2], &mut image, WHITE);
    triangle(t2[0], t2[1], t2[2], &mut image, RED);

    image.write_to_file("output.tga", true, true).unwrap();
}
