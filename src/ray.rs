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
        Ray {
            origin,
            dir,
        }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (self.dir * t)
    }

    pub fn get_color(&self, world: &World) -> RGB<f64> {
        if let Some(hr) = world.hit(&self) {
            // eprintln!("{:?}", hr);
            let mut surface_normal = hr.normal;
            if !hr.face {
                surface_normal = -surface_normal;
            }

            RGB::new(
                1. + surface_normal.x,
                1. + surface_normal.y,
                1. + surface_normal.z,
            ) * 0.5
        } else {
            let unit = self.dir.unit_vec();
            let t = 0.5 * (unit.y + 1.);
            RGB::new(1., 1., 1.) * (1. - t) + RGB::new(0.5, 0.75, 1.) * t
        }
    }
}