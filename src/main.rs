mod materials;
mod objects;
mod ray;
mod vec3;
mod world;

use crate::materials::{
    lambertian::Lambertian,
    metal::Metal,
};
use crate::objects::sphere::Sphere;
use crate::vec3::Vector3;
use crate::world::World;

use rgb::RGB;

fn main() {
    let filename = "out.png";
    let mut world = World::new();
    eprintln!(
        "{} x {}\nOutput file: {}",
        world.width, world.height, filename
    );

    world.add(Sphere::new(
        Vector3::new(0., 0., -1.),
        0.5,
        Lambertian::new(RGB::new(0.7, 0.3, 0.3)),
    ));
    world.add(Sphere::new(
        Vector3::new(0., -100.5, -1.),
        100.,
        Lambertian::new(RGB::new(0.8, 0.8, 0.)),
    ));

    world.add(Sphere::new(
        Vector3::new(1., 0., -1.),
        0.5,
        Metal::new(RGB::new(0.8, 0.6, 0.2), 0.0)
    ));
    world.add(Sphere::new(
        Vector3::new(-1., 0., -1.),
        0.5,
        Metal::new(RGB::new(0.8, 0.8, 0.8), 1.)
    ));

    world.run(filename);
    eprintln!("Done");
}
