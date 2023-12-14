use std::sync::Arc;

use rand::Rng;

use halide::camera::Camera;
use halide::color::Color;
use halide::hittable::HittableList;
use halide::material::{Dielectric, Lambertian, Material, Metal};
use halide::sphere::Sphere;
use halide::vector::{Point3D, Vector3D};

fn main() {
    let mut world = HittableList::default();
    let mut rng = rand::thread_rng();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen_range(0.0..1.0);
            let center = Point3D::new(a as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rng.gen_range(0.0..1.0));

            if (center - Point3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_material < 0.95 {
                    let albedo = Color::random_in_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };

                world.objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.objects.push(Arc::new(Sphere::new(Point3D::new(0.0, 1.0, 0.0), 1.0, material_1)));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(-4.0, 1.0, 0.0), 1.0, material_2)));
    world.objects.push(Arc::new(Sphere::new(Point3D::new(4.0, 1.0, 0.0), 1.0, material_3)));

    let mut camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        20.0,
        Point3D::new(13.0, 2.0, 3.0),
        Point3D::default(),
        Vector3D::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    camera.render_parallel(Arc::new(&world));
}
