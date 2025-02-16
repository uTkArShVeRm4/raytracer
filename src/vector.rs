#![allow(dead_code)]

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
