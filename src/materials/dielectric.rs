use crate::materials::{Material, MaterialResult};
use crate::objects::HitResult;
use crate::ray::Ray;
use crate::vec3::Vector3;

use rand::Rng;
use rgb::RGB;

pub struct Dielectric {
    ref_idx: f64,
}

#[allow(dead_code)]
impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hr: &HitResult) -> Option<MaterialResult> {
        let etai_over_etat = if hr.face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_dir = ray_in.dir.unit_vec();
        let cos_theta = (-unit_dir.dot(&hr.normal)).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
        let reflected = unit_dir.reflect(&hr.normal);
        if etai_over_etat * sin_theta > 1.0 {
            Some(MaterialResult::new(
                RGB::new(1.0, 1.0, 1.0),
                Ray::new(hr.hit_point, reflected),
            ))
        } else {
            let mut rng = rand::thread_rng();
            if rng.gen::<f64>() < schlick(cos_theta, self.ref_idx) {
                Some(MaterialResult::new(
                    RGB::new(1.0, 1.0, 1.0),
                    Ray::new(hr.hit_point, reflected),
                ))
            } else {
                let refracted = refract(unit_dir, hr.normal, etai_over_etat);
                Some(MaterialResult::new(
                    RGB::new(1.0, 1.0, 1.0),
                    Ray::new(hr.hit_point, refracted),
                ))
            }
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let d = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = d * d;
    r0 + ((1.0 - r0) * (1.0 - cosine).powf(5.0))
}

fn refract(uv: Vector3, normal: Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = -uv.dot(&normal);
    let r_out_parallel: Vector3 = (uv + (normal * cos_theta)) * etai_over_etat;
    let length_squared = r_out_parallel.dot(&r_out_parallel);
    let r_out_perp: Vector3 = normal * -(1.0 - length_squared).sqrt();
    r_out_parallel + r_out_perp
}
