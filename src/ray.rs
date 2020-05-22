use crate::vec3::Vector3;
use crate::world::World;
use rgb::RGB;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, dir: Vector3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (self.dir * t)
    }

    pub fn get_color(&self, world: &World, depth: u32) -> RGB<f64> {
        if depth >= world.max_depth {
            return RGB::new(0., 0., 0.);
        }

        if let Some(hr) = world.hit(&self) {
            // let mr = hr.mat.scatter(&self, &hr);
            if let Some(mr) = hr.mat.scatter(&self, &hr) {
                let tmp = mr.scattered.get_color(world, depth + 1);
                RGB::new(
                    tmp.r * mr.attenuation.r,
                    tmp.g * mr.attenuation.g,
                    tmp.b * mr.attenuation.b,
                )
            } else {
                RGB::new(0., 0., 0.)
            }
        } else {
            let unit = self.dir.unit_vec();
            let t = 0.5 * (unit.y + 1.);
            RGB::new(1., 1., 1.) * (1. - t) + RGB::new(0.5, 0.75, 1.) * t
        }
    }
}
