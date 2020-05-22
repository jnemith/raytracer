use crate::materials::{Material, MaterialResult};
use crate::objects::HitResult;
use crate::ray::Ray;
use crate::vec3::Vector3;

use rgb::RGB;

pub struct Lambertian {
    color: RGB<f64>,
}

impl Lambertian {
    pub fn new(color: RGB<f64>) -> Lambertian {
        Lambertian { color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hr: &HitResult) -> Option<MaterialResult> {
        let attenuation = RGB::new(self.color.r, self.color.g, self.color.b);
        let scatter_dir = hr.normal + Vector3::random_unit_vec();
        let scattered = Ray::new(hr.hit_point, scatter_dir);
        Some(MaterialResult::new(attenuation, scattered))
    }
}
