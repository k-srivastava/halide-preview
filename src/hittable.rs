use std::sync::Arc;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vector3D,
    pub depth: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        Self {
            point: Point3D::default(),
            normal: Vector3D::default(),
            depth: f64::default(),
            front_face: bool::default(),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3D) {
        self.front_face = Vector3D::dot(&ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal.clone() } else { -outward_normal.clone() };
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn default() -> Self {
        Self { objects: vec![] }
    }

    pub fn new(object: Arc<dyn Hittable>) -> Self {
        Self { objects: vec![object] }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: &Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if object.hit(ray, &Interval::new(interval.min, closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.depth;

                record.point = temp_record.point;
                record.normal = temp_record.normal;
                record.depth = temp_record.depth;
                record.front_face = temp_record.front_face;
            }
        }

        hit_anything
    }
}
