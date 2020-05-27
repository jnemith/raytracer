use crate::vec3::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, dir: Vector3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (self.dir * t)
    }
}