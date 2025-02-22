use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils::sample_square;
use crate::vector::{Point3, Vec3};
use std::sync::Arc;
use std::thread;
#[derive(Debug, Default, Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32, // Number of ray bounces allowed
    pub threads: u32,

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
        c.max_depth = 10;
        c
    }
    pub fn render<T>(&mut self, world: T)
    where
        T: Hittable + Sync + 'static,
    {
        self.initialize();

        // Print header
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Create thread-safe reference to world
        let world = Arc::new(world);

        // Determine number of threads and chunk size
        let num_threads = self.threads;
        let chunk_size = (self.image_height + num_threads - 1) / num_threads;

        // Spawn threads
        let mut handles = vec![];

        for thread_id in 0..num_threads {
            // Calculate this thread's row range
            let start_row = thread_id * chunk_size;
            let end_row = (start_row + chunk_size).min(self.image_height);

            // Clone Arc for this thread
            let world = Arc::clone(&world);

            // Clone renderer for this thread
            let renderer = self.clone();

            // Spawn thread
            let handle = thread::spawn(move || {
                let mut output = String::new();

                for j in start_row..end_row {
                    eprintln!("Thread {} processing row {}", thread_id, j);

                    for i in 0..renderer.image_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                        for _ in 0..renderer.samples_per_pixel {
                            let ray = renderer.get_ray(i, j);
                            pixel_color += ray.color(renderer.max_depth.clone(), &*world);
                        }

                        output.push_str(&(pixel_color * renderer.pixel_sample_scale).to_string());
                    }
                    output.push('\n');
                }

                (start_row, output)
            });

            handles.push(handle);
        }

        // Collect and sort results
        let mut results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        results.sort_by_key(|(row, _)| *row);

        // Print results in order
        for (_, output) in results {
            print!("{}", output);
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
