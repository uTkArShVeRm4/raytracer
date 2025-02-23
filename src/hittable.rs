use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{Point3, Vec3},
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            outward_normal * -1
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
    fn clone_box(&self) -> Box<dyn Hittable>;
}

#[derive(Default, Clone)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.list.push(object);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.list {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }
        hit_anything
    }
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(HittableList {
            list: self.list.iter().map(|x| x.clone_box()).collect(),
        })
    }
}
