use crate::color::Color;
use crate::constants::INFINITY;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::vector::{Point3, Vec3};

#[derive(Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }

    pub fn color<T>(&self, depth: u32, world: &T) -> Color
    where
        T: Hittable,
    {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut hit_record = HitRecord::default();

        if world.hit(self, Interval::new(0.001, INFINITY), &mut hit_record) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if hit_record
                .material
                .scatter(self, &hit_record, &mut attenuation, &mut scattered)
            {
                return attenuation * scattered.color(depth - 1, world);
            }
        }
        let unit_direction = &self.direction.normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
