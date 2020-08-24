use crate::materials::{Material, MaterialResult};
use crate::objects::HitResult;
use crate::ray::Ray;
use crate::textures::Texture;
use crate::vec3::Vector3;

pub struct Lambertian {
    texture: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: impl Texture + 'static) -> Lambertian {
        Lambertian {
            texture: Box::new(texture),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hr: &HitResult) -> Option<MaterialResult> {
        let attenuation = self.texture.value(hr.u, hr.v, hr.hit_point);
        let target = hr.normal + Vector3::random_unit_vec();
        Some(MaterialResult::new(
            attenuation,
            Ray::new(hr.hit_point, target),
        ))
    }
}
