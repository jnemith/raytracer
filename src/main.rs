mod world;
mod vec3;
mod ray;
mod objects;

use crate::world::World;
use crate::vec3::Vector3;
use crate::objects::sphere::Sphere;

fn main() {
    let filename = "out.png";
    let mut world = World::new();
    println!("{} x {}\nOutput file: {}", world.width, world.height, filename);

    world.add(Sphere::new(
        Vector3::new(0., 0., -1.), 0.5
    ));
    world.add(Sphere::new(
        Vector3::new(0., -50., -1.), 49.5
    ));

    world.run(filename);
    eprintln!("Done");
}