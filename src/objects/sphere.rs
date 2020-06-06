use crate::materials::Material;
use crate::objects::{HitResult, Hittable};
use crate::ray::Ray;
use crate::vec3::Vector3;

use std::rc::Rc;

pub struct Sphere {
    pub center: Vector3,
    pub r: f64,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, r: f64, mat: impl Material + 'static) -> Sphere {
        let mat = Rc::new(mat);
        Sphere { center, r, mat }
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, min: f64) -> Option<HitResult> {
        let oc = ray.origin - self.center;
        let a = ray.dir.dot(&ray.dir);
        let b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - (self.r * self.r);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let tmp_s = (-b - root) / a;
            let tmp_a = (-b + root) / a;
            if tmp_s < min && tmp_a < min {
                return None;
            }

            let distance = if tmp_s < min {
                tmp_a
            } else if tmp_a < min {
                tmp_s
            } else {
                tmp_s.min(tmp_a)
            };

            let hp = ray.at(distance);
            let normal = (hp - self.center) / self.r;
            let face = ray.dir.dot(&normal) < 0.0;

            Some(HitResult::new(
                distance,
                hp,
                if face { normal } else { -normal },
                face,
                Rc::clone(&self.mat),
            ))
        } else {
            None
        }
    }
}
