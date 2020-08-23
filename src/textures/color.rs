use super::Texture;
use crate::vec3::Vector3;

use rgb::RGB;

pub struct SolidColor {
    pub color_value: RGB<f64>,
}

impl SolidColor {
    pub fn new(color_value: RGB<f64>) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: Vector3) -> RGB<f64> {
        self.color_value
    }
}
