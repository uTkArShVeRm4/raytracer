use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::hittable::HittableList;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::sphere::Sphere;
use raytracer::utils::{random_f64, random_f64_in_range};
use raytracer::vector::{Point3, Vec3};

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new(random_f64(), random_f64(), random_f64());
                    let albedo = albedo * albedo;
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(
                        random_f64_in_range(0.5, 1.0),
                        random_f64_in_range(0.5, 1.0),
                        random_f64_in_range(0.5, 1.0),
                    );
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    let material_1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    //let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    //let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    //let material_left = Box::new(Dielectric::new(1.5));
    //let material_bubble = Box::new(Dielectric::new(1.0 / 1.5));
    //let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.4));
    //
    //world.add(Box::new(Sphere::new(
    //    Point3::new(0.0, -100.5, -1.0),
    //    100.0,
    //    material_ground,
    //)));
    //world.add(Box::new(Sphere::new(
    //    Point3::new(0.0, 0.0, -1.2),
    //    0.5,
    //    material_center,
    //)));
    //world.add(Box::new(Sphere::new(
    //    Point3::new(-1.0, 0.0, -1.0),
    //    0.5,
    //    material_left,
    //)));
    //// sphere inside a sphere to make it like a bubble
    //world.add(Box::new(Sphere::new(
    //    Point3::new(-1.0, 0.0, -1.0),
    //    0.4,
    //    material_bubble,
    //)));
    //world.add(Box::new(Sphere::new(
    //    Point3::new(1.0, 0.0, -1.0),
    //    0.5,
    //    material_right,
    //)));
    //
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.multithreaded = true;
    camera.render(world);
}
