mod world;
mod vec3;
mod ray;
mod objects;

use crate::world::{Camera, World};
use crate::vec3::Vector3;
use crate::ray::Ray;
use crate::objects::sphere::Sphere;

use image::{Rgb, RgbImage};

fn main() {
    let filename = "out.png";

    let aspect_ratio = 16. / 9.;
    let width = 768;
    let height = (width as f64 / aspect_ratio) as u32;
    println!("{} x {}\nOutput file: {}", width, height, filename);

    let vp_height = 2.;
    let vp_width = aspect_ratio * vp_height;
    let focal_length = 1.;
    let cam = Camera::new(vp_width, vp_height, focal_length);

    let mut world = World::new();

    world.add(Sphere::new(
        Vector3::new(0., 0., -2.), 0.5
    ));
    world.add(Sphere::new(
        Vector3::new(0.35, -0.3, -1.), 0.15
    ));
    world.add(Sphere::new(
        Vector3::new(0., -50., -1.), 49.5
    ));

    let mut img = RgbImage::new(width, height);

    for (index, row) in (0..height).rev().enumerate() {
        // eprintln!("Scanlines remaining: {}", row+1);
        for col in 0..width {
            let u = col as f64 / (width as f64);
            let v = row as f64 / (height as f64);
            let r = Ray::new(cam.origin, cam.dir(u, v));

            let color = r.get_color(&world);
            img.put_pixel(col, index as u32, Rgb([
                (color.r * 255.) as u8,
                (color.g * 255.) as u8,
                (color.b * 255.) as u8,
            ]));
            // println!("{} {} {}",
            //     (color.r * 255.) as u32,
            //     (color.g * 255.) as u32,
            //     (color.b * 255.) as u32,
            // );
        }
    }
    eprintln!("Done");
    img.save(filename).unwrap();
}