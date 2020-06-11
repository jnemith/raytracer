use crate::ray::Ray;
use crate::vec3::Vector3;

pub struct Camera {
    pub origin: Vector3,
    pub horiz: Vector3,
    pub vert: Vector3,
    pub ll_corner: Vector3,
}

impl Camera {
    pub fn new(
        from: Vector3,
        lookat: Vector3,
        vup: Vector3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let height = (theta / 2.0).tan();

        let vp_height = 2.0 * height;
        let vp_width = aspect_ratio * vp_height;

        let w = (from - lookat).unit_vec();
        let u = (vup.cross(&w)).unit_vec();
        let v = w.cross(&u);

        let origin = from;
        let horiz = u * vp_width;
        let vert = v * vp_height;
        let ll_corner = origin - (horiz / 2.0) - (vert / 2.0) - w;

        Camera {
            origin,
            horiz,
            vert,
            ll_corner,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            (self.horiz * u) + (self.vert * v) + self.ll_corner - self.origin,
        )
    }
}
