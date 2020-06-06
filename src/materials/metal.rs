use crate::materials::{Material, MaterialResult};
use crate::objects::HitResult;
use crate::ray::Ray;
use crate::vec3::Vector3;

use rgb::RGB;

pub struct Metal {
    color: RGB<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: RGB<f64>, f: f64) -> Metal {
        let fuzz = f.min(1.);
        Metal { color, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hr: &HitResult) -> Option<MaterialResult> {
        let attenuation = RGB::new(self.color.r, self.color.g, self.color.b);
        let reflected = ray_in.dir.unit_vec().reflect(&hr.normal);
        let scattered = Ray::new(
            hr.hit_point,
            reflected + Vector3::random_in_unit_sphere() * self.fuzz,
        );
        if scattered.dir.dot(&hr.normal) > 0. {
            Some(MaterialResult::new(attenuation, scattered))
        } else {
            None
        }
    }
}
