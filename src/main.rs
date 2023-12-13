use std::sync::Arc;

use halide::camera::Camera;
use halide::hittable::HittableList;
use halide::sphere::Sphere;
use halide::vector::Point3D;

fn main() {
    let mut world = HittableList::default();

    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5)));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::new(16.0 / 9.0, 400, 100);
    camera.render(&world);
}
