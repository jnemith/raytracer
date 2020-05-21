pub mod sphere;

use crate::ray::Ray;
use crate::vec3::Vector3;

pub trait Hittable {
    fn intersect(&self, ray: &Ray) -> Option<HitResult>;
}

pub struct HitResult {
    pub dist: f64,
    pub hit_point: Vector3,
    pub normal: Vector3,

    // True: ray is outside. False: ray is inside
    pub face: bool,
}

impl HitResult {
    pub fn new(dist: f64, hit_point: Vector3, normal: Vector3, face: bool) -> HitResult {
        HitResult { dist, hit_point, normal, face }
    }
}