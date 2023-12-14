use rand::Rng;

use crate::color::Color;
use crate::ray::Ray;
use crate::vector::Vector3D;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record_point: &Vector3D, record_normal: &Vector3D, record_front_face: bool, attenuation: &mut Color) -> Option<Ray>;
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
    fn scatter(&self, _ray_in: &Ray, record_point: &Vector3D, record_normal: &Vector3D, _record_front_face: bool, attenuation: &mut Color) -> Option<Ray> {
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
    fn scatter(&self, ray_in: &Ray, record_point: &Vector3D, record_normal: &Vector3D, _record_front_face: bool, attenuation: &mut Color) -> Option<Ray> {
        let reflected = Vector3D::reflect(&ray_in.direction().normalized(), record_normal);
        let scattered = Ray::new(record_point.clone(), reflected + Vector3D::random_normal() * self.fuzz);

        attenuation[0] = self.albedo.x();
        attenuation[1] = self.albedo.y();
        attenuation[2] = self.albedo.z();

        if Vector3D::dot(&scattered.direction(), record_normal) > 0.0 { Some(scattered) } else { None }
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    pub fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record_point: &Vector3D, record_normal: &Vector3D, record_front_face: bool, attenuation: &mut Color) -> Option<Ray> {
        attenuation[0] = 1.0;
        attenuation[1] = 1.0;
        attenuation[2] = 1.0;

        let refraction_ratio = if record_front_face { 1.0 / self.refractive_index } else { self.refractive_index };
        let direction_normal = ray_in.direction().normalized();

        let cos_theta = Vector3D::dot(&-direction_normal, record_normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0) {
            Vector3D::reflect(&direction_normal, record_normal)
        } else {
            Vector3D::refract(&direction_normal, record_normal, refraction_ratio)
        };

        Some(Ray::new(record_point.clone(), direction))
    }
}
