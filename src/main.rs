use std::sync::Arc;

use halide::camera::Camera;
use halide::color::Color;
use halide::hittable::HittableList;
use halide::material::{Lambertian, Metal};
use halide::sphere::Sphere;
use halide::vector::Point3D;

fn main() {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));

    let left_material = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let right_material = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, ground_material)));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5, center_material)));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, left_material)));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, right_material)));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}
