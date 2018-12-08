use crate::objects::HitRecord;
use crate::ray::Ray;

use super::{point_in_unit_sphere, Scatterable};

use cgmath::dot;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Point3;
use cgmath::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn from(x: f64, y: f64, z: f64, fuzz: f64) -> Self {
        Metal {
            albedo: vec3::<f64>(x, y, z),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = super::reflect(ray.direction.normalize(), rec.normal);
        let scattered = Ray::from(
            Point3::from_vec(rec.p),
            reflected + point_in_unit_sphere() * self.fuzz,
            ray.time,
        );
        let attenuation = self.albedo;

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
