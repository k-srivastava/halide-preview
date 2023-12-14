use std::io;
use std::io::Write;
use std::sync::Arc;

use rand::Rng;
use rayon::prelude::*;

use crate::color;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pub vertical_fov: f64,
    pub look_from: Point3D,
    pub look_at: Point3D,
    pub vertical_up: Vector3D,

    pub defocus_angle: f64,
    pub focus_distance: f64,

    image_height: usize,
    center: Point3D,
    pixel_location_100: Point3D,
    pixel_delta_u: Vector3D,
    pixel_delta_v: Vector3D,

    u: Vector3D,
    v: Vector3D,
    w: Vector3D,

    defocus_disk_u: Vector3D,
    defocus_disk_v: Vector3D,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            vertical_fov: 90.0,
            look_from: Point3D::new(0.0, 0.0, -1.0),
            look_at: Point3D::default(),
            vertical_up: Vector3D::new(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_distance: 10.0,

            image_height: usize::default(),
            center: Point3D::default(),
            pixel_location_100: Point3D::default(),
            pixel_delta_u: Vector3D::default(),
            pixel_delta_v: Vector3D::default(),

            u: Vector3D::default(),
            v: Vector3D::default(),
            w: Vector3D::default(),

            defocus_disk_u: Vector3D::default(),
            defocus_disk_v: Vector3D::default(),
        }
    }

    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vertical_fov: f64,
        look_from: Point3D,
        look_at: Point3D,
        vertical_up: Vector3D,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,

            vertical_fov,
            look_from,
            look_at,
            vertical_up,

            defocus_angle,
            focus_distance,

            image_height: usize::default(),
            center: Point3D::default(),
            pixel_location_100: Point3D::default(),
            pixel_delta_u: Vector3D::default(),
            pixel_delta_v: Vector3D::default(),

            u: Vector3D::default(),
            v: Vector3D::default(),
            w: Vector3D::default(),

            defocus_disk_u: Vector3D::default(),
            defocus_disk_v: Vector3D::default(),
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.center = self.look_from;

        let theta = self.vertical_fov.to_radians();
        let height = (theta / 2.0).tan();

        let viewport_height = height * self.focus_distance * 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.look_from - self.look_at).normalized();
        self.u = Vector3D::cross(&self.vertical_up, &self.w).normalized();
        self.v = Vector3D::cross(&self.w, &self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - self.w * self.focus_distance - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_location_100 = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn pixel_sample_square(&self) -> Vector3D {
        let mut rng = rand::thread_rng();

        let px = -0.5 + rng.gen_range(0.0..1.0);
        let py = -0.5 + rng.gen_range(0.0..1.0);

        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center = self.pixel_location_100 + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 { self.center.clone() } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rLines Remaining: {}", self.image_height - j);
            io::stderr().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_color = Color::default();

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&ray, self.max_depth, world);
                }

                color::write_color(&pixel_color, self.samples_per_pixel);
            }
        }

        eprintln!("\n\rDone.")
    }

    // pub fn render_parallel(&mut self, world: Arc<&dyn Hittable>) {
    //     self.initialize();
    //
    //     println!("P3\n{} {}\n255\n", self.image_width, self.image_height);
    //
    //     for j in 0..self.image_height {
    //         eprint!("\rLines Remaining: {}", self.image_height - j);
    //         io::stderr().flush().unwrap();
    //
    //         let pixel_colors: Vec<Color> = (0..self.image_width).into_par_iter().map(|i| {
    //             let mut pixel_color = Color::default();
    //
    //             (0..self.samples_per_pixel).for_each(|_| {
    //                 let ray = self.get_ray(i, j);
    //                 pixel_color += Camera::ray_color_parallel(&ray, self.max_depth, world.clone())
    //             });
    //
    //             pixel_color
    //         }).collect();
    //
    //         for pixel_color in pixel_colors {
    //             color::write_color(&pixel_color, self.samples_per_pixel);
    //         }
    //     }
    //
    //     eprintln!("\n\rDone.")
    //
    // }

    pub fn render_parallel(&mut self, world: Arc<&dyn Hittable>) {
        self.initialize();

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let lines: Vec<Vec<Color>> = (0..self.image_height).into_par_iter().map(|j| {
            eprint!("\rCurrent Line: {j}");
            io::stderr().flush().unwrap();

            let pixel_colors: Vec<Color> = (0..self.image_width).into_par_iter().map(|i| {
                let mut pixel_color = Color::default();

                (0..self.samples_per_pixel).for_each(|_| {
                    let ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color_parallel(&ray, self.max_depth, world.clone());
                });

                pixel_color
            }).collect();

            pixel_colors
        }).collect();

        for line in lines {
            for pixel_color in line {
                color::write_color(&pixel_color, self.samples_per_pixel);
            }
        }

        eprintln!("\n\rDone.")
    }

    fn defocus_disk_sample(&self) -> Point3D {
        let point = Vector3D::random_in_unit_disk();
        self.center + self.defocus_disk_u * point.x() + self.defocus_disk_v * point.y()
    }

    fn ray_color(ray: &Ray, max_depth: usize, world: &dyn Hittable) -> Color {
        if max_depth <= 0 { return Color::default(); }

        if let Some(record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            return match record.material {
                None => Color::default(),

                Some(material) => {
                    let mut attenuation = Color::default();

                    return if let Some(scattered) = material.scatter(
                        ray,
                        &record.point,
                        &record.normal,
                        record.front_face,
                        &mut attenuation,
                    ) {
                        attenuation * Camera::ray_color(&scattered, max_depth - 1, world)
                    } else {
                        Color::default()
                    };
                }
            };
        }

        let direction_normal = ray.direction().normalized();
        let a = (direction_normal.y() + 1.0) * 0.5;

        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn ray_color_parallel(ray: &Ray, max_depth: usize, world: Arc<&dyn Hittable>) -> Color {
        if max_depth <= 0 { return Color::default(); }

        if let Some(record) = world.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
            return match record.material {
                None => Color::default(),

                Some(material) => {
                    let mut attenuation = Color::default();

                    return if let Some(scattered) = material.scatter(
                        ray,
                        &record.point,
                        &record.normal,
                        record.front_face,
                        &mut attenuation,
                    ) {
                        attenuation * Camera::ray_color_parallel(&scattered, max_depth - 1, world)
                    } else {
                        Color::default()
                    };
                }
            };
        }

        let direction_normal = ray.direction().normalized();
        let a = (direction_normal.y() + 1.0) * 0.5;

        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
