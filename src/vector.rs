use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign};

use rand::Rng;

pub type Point3D = Vector3D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3D {
    values: [f64; 3],
}

impl Vector3D {
    pub fn default() -> Self {
        Self { values: [0.0, 0.0, 0.0] }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { values: [x, y, z] }
    }

    pub fn x(&self) -> f64 { self.values[0] }

    pub fn y(&self) -> f64 { self.values[1] }

    pub fn z(&self) -> f64 { self.values[2] }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.values[0].powi(2) + self.values[1].powi(2) + self.values[2].powi(2)
    }

    pub fn normalized(&self) -> Self { *self / self.length() }

    pub fn near_zero(&self) -> bool {
        let delta = 1e-8;
        self.values[0].abs() < delta && self.values[1].abs() < delta && self.values[2].abs() < delta
    }

    pub fn dot(lhs: &Self, rhs: &Self) -> f64 {
        lhs.values[0] * rhs.values[0] + lhs.values[1] * rhs.values[1] + lhs.values[2] * rhs.values[2]
    }

    pub fn cross(lhs: &Self, rhs: &Self) -> Self {
        Self::new(
            lhs.values[1] * rhs.values[2] - lhs.values[2] * rhs.values[1],
            lhs.values[2] * rhs.values[0] - lhs.values[0] * rhs.values[2],
            lhs.values[0] * rhs.values[1] - lhs.values[1] * rhs.values[0],
        )
    }

    pub fn reflect(vector: &Self, normal: &Self) -> Self {
        *vector - *normal * Self::dot(vector, normal) * 2.0
    }

    pub fn refract(uv: &Self, normal: &Self, eta_i_over_eta_t: f64) -> Self {
        let cos_theta = Self::dot(&-uv.clone(), normal).min(1.0);

        let ray_out_perpendicular = (normal.clone() * cos_theta + uv.clone()) * eta_i_over_eta_t;
        let ray_out_parallel = normal.clone() * -(1.0 - ray_out_perpendicular.length_squared()).abs().sqrt();

        ray_out_perpendicular + ray_out_parallel
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
    }

    pub fn random_in_range(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(range.clone()), rng.gen_range(range.clone()), rng.gen_range(range.clone()))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Self::random_in_range(-1.0..1.0);
            if point.length_squared() < 1.0 { return point; }
        }
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_unit_sphere = Self::random_normal();
        return if Self::dot(&on_unit_sphere, normal) > 0.0 { on_unit_sphere } else { -on_unit_sphere };
    }

    pub fn random_normal() -> Self {
        Self::random_in_unit_sphere().normalized()
    }
}

impl Index<usize> for Vector3D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 2 {
            panic!("Cannot access index {index} for Vector3D.");
        }

        &self.values[index]
    }
}

impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 2 {
            panic!("Cannot access index {index} for Vector3D.");
        }

        &mut self.values[index]
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        Vector3D::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.values[0] + rhs.values[0],
            self.values[1] + rhs.values[1],
            self.values[2] + rhs.values[2],
        )
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        self.values[0] += rhs.values[0];
        self.values[1] += rhs.values[1];
        self.values[2] += rhs.values[2];
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.values[0] - rhs.values[0],
            self.values[1] - rhs.values[1],
            self.values[2] - rhs.values[2],
        )
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.values[0] -= rhs.x();
        self.values[1] -= rhs.y();
        self.values[2] -= rhs.z();
    }
}

impl Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.values[0] * scalar, self.values[1] * scalar, self.values[2] * scalar)
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, scalar: f64) {
        self.values[0] *= scalar;
        self.values[1] *= scalar;
        self.values[2] *= scalar;
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.values[0] / scalar, self.values[1] / scalar, self.values[2] / scalar)
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, scalar: f64) {
        self.values[0] /= scalar;
        self.values[1] /= scalar;
        self.values[2] /= scalar;
    }
}


