use crate::ray::Ray;
use crate::vec3::Vector3;

use rand::Rng;

pub struct Camera {
    pub origin: Vector3,
    pub horiz: Vector3,
    pub vert: Vector3,
    pub ll_corner: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        from: Vector3,
        lookat: Vector3,
        vup: Vector3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let height = (theta / 2.0).tan();

        let vp_height = 2.0 * height;
        let vp_width = aspect_ratio * vp_height;

        let w = (from - lookat).unit_vec();
        let u = (vup.cross(&w)).unit_vec();
        let v = w.cross(&u);

        let origin = from;
        let horiz = u * vp_width * focus_dist;
        let vert = v * vp_height * focus_dist;
        let ll_corner = origin - (horiz / 2.0) - (vert / 2.0) - (w * focus_dist);

        Camera {
            origin,
            horiz,
            vert,
            ll_corner,
            u, v, w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vector3 = random_in_unit_disc() * self.lens_radius;
        let offset: Vector3 = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            (self.horiz * s) + (self.vert * t) + self.ll_corner - self.origin - offset,
        )
    }
}

pub fn random_in_unit_disc() -> Vector3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            0.0,
        );

        if p.dot(&p) >= 1.0 {
            continue;
        }
        return p;
    }
}