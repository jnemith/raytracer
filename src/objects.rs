pub mod sphere;

use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vector3;

use std::sync::Arc;

pub trait Hittable: Sync + Send {
    fn intersect(&self, ray: &Ray, min: f64) -> Option<HitResult>;
}

pub struct HitResult {
    pub dist: f64,
    pub hit_point: Vector3,
    pub u: f64,
    pub v: f64,
    pub normal: Vector3,

    // True: ray is outside. False: ray is inside
    pub face: bool,
    pub mat: Arc<dyn Material>,
}

pub struct HitList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HitResult {
    pub fn new(
        dist: f64,
        hit_point: Vector3,
        u: f64,
        v: f64,
        normal: Vector3,
        face: bool,
        mat: Arc<dyn Material>,
    ) -> HitResult {
        HitResult {
            dist,
            hit_point,
            u,
            v,
            normal,
            face,
            mat,
        }
    }
}

impl HitList {
    pub fn new() -> Self {
        HitList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HitList {
    fn intersect(&self, ray: &Ray, min: f64) -> Option<HitResult> {
        self.objects
            .iter()
            .filter_map(|obj| obj.intersect(ray, min))
            .min_by(|hr1, hr2| hr1.dist.partial_cmp(&hr2.dist).unwrap())
    }
}
