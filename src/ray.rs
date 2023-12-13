use crate::vector::{Point3D, Vector3D};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Ray {
    pub fn default() -> Self {
        Self { origin: Point3D::default(), direction: Vector3D::default() }
    }

    pub fn new(origin: Point3D, direction: Vector3D) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, depth: f64) -> Point3D {
        self.origin + self.direction * depth
    }

    pub fn origin(&self) -> Point3D { self.origin }

    pub fn direction(&self) -> Vector3D { self.direction }
}