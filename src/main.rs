mod camera;
mod materials;
mod objects;
mod ray;
mod textures;
mod vec3;
mod world;

use crate::materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::objects::sphere::Sphere;
use crate::textures::color::SolidColor;
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
        Lambertian::new(SolidColor::new((0.5, 0.5, 0.5).into())),
    ));

    world.add(Sphere::new(
        Vector3::new(-4.0, 2.0, 0.0),
        2.0,
        Metal::new(RGB::new(0.7, 0.6, 0.5), 0.0),
    ));

    world.add(Sphere::new(
        Vector3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(SolidColor::new((0.2, 0.6, 0.2).into())),
    ));

    world.add(Sphere::new(
        Vector3::new(3.0, 1.0, 0.0),
        1.0,
        Lambertian::new(SolidColor::new((0.1, 0.2, 0.7).into())),
    ));

    world.run(filename);
    eprintln!("Done");
}

fn random_scene() -> World {
    let mut world = World::new();

    let ground = Lambertian::new(SolidColor::new((0.5, 0.5, 0.5).into()));
    world.add(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, ground));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_color();
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(SolidColor::new(albedo)),
                    ));
                } else if choose_mat < 0.95 {
                    let cvec = Vector3::random(0.5, 1.0);
                    let albedo = RGB::new(cvec.x, cvec.y, cvec.z);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));

    world.add(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(SolidColor::new((0.5, 0.3, 0.1).into())),
    ));

    world.add(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(RGB::new(0.7, 0.6, 0.5), 0.0),
    ));

    world
}

pub fn random_color() -> RGB<f64> {
    let mut rng = rand::thread_rng();
    RGB::new(
        rng.gen::<f64>() * rng.gen::<f64>(),
        rng.gen::<f64>() * rng.gen::<f64>(),
        rng.gen::<f64>() * rng.gen::<f64>(),
    )
}
