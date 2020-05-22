use crate::vec3::Vector3;
use crate::ray::Ray;
use crate::objects::{
    Hittable, HitResult
};

use image::{Rgb, RgbImage};

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
    pub cam: Camera,
    pub width: u32,
    pub height: u32,
    pub output: RgbImage,
}

pub struct Camera {
    pub origin: Vector3,
    pub horiz: Vector3,
    pub vert: Vector3,
    pub ll_corner: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let vp_height = 2.;
        let vp_width = aspect_ratio * vp_height;
        let focal_length = 1.;

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

    pub fn ray(&self, u: f64, v: f64) -> Vector3 {
        (self.horiz * u) + (self.vert * v) + self.ll_corner
    }
}

impl World {
    pub fn new() -> World {
        let aspect_ratio = 16. / 9.;
        let width = 768;
        let height = (width as f64 / aspect_ratio) as u32;

        World {
            objects: Vec::new(),
            cam: Camera::new(aspect_ratio),
            width,
            height,
            output: RgbImage::new(width, height),
        }
    }

    pub fn run(&mut self, filename: &str) {
        for (index, row) in (0..self.height).rev().enumerate() {
            for col in 0..self.width {
                let u = col as f64 / (self.width as f64);
                let v = row as f64 / (self.height as f64);
                let r = Ray::new(self.cam.origin, self.cam.ray(u, v));
    
                let color = r.get_color(&self);
                self.output.put_pixel(col, index as u32, Rgb([
                    (color.r * 255.) as u8,
                    (color.g * 255.) as u8,
                    (color.b * 255.) as u8,
                ]));
            }
        }
        self.output.save(filename).unwrap();
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