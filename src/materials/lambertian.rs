use objects::HitRecord;
use ray::Ray;

use super::{point_in_unit_sphere, Scatterable};

use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Point3;
use cgmath::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn from(x: f64, y: f64, z: f64) -> Lambertian {
        Lambertian {
            albedo: vec3::<f64>(x, y, z),
        }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let target = rec.p + rec.normal + point_in_unit_sphere();
        let scattered = Ray::from(Point3::from_vec(rec.p), target - rec.p);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
