use crate::materials::{Material, MaterialResult};
use crate::objects::HitResult;
use crate::ray::Ray;
use crate::vec3::Vector3;

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
    fn scatter(&self, _ray_in: &Ray, hr: &HitResult) -> MaterialResult {
        //
    }
}