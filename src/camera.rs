use std::io;
use std::io::Write;

use crate::color;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    center: Point3D,
    pixel_location_100: Point3D,
    pixel_delta_u: Vector3D,
    pixel_delta_v: Vector3D,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: usize::default(),
            center: Point3D::default(),
            pixel_location_100: Point3D::default(),
            pixel_delta_u: Vector3D::default(),
            pixel_delta_v: Vector3D::default(),
        }
    }

    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: usize::default(),
            center: Point3D::default(),
            pixel_location_100: Point3D::default(),
            pixel_delta_u: Vector3D::default(),
            pixel_delta_v: Vector3D::default(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.center = Point3D::default();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vector3D::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3D::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vector3D::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_location_100 = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
        let mut record = HitRecord::default();

        if world.hit(ray, &Interval::new(0.0, f64::INFINITY), &mut record) {
            return (record.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let direction_normal = ray.direction().normalized();
        let a = (direction_normal.y() + 1.0) * 0.5;

        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rLines Remaining: {}", self.image_height - j);
            io::stderr().flush().unwrap();

            for i in 0..self.image_width {
                let pixel_center = self.pixel_location_100 + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;

                let ray = Ray::new(self.center, ray_direction);
                let pixel_color = Camera::ray_color(&ray, world);

                color::write_color(&pixel_color);
            }
        }

        eprintln!("\n\rDone.")
    }
}
