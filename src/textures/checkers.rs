use super::Texture;
use crate::vec3::Vector3;

use rgb::RGB;

pub struct Checkers {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl Checkers {
    pub fn new(odd: impl Texture + 'static, even: impl Texture + 'static) -> Self {
        Self {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }
}

impl Texture for Checkers {
    fn value(&self, u: f64, v: f64, point: Vector3) -> RGB<f64> {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
