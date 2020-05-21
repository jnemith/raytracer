use crate::vec3::Vector3;
use crate::ray::Ray;
use crate::objects::{
    Hittable, HitResult
};

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

pub struct Camera {
    pub origin: Vector3,
    pub horiz: Vector3,
    pub vert: Vector3,
    pub ll_corner: Vector3,
}

impl Camera {
    pub fn new(vp_width: f64, vp_height: f64, focal_length: f64) -> Camera {
        let origin = Vector3::new(0., 0., 0.);
        let horiz = Vector3::new(vp_width, 0., 0.);
        let vert = Vector3::new(0., vp_height, 0.);
        Camera {
            origin,
            horiz,
            vert,
            ll_corner: origin - (horiz / 2.) - (vert / 2.) -
                Vector3::new(0., 0., focal_length),
        }
    }

    pub fn dir(&self, u: f64, v: f64) -> Vector3 {
        (self.horiz * u) + (self.vert * v) + self.ll_corner
    }
}

impl World {
    pub fn new() -> World {
        World { objects: Vec::new() }
    }
    pub fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.objects.iter()
            .filter_map(|obj| obj.intersect(ray))
            .min_by(|hr1, hr2| hr1.dist.partial_cmp(&hr2.dist).unwrap())
    }

    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }
}