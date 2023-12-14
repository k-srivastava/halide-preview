use std::sync::Arc;

use halide::camera::Camera;
use halide::color::Color;
use halide::hittable::HittableList;
use halide::material::{Dielectric, Lambertian, Metal};
use halide::sphere::Sphere;
use halide::vector::Point3D;

fn main() {
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));

    let left_material = Arc::new(Dielectric::new(1.5));
    let right_material = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0, ground_material.clone())));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5, center_material.clone())));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), 0.5, left_material.clone())));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(-1.0, 0.0, -1.0), -0.4, left_material.clone())));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(1.0, 0.0, -1.0), 0.5, right_material.clone())));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world);
}
