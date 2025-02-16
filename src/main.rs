use raytracer::constants::PI;
use raytracer::hittable::HittableList;
use raytracer::sphere::Sphere;
use raytracer::{
    ray::Ray,
    vector::{Point3, Vec3},
};
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let mut image_height: u32 = (image_width as f64 / aspect_ratio).floor() as u32;
    image_height = image_height.max(1);

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Calculate horizontal and vertical vectors along viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate horizontal and vertical delta vectors for each pixel
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate location of upper left corner pixel
    let viewport_upper_left =
        &camera_center - &Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;

    let pixel_loc = &viewport_upper_left + &((&pixel_delta_u + &pixel_delta_v) * 0.5);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        // Add log for number of rows left to render, not in stdout
        eprintln!("Rows remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                &pixel_loc + &(&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);
            let ray_direction = &pixel_center - &camera_center;
            let ray = Ray::new(camera_center.clone(), ray_direction);
            print!("{}", ray.color(&world).to_string());
        }
    }
}
