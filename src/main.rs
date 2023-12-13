use std::io;
use std::io::Write;

use halide::color;
use halide::color::Color;
use halide::ray::Ray;
use halide::vector::{Point3D, Vector3D};

fn ray_color(ray: &Ray) -> Color {
    let direction_normal = ray.direction().normalized();
    let a = (direction_normal.y() + 1.0) * 0.5;

    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    let camera_center = Point3D::default();

    let viewport_u = Vector3D::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3D::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vector3D::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_location_100 = viewport_upper_left + (pixel_delta_u - pixel_delta_v) * 0.5;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rLines Remaining: {}", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel_center = pixel_location_100 + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray);

            color::write_color(&pixel_color);
        }
    }

    eprintln!("\n\rDone.")
}
