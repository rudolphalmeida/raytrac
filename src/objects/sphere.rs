use super::HitRecord;
use super::Hittable;
use crate::aabb::AABB;
use crate::materials::Material;
use crate::ray::Ray;

use cgmath::dot;
use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Point3;

use std::sync::Arc;

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    material: Arc<Material>,
}

impl Sphere {
    pub fn from(center: Point3<f64>, radius: f64, material: Arc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let (u, v) = super::get_sphere_uv((point - self.center.to_vec()) / self.radius);
                let normal = (point - self.center.to_vec()) / self.radius;
                let material = Arc::clone(&self.material);
                return Some(HitRecord::new(t, point, normal, material, u, v));
            }

            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let (u, v) = super::get_sphere_uv((point - self.center.to_vec()) / self.radius);
                let normal = (point - self.center.to_vec()) / self.radius;
                let material = Arc::clone(&self.material);
                return Some(HitRecord::new(t, point, normal, material, u, v));
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center.to_vec() - vec3(self.radius, self.radius, self.radius),
            self.center.to_vec() + vec3(self.radius, self.radius, self.radius),
        ))
    }
}
