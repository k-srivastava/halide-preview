use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Point3D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn default() -> Self {
        Self { x: Interval::empty(), y: Interval::empty(), z: Interval::empty() }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_vector_bounds(extrema1: &Point3D, extrema2: &Point3D) -> Self {
        Self {
            x: Interval::new(extrema1.x().min(extrema2.x()), extrema1.x().max(extrema2.x())),
            y: Interval::new(extrema1.y().min(extrema2.y()), extrema1.y().max(extrema2.y())),
            z: Interval::new(extrema1.z().min(extrema2.z()), extrema1.z().max(extrema2.z())),
        }
    }

    pub fn from_aabb_bounds(box1: &Self, box2: &Self) -> Self {
        Self {
            x: Interval::from_interval_bounds(&box1.x, &box2.x),
            y: Interval::from_interval_bounds(&box1.y, &box2.y),
            z: Interval::from_interval_bounds(&box1.z, &box2.z),
        }
    }

    pub fn axis(&self, index: usize) -> Interval {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Cannot access index {index} for an AABB.")
        }
    }

    pub fn hit(&self, ray: &Ray, interval: &mut Interval) -> bool {
        for a in 0..3 {
            let inverse_direction = 1.0 / ray.direction()[a];
            let origin = ray.origin()[a];

            let (min_depth, max_depth) = if inverse_direction >= 0.0 {
                (
                    (self.axis(a).min - origin) * inverse_direction,
                    (self.axis(a).max - origin) * inverse_direction
                )
            } else {
                (
                    (self.axis(a).max - origin) * inverse_direction,
                    (self.axis(a).min - origin) * inverse_direction
                )
            };

            if min_depth > interval.min { interval.min = min_depth; }
            if max_depth < interval.max { interval.max = max_depth; }

            if interval.max <= interval.min { return false; }
        }

        true
    }
}
