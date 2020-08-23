use crate::materials::Material;
use crate::objects::{HitResult, Hittable};
use crate::ray::Ray;
use crate::vec3::Vector3;

use std::sync::Arc;

pub struct Sphere {
    pub center: Vector3,
    pub r: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3, r: f64, mat: impl Material + 'static) -> Sphere {
        let mat = Arc::new(mat);
        Sphere { center, r, mat }
    }

    pub fn get_uv(point: Vector3) -> (f64, f64) {
        use std::f64::consts::{FRAC_PI_2, PI};

        let phi = point.z.atan2(point.x);
        let theta = point.y.asin();
        (
            // u
            1.0 - ((phi + PI) / (2.0 * PI)),
            // v
            (theta + FRAC_PI_2) / PI,
        )
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

            let (u, v) = Sphere::get_uv(normal);

            Some(HitResult::new(
                distance,
                hp,
                u,
                v,
                if face { normal } else { -normal },
                face,
                Arc::clone(&self.mat),
            ))
        } else {
            None
        }
    }
}
