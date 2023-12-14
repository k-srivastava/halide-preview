use crate::color::Color;
use crate::ray::Ray;
use crate::vector::Vector3D;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record_point: &Vector3D, record_normal: &Vector3D, attenuation: &mut Color) -> Option<Ray>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record_point: &Vector3D, record_normal: &Vector3D, attenuation: &mut Color) -> Option<Ray> {
        let mut scatter_direction = *record_normal + Vector3D::random_normal();
        if scatter_direction.near_zero() { scatter_direction = record_normal.clone(); }

        let scattered = Ray::new(record_point.clone(), scatter_direction);

        attenuation[0] = self.albedo.x();
        attenuation[1] = self.albedo.y();
        attenuation[2] = self.albedo.z();

        Some(scattered)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self { Self { albedo, fuzz } }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record_point: &Vector3D, record_normal: &Vector3D, attenuation: &mut Color) -> Option<Ray> {
        let reflected = Vector3D::reflect(&ray_in.direction().normalized(), record_normal);
        let scattered = Ray::new(record_point.clone(), reflected + Vector3D::random_normal() * self.fuzz);

        attenuation[0] = self.albedo.x();
        attenuation[1] = self.albedo.y();
        attenuation[2] = self.albedo.z();

        if Vector3D::dot(&scattered.direction(), record_normal) > 0.0 { Some(scattered) } else { None }
    }
}
