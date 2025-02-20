use rand::{self, Rng};

use crate::vector::Vec3;

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn sample_square() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-0.5..0.5);
    let y = rng.gen_range(-0.5..0.5);
    Vec3::new(x, y, 0.0)
}
