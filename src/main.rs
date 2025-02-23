use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::hittable::HittableList;
use raytracer::material::{Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::vector::Point3;

//fn degrees_to_radians(degrees: f64) -> f64 {
//    degrees * PI / 180.0
//}

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.0));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.threads = 8;
    camera.render(world);
}
