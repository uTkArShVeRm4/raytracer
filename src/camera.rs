use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils::sample_square;
use crate::vector::{Point3, Vec3};
#[derive(Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,

    image_height: u32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let mut c = Camera::default();
        c.aspect_ratio = 1.0;
        c.image_width = 100;
        c
    }
    pub fn render<T>(&mut self, world: &T)
    where
        T: Hittable,
    {
        self.initialize();
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            // Add log for number of rows left to render, not in stdout
            eprintln!("Rows remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray.color(world);
                }
                print!("{}", (pixel_color * self.pixel_sample_scale).to_string());
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a ray from the camera's center point shooting towards a randomly sampled point
        // near i,j

        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin = self.center.clone();
        let ray_direction = &pixel_sample - &ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn initialize(&mut self) {
        let image_height: u32 = (self.image_width as f64 / self.aspect_ratio).floor() as u32;
        self.image_height = image_height.max(1);

        self.pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);

        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * self.image_width as f64 / self.image_height as f64;

        // Calculate horizontal and vertical vectors along viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate horizontal and vertical delta vectors for each pixel
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // Calculate location of upper left corner pixel
        let viewport_upper_left = &self.center
            - &Vec3::new(0.0, 0.0, focal_length)
            - &viewport_u / 2.0
            - &viewport_v / 2.0;

        self.pixel00_loc =
            &viewport_upper_left + &((&self.pixel_delta_u + &self.pixel_delta_v) * 0.5);
    }
}
