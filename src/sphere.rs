use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Point3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_depth: f64, max_depth: f64, record: &mut HitRecord) -> bool {
        let origin_center = ray.origin() - self.center;

        let a = ray.direction().length_squared();
        let half_b = Vector3D::dot(&origin_center, &ray.direction());
        let c = origin_center.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 { return false; }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;

        if root <= min_depth || root >= max_depth {
            root = (-half_b + discriminant_sqrt) / a;
            if root <= min_depth || root >= max_depth { return false; }
        }

        record.depth = root;
        record.point = ray.at(record.depth);

        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        true
    }
}
