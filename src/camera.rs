use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils::degrees_to_radians;
use crate::utils::sample_square;
use crate::vector::{Point3, Vec3};

use parallel_executor::parallel_iterator::ParallelIterator;

#[derive(Debug, Default, Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32, // Number of ray bounces allowed
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub multithreaded: bool,

    image_height: u32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let mut c = Camera::default();
        c.aspect_ratio = 1.0;
        c.image_width = 100;
        c.max_depth = 10;
        c.look_from = Point3::new(0.0, 0.0, 0.0);
        c.look_at = Point3::new(0.0, 0.0, -1.0);
        c.vup = Vec3::new(0.0, 1.0, 0.0);
        c
    }
    pub fn render<T>(mut self, world: T)
    where
        T: Hittable + Send + Clone + 'static, // Clone for each thread, Send for parallel execution
    {
        self.initialize();

        // Print header
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        if self.multithreaded {
            // Flatten the 2D loop into a 1D iterator
            let pixel_indices: Vec<(u32, u32)> = (0..self.image_height)
                .flat_map(|j| (0..self.image_width).map(move |i| (i, j)))
                .collect();

            // Parallel processing of pixels
            let mut colors: Vec<(u32, u32, String)> =
                pixel_indices.into_iter().par_map(100, move |(i, j)| {
                    let thread_renderer = self.clone(); // Clone renderer for each thread
                    let thread_world = world.clone(); // Clone world for each thread

                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..thread_renderer.samples_per_pixel {
                        let ray = thread_renderer.get_ray(i, j);
                        pixel_color += ray.color(thread_renderer.max_depth.clone(), &thread_world);
                    }
                    //eprintln!("processing row {}", j);
                    (
                        i,
                        j,
                        (pixel_color * thread_renderer.pixel_sample_scale).to_string(),
                    )
                });

            // Sort by (j, i) to ensure correct row-wise order
            colors.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

            // Print the colors in order
            for (_, _, color) in colors {
                print!("{}", color);
            }
        } else {
            for j in 0..self.image_height {
                for i in 0..self.image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.get_ray(i, j);
                        pixel_color += ray.color(self.max_depth.clone(), &world);
                    }
                    print!("{}", (pixel_color * self.pixel_sample_scale).to_string());
                }
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

        self.center = self.look_from;

        let focal_length = (self.look_from - self.look_at).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h * focal_length;
        let viewport_width: f64 =
            viewport_height * self.image_width as f64 / self.image_height as f64;

        self.w = (self.look_from - self.look_at).normalize();
        self.u = self.vup.cross(&self.w).normalize();
        self.v = self.w.cross(&self.u);

        // Calculate horizontal and vertical vectors along viewport edges
        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * viewport_height * -1.0;

        // Calculate horizontal and vertical delta vectors for each pixel
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // Calculate location of upper left corner pixel
        let viewport_upper_left =
            &self.center - &(&self.w * focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;

        self.pixel00_loc =
            &viewport_upper_left + &((&self.pixel_delta_u + &self.pixel_delta_v) * 0.5);
    }
}
