use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random(-1., 1.);
            if p.dot(&p) >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere = Vector3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_unit_vec() -> Vector3 {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0., 2. * std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1., 1.);
        let r = (1. - (z * z)).sqrt();
        Vector3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn unit_vec(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        } / self.length()
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn reflect(&self, n: &Vector3) -> Vector3 {
        let r = Vector3::new(self.x, self.y, self.z);
        r - (*n * r.dot(n) * 2.)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        if other == 0. {
            panic!("Cannot divide by zero!");
        }

        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
