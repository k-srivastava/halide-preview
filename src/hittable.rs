use std::sync::Arc;
use crate::aabb::AABB;

use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point3D, Vector3D};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &mut Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vector3D,
    pub material: Option<Arc<dyn Material>>,
    pub depth: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        Self {
            point: Point3D::default(),
            normal: Vector3D::default(),
            material: None,
            depth: f64::default(),
            front_face: bool::default(),
        }
    }

    pub fn new(
        point: Point3D,
        normal: Vector3D,
        material: Option<Arc<dyn Material>>,
        depth: f64,
        front_face: bool,
    ) -> Self {
        Self { point, normal, material, depth, front_face }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3D) {
        self.front_face = Vector3D::dot(&ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal.clone() } else { -outward_normal.clone() };
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bounding_box: AABB,
}

impl HittableList {
    pub fn default() -> Self { Self { objects: vec![], bounding_box: AABB::default() } }

    pub fn new(object: Arc<dyn Hittable>) -> Self { Self { objects: vec![object.clone()], bounding_box: object.clone().bounding_box() } }

    pub fn add_object(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bounding_box = AABB::from_aabb_bounds(&self.bounding_box, &object.clone().bounding_box());
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: &mut Interval) -> Option<HitRecord> {
        let mut return_record: Option<HitRecord> = None;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if let Some(record) = object.hit(ray, &mut Interval::new(interval.min, closest_so_far)) {
                closest_so_far = record.depth;

                return_record = Some(record.clone());
            }
        }

        return_record
    }

    fn bounding_box(&self) -> AABB { self.bounding_box }
}
