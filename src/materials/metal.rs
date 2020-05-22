use crate::materials::{Material, MaterialResult};
use crate::objects::HitResult;
use crate::ray::Ray;

use rgb::RGB;

pub struct Metal {
    color: RGB<f64>,
}

impl Metal {
    pub fn new(color: RGB<f64>) -> Metal {
        Metal { color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hr: &HitResult) -> Option<MaterialResult> {
        let attenuation = RGB::new(self.color.r, self.color.g, self.color.b);
        let reflected = ray_in.dir.unit_vec().reflect(&hr.normal);
        let scattered = Ray::new(hr.hit_point, reflected);
        Some(MaterialResult::new(attenuation, scattered))
    }
}