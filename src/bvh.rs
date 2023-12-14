use std::cmp::Ordering;
use std::sync::Arc;

use rand::Rng;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut mut_objects = objects.clone();

        let axis = rand::thread_rng().gen_range(0usize..=2usize);
        let comparator = match axis {
            0 => BVHNode::box_x_compare,
            1 => BVHNode::box_y_compare,
            _ => BVHNode::box_z_compare
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (mut_objects[start].clone(), mut_objects[start].clone())
        } else if object_span == 2 {
            if comparator(mut_objects[start].clone(), mut_objects[start + 1].clone()) == Ordering::Less {
                (mut_objects[start].clone(), mut_objects[start + 1].clone())
            } else {
                (mut_objects[start + 1].clone(), mut_objects[start].clone())
            }
        } else {
            mut_objects[start..end].sort_by(|a, b| comparator(a.clone(), b.clone()));

            let middle = start + object_span / 2;
            (
                Arc::new(BVHNode::new(&mut_objects, start, middle)) as Arc<dyn Hittable>,
                Arc::new(BVHNode::new(&mut_objects, middle, end)) as Arc<dyn Hittable>
            )
        };

        Self {
            left: left.clone(),
            right: right.clone(),
            bounding_box: AABB::from_aabb_bounds(&left.clone().bounding_box(), &right.clone().bounding_box()),
        }
    }

    pub fn from_hittable_list(list: &HittableList) -> Self {
        Self::new(&list.objects, 0, list.objects.len())
    }

    fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        if a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
        BVHNode::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, interval: &mut Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, interval) { return None; }

        let hit_left = self.left.hit(ray, interval);
        let hit_right = self.right.hit(
            ray,
            &mut Interval::new(
                interval.min,
                if hit_left.clone().is_some() { hit_left.clone().unwrap().depth } else { interval.max }
            )
        );

        hit_left.or(hit_right)
    }

    fn bounding_box(&self) -> AABB { self.bounding_box }
}
