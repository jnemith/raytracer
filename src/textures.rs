pub mod checkers;
pub mod color;

use crate::vec3::Vector3;

use rgb::RGB;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, point: Vector3) -> RGB<f64>;
}
