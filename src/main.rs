use rustyrender::tga::{Image, RGB};

const RED: RGB = RGB { r: 255, g: 0, b: 0 };

fn main() {
    let mut image: Image<RGB> = Image::new(100, 100);
    image.set(52, 41, RED).unwrap();
    image.write_to_file("output.tga", true, true).unwrap();
}
