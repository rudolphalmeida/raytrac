use crate::objects::HitRecord;
use crate::ray::Ray;

use super::Scatterable;

use cgmath::dot;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Point3;
use cgmath::Vector3;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn from(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let reflected = super::reflect(ray.direction.normalize(), rec.normal);
        let attenuation = vec3::<f64>(1.0, 1.0, 1.0);
        let mut refracted = Vector3::new(0.0, 0.0, 0.0);
        let reflect_prob: f64;

        let (outward_normal, ni_over_nt, cosine) = if dot(ray.direction, rec.normal) > 0.0 {
            (
                -rec.normal,
                self.refractive_index,
                self.refractive_index * dot(ray.direction, rec.normal) / ray.direction.magnitude(),
            )
        } else {
            (
                rec.normal,
                1.0 / self.refractive_index,
                -1.0 * ray.direction.dot(rec.normal) / ray.direction.magnitude(),
            )
        };

        if let Some(r) = refract(ray.direction, outward_normal, ni_over_nt) {
            refracted = r;
            reflect_prob = schlick(cosine, self.refractive_index);
        } else {
            reflect_prob = 1.0;
        }

        let scattered;
        let mut rng = thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            scattered = Ray::from(Point3::from_vec(rec.p), reflected, ray.time);
        } else {
            scattered = Ray::from(Point3::from_vec(rec.p), refracted, ray.time);
        }

        Some((scattered, attenuation))
    }
}

pub fn refract(v: Vector3<f64>, n: Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
