use rand::{self, Rng};
use std::f64::consts::PI;

use crate::vector::Vec3;

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn sample_square() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-0.5..0.5);
    let y = rng.gen_range(-0.5..0.5);
    Vec3::new(x, y, 0.0)
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
