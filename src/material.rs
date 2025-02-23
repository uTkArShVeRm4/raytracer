use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::Vec3;

pub struct DefaultMaterial {}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn clone_box(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

impl Material for DefaultMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(DefaultMaterial {})
    }
}

impl Default for Box<dyn Material> {
    fn default() -> Box<dyn Material> {
        Box::new(DefaultMaterial {})
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = &hit_record.normal + &Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Lambertian {
            albedo: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = &ray_in.direction().reflect(&hit_record.normal);
        let reflected = reflected.normalize() + (Vec3::random_unit_vector() * self.fuzz);
        *scattered = Ray::new(hit_record.p, reflected);
        *attenuation = self.albedo;
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Metal {
            albedo: self.albedo,
            fuzz: self.fuzz,
        })
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Dielectric { ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hit_record.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = ray_in.direction().normalize();

        let cos_theta = (unit_direction * -1.0).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract {
            direction = unit_direction.reflect(&hit_record.normal);
        } else {
            direction = unit_direction.refract(&hit_record.normal, ri);
        }

        *scattered = Ray::new(hit_record.p, direction);
        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(Dielectric {
            ref_idx: self.ref_idx,
        })
    }
}
