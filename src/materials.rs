pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::objects::HitResult;
use crate::ray::Ray;

use rgb::RGB;

pub struct MaterialResult {
    pub attenuation: RGB<f64>,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hr: &HitResult) -> Option<MaterialResult>;
}

impl MaterialResult {
    pub fn new(attenuation: RGB<f64>, scattered: Ray) -> MaterialResult {
        MaterialResult {
            attenuation,
            scattered,
        }
    }
}
