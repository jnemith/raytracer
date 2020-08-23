use crate::materials::{Material, MaterialResult};
use crate::objects::HitResult;
use crate::ray::Ray;
use crate::textures::Texture;
use crate::vec3::Vector3;

use rgb::RGB;

pub struct Lambertian {
    color: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(color: impl Texture + 'static) -> Lambertian {
        Lambertian {
            color: Box::new(color),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hr: &HitResult) -> Option<MaterialResult> {
        let attenuation = self.color.value(hr.u, hr.v, hr.hit_point);
        let target = hr.normal + Vector3::random_unit_vec();
        Some(MaterialResult::new(
            attenuation,
            Ray::new(hr.hit_point, target),
        ))
    }
}
