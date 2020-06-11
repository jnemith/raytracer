use crate::camera::Camera;
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

impl World {
    pub fn new() -> World {
        let spp = 15;
        let max_depth = 10;

        let aspect_ratio = 16. / 9.;
        let width = 500;
        let height = (width as f64 / aspect_ratio) as u32;
        let cam = Camera::new(
            Vector3::new(-2.0, 2.0, 1.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
            20.0,
            aspect_ratio,
        );

        World {
            objects: Vec::new(),
            cam,
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
        let min_dist = 0.001;
        self.objects
            .iter()
            .filter_map(|obj| obj.intersect(ray, min_dist))
            .min_by(|hr1, hr2| hr1.dist.partial_cmp(&hr2.dist).unwrap())
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
