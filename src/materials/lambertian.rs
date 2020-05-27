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
        let target = hr.hit_point + hr.normal + Vector3::random_unit_vec();
        Some(MaterialResult::new(attenuation, Ray::new(hr.hit_point, target - hr.hit_point)))
    }
}
