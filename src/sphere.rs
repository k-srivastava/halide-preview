use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D};

#[derive(Clone)]
pub struct Sphere {
    center: Point3D,
    center_vector: Vector3D,
    radius: f64,
    is_moving: bool,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new_static(center: Point3D, radius: f64, material: Arc<dyn Material>) -> Self {
        Self { center, center_vector: Vector3D::default(), radius, is_moving: false, material }
    }

    pub fn new_dynamic(center1: Point3D, center2: Point3D, radius: f64, material: Arc<dyn Material>) -> Self {
        Self { center: center1, center_vector: center2 - center1, radius, is_moving: true, material }
    }

    pub fn center(&self, time: f64) -> Point3D { self.center + self.center_vector * time }
}

unsafe impl Send for Sphere {}

unsafe impl Sync for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving { self.center(ray.time()) } else { self.center };
        let origin_center = ray.origin() - center;

        let a = ray.direction().length_squared();
        let half_b = Vector3D::dot(&origin_center, &ray.direction());
        let c = origin_center.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 { return None; }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;

        if !interval.surrounds(root) {
            root = (-half_b + discriminant_sqrt) / a;
            if !interval.surrounds(root) { return None; }
        }

        let mut record = HitRecord::default();
        record.depth = root;
        record.point = ray.at(record.depth);

        let outward_normal = (record.point - center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.material = Some(self.material.clone());

        Some(record)
    }
}
