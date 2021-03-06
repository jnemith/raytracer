use crate::camera::Camera;
use crate::objects::{HitList, Hittable};
use crate::ray::Ray;
use crate::vec3::Vector3;

use image::{Rgb, RgbImage};
use rand::Rng;
use rayon::prelude::*;
use rgb::RGB;

pub struct World {
    pub objects: HitList,
    pub cam: Camera,
    pub width: u32,
    pub height: u32,
    pub output: RgbImage,
    pub spp: u32,
    pub max_depth: u32,
}

impl World {
    pub fn new() -> World {
        let spp = 25;
        let max_depth = 10;

        let aspect_ratio = 16. / 9.;
        let width = 426;
        let height = (width as f64 / aspect_ratio) as u32;
        let from = Vector3::new(5.0, 1.0, 3.0);
        let to = Vector3::new(0.0, 2.0, 0.0);

        let cam = Camera::new(
            from,
            to,
            Vector3::new(0.0, 1.0, 0.0),
            70.0,
            aspect_ratio,
            0.1,
            10.0,
        );

        World {
            objects: HitList::new(),
            cam,
            width,
            height,
            output: RgbImage::new(width, height),
            spp,
            max_depth,
        }
    }

    pub fn run(&mut self, filename: &str) {
        let colors: Vec<Vec<RGB<f64>>> = (0..self.height)
            .into_par_iter()
            .rev()
            .map(|row| {
                (0..self.width)
                    .into_par_iter()
                    .map(|col| {
                        let mut rng = rand::thread_rng();
                        let mut color = RGB::new(0., 0., 0.);

                        for _ in 0..self.spp {
                            let u = (col as f64 + rng.gen_range(0., 1.)) / (self.width as f64);
                            let v = (row as f64 + rng.gen_range(0., 1.)) / (self.height as f64);
                            let r = self.cam.ray(u, v);
                            color = color + self.get_color(&r, 0);
                        }
                        color
                    })
                    .collect()
            })
            .collect();

        for (row, vecrow) in colors.iter().enumerate() {
            for (col, color) in vecrow.iter().enumerate() {
                self.write_color(*color, col as u32, row as u32);
            }
        }
        self.output.save(filename).unwrap();
    }

    pub fn get_color(&self, ray: &Ray, depth: u32) -> RGB<f64> {
        if depth >= self.max_depth {
            return RGB::new(0., 0., 0.);
        }

        if let Some(hr) = self.objects.intersect(&ray, 0.001) {
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

    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.add(obj);
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
