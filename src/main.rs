use raytracer::camera::Camera;
use raytracer::constants::PI;
use raytracer::hittable::HittableList;
use raytracer::sphere::Sphere;
use raytracer::vector::Point3;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.render(&world);
}
