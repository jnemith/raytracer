use crate::objects::{HitResult, Hittable};
use crate::ray::Ray;
use crate::vec3::Vector3;

use image::{Rgb, RgbImage};
use rand::Rng;
use rgb::RGB;

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
    pub cam: Camera,
    pub width: u32,
    pub height: u32,
    pub output: RgbImage,
    pub spp: u32,
    pub max_depth: u32,
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
            ll_corner: origin - (horiz / 2.) - (vert / 2.) - Vector3::new(0., 0., focal_length),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            (self.horiz * u) + (self.vert * v) + self.ll_corner,
        )
    }
}

impl World {
    pub fn new() -> World {
        let spp = 15;
        let max_depth = 10;

        let aspect_ratio = 16. / 9.;
        let width = 500;
        let height = (width as f64 / aspect_ratio) as u32;

        World {
            objects: Vec::new(),
            cam: Camera::new(aspect_ratio),
            width,
            height,
            output: RgbImage::new(width, height),
            spp,
            max_depth,
        }
    }

    pub fn run(&mut self, filename: &str) {
        let mut rng = rand::thread_rng();
        for (index, row) in (0..self.height).rev().enumerate() {
            for col in 0..self.width {
                let mut color = RGB::new(0., 0., 0.);

                for _ in 0..self.spp {
                    let u = (col as f64 + rng.gen_range(0., 1.)) / (self.width as f64);
                    let v = (row as f64 + rng.gen_range(0., 1.)) / (self.height as f64);
                    let r = self.cam.ray(u, v);
                    color = color + self.get_color(&r, 0);
                }
                self.write_color(color, col, index as u32);
            }
        }
        self.output.save(filename).unwrap();
    }

    pub fn get_color(&self, ray: &Ray, depth: u32) -> RGB<f64> {
        if depth >= self.max_depth {
            return RGB::new(0., 0., 0.);
        }

        if let Some(hr) = self.hit(&ray) {
            if let Some(mr) = hr.mat.scatter(&ray, &hr) {
                let tmp = self.get_color(&mr.scattered, depth + 1);
                RGB::new(
                    tmp.r * mr.attenuation.r,
                    tmp.g * mr.attenuation.g,
                    tmp.b * mr.attenuation.b,
                )
            } else {
                RGB::new(0., 0., 0.)
            }
        } else {
            let unit = ray.dir.unit_vec();
            let t = 0.5 * (unit.y + 1.);
            RGB::new(1., 1., 1.) * (1. - t) + RGB::new(0.5, 0.75, 1.) * t
        }
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.objects
            .iter()
            .filter_map(|obj| {
                if let Some(hit) = obj.intersect(ray) {
                    if hit.dist > 0.001 {
                        return Some(hit);
                    }
                }
                None
            })
            .min_by(|hr1, hr2| {
                hr1.dist.partial_cmp(&hr2.dist).unwrap()
            })
    }

    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }

    fn write_color(&mut self, color: RGB<f64>, col: u32, row: u32) {
        let scale = 1. / (self.spp as f64);
        let scaled = RGB::new(
            (color.r * scale).sqrt(),
            (color.g * scale).sqrt(),
            (color.b * scale).sqrt(),
        );

        self.output.put_pixel(
            col,
            row,
            Rgb([
                (clamp(scaled.r, 0., 0.999) * 256.) as u8,
                (clamp(scaled.g, 0., 0.999) * 256.) as u8,
                (clamp(scaled.b, 0., 0.999) * 256.) as u8,
            ]),
        );
    }
}

fn clamp(num: f64, min: f64, max: f64) -> f64 {
    if num > max {
        return max;
    }
    if num < min {
        return min;
    }
    num
}