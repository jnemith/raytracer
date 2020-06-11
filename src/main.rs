mod camera;
mod materials;
mod objects;
mod ray;
mod vec3;
mod world;

use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::objects::sphere::Sphere;
use crate::vec3::Vector3;
use crate::world::World;

use rand::Rng;
use rgb::RGB;

fn main() {
    let filename = "out.png";
    let mut world = World::new();
    eprintln!(
        "{} x {}\nOutput file: {}",
        world.width, world.height, filename
    );

    world.add(Sphere::new(
        Vector3::new(0., -1000.0, 0.0),
        1000.0,
        Lambertian::new(RGB::new(0.5, 0.5, 0.5)),
    ));

    world.add(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));

    world.add(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(RGB::new(0.1, 0.1, 0.9)),
    ));

    world.add(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(RGB::new(0.7, 0.6, 0.5), 0.0),
    ));

    world.run(filename);
    eprintln!("Done");
}
