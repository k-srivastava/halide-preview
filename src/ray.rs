use crate::vector::{Point3D, Vector3D};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
    time: f64,
}

impl Ray {
    pub fn default() -> Self {
        Self { origin: Point3D::default(), direction: Vector3D::default(), time: 0.0 }
    }

    pub fn new(origin: Point3D, direction: Vector3D, time: f64) -> Self {
        Self { origin, direction, time }
    }

    pub fn at(&self, depth: f64) -> Point3D { self.origin + self.direction * depth }

    pub fn origin(&self) -> Point3D { self.origin }

    pub fn direction(&self) -> Vector3D { self.direction }

    pub fn time(&self) -> f64 { self.time }
}