use crate::vec3::Vector3;
use crate::ray::Ray;
use crate::objects::{Hittable, HitResult};

pub struct Sphere {
    pub center: Vector3,
    pub r: f64
}

impl Sphere {
    pub fn new(center: Vector3, r: f64) -> Sphere {
        Sphere { center, r }
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitResult> {
        let oc = ray.origin - self.center;
        let a = ray.dir.dot(&ray.dir);
        let b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - (self.r * self.r);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let tmp_s = (-b - root) / a;
            let tmp_a = (-b + root) / a;
            if tmp_s < 0. && tmp_a < 0. {
                return None;
            }

            let distance = if tmp_s < 0. {
                tmp_a
            } else if tmp_a < 0. {
                tmp_s
            } else {
                tmp_s.min(tmp_a)
            };
            
            let hp = ray.at(distance);
            let normal = (hp - self.center).unit_vec();

            Some(HitResult::new(
                distance,
                hp,
                normal,
                ray.dir.dot(&normal) < 0.
            ))
        } else {
            None
        }
    }
}