use super::Texture;
use crate::vec3::Vector3;

use image::{DynamicImage, GenericImageView, ImageError};
use rgb::RGB;

pub struct ImageTexture {
    img: Result<DynamicImage, ImageError>,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img = image::open(filename);

        Self { img }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: Vector3) -> RGB<f64> {
        let img_inner = if let Ok(img) = &self.img {
            img
        } else {
            return RGB::new(0.0, 1.0, 1.0);
        };
        let width = img_inner.width() as f64;
        let height = img_inner.height() as f64;

        let u = u.max(0.0).min(1.0);
        let v = 1.0 - v.max(0.0).min(1.0);

        let mut i = u * width;
        let mut j = v * height;

        if i >= width {
            i = width - 1.0;
        }
        if j >= height {
            j = height - 1.0;
        }
        let color_scale: f64 = 1.0 / 255.0;
        let px = img_inner.get_pixel(i as u32, j as u32);

        RGB::new(
            px[0] as f64 * color_scale,
            px[1] as f64 * color_scale,
            px[2] as f64 * color_scale,
        )
    }
}
