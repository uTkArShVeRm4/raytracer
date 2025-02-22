#![allow(dead_code)]

use crate::utils::{random_f64, random_f64_in_range};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_f64(),
            y: random_f64(),
            z: random_f64(),
        }
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_f64_in_range(min, max),
            y: random_f64_in_range(min, max),
            z: random_f64_in_range(min, max),
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            let l = p.length_squared();
            if l > 1e-160 && l <= 1.0 {
                return p / l.sqrt();
            }
        }
    }

    pub fn random_unit_vector_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_spher = Vec3::random_unit_vector();
        if on_spher.dot(normal) > 0.0 {
            return on_spher;
        }
        on_spher * -1.0
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn length(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl<T> std::ops::Mul<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    fn mul(self, other: T) -> Vec3 {
        let other_f64: f64 = other.into();
        Vec3 {
            x: self.x * other_f64,
            y: self.y * other_f64,
            z: self.z * other_f64,
        }
    }
}
impl<T> std::ops::Mul<T> for &Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    fn mul(self, other: T) -> Vec3 {
        let other_f64: f64 = other.into();
        Vec3 {
            x: self.x * other_f64,
            y: self.y * other_f64,
            z: self.z * other_f64,
        }
    }
}

impl<T> std::ops::MulAssign<T> for Vec3
where
    T: Into<f64>,
{
    fn mul_assign(&mut self, other: T) {
        let other_f64: f64 = other.into();
        self.x *= other_f64;
        self.y *= other_f64;
        self.z *= other_f64;
    }
}
impl<T> std::ops::Div<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    fn div(self, other: T) -> Vec3 {
        let other_f64: f64 = other.into();
        Vec3 {
            x: self.x / other_f64,
            y: self.y / other_f64,
            z: self.z / other_f64,
        }
    }
}
impl<T> std::ops::Div<T> for &Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    fn div(self, other: T) -> Vec3 {
        let other_f64: f64 = other.into();
        Vec3 {
            x: self.x / other_f64,
            y: self.y / other_f64,
            z: self.z / other_f64,
        }
    }
}

impl<T> std::ops::DivAssign<T> for Vec3
where
    T: Into<f64>,
{
    fn div_assign(&mut self, other: T) {
        let other_f64: f64 = other.into();
        self.x /= other_f64;
        self.y /= other_f64;
        self.z /= other_f64;
    }
}
