use std::isize;

use rustyrender::drawing::triangle;
use rustyrender::geometry::{self, Vec2, Vec3};
use rustyrender::model::Model;
use rustyrender::tga::{Image, RGBA};

fn main() -> geometry::Result<()> {
    let width = 800;
    let height = 800;
    let mut image: Image<RGBA> = Image::new(width, height);
    let model = Model::from("obj/african_head.obj").unwrap();

    let light_dir: Vec3<f64> = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    for i in 0..model.num_faces() {
        let face = model.face(i);
        let (screen_coords, world_coords): (Vec<Vec2<isize>>, Vec<Vec3<f64>>) = (0..3)
            .map(|j| {
                let world_coords = model.vertex(face[j]);
                (
                    Vec2::<isize> {
                        x: ((world_coords.x + 1.0) * width as f64 / 2.0) as isize,
                        y: ((world_coords.y + 1.0) * width as f64 / 2.0) as isize,
                    },
                    world_coords,
                )
            })
            .unzip();
        let n =
            (world_coords[2] - world_coords[1]).cross_product(world_coords[1] - world_coords[0]);
        let n = n.normalize().unwrap();
        let intensity = (n * light_dir).unwrap();
        if intensity > 0.0 {
            triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                &mut image,
                RGBA {
                    b: (intensity * 255.0) as u8,
                    g: (intensity * 255.0) as u8,
                    r: (intensity * 255.0) as u8,
                    a: 255,
                },
            )
        }
    }

    image.write_to_file("output.tga", true, true).unwrap();
    Ok(())
}
