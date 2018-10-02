use super::HitRecord;
use super::Hittable;
use materials::Material;
use ray::Ray;

use cgmath::dot;
use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Moving_Sphere {
    pub center0: Point3<f64>,
    pub center1: Point3<f64>,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Material,
}

impl Moving_Sphere {
    pub fn from(
        center0: Point3<f64>,
        center1: Point3<f64>,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Material,
    ) -> Moving_Sphere {
        Moving_Sphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vector3<f64> {
        self.center0.to_vec()
            + ((time - self.time0) / (self.time1 - time)) * (self.center1 - self.center0)
    }
}

impl Hittable for Moving_Sphere {
    fn hits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = (ray.origin - self.center(ray.time)).to_vec();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let p = ray.point_at(t);
                let normal = (p - self.center(ray.time)) / self.radius;
                let material = self.material;
                return Some(HitRecord::new(t, p, normal, material));
            }

            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let p = ray.point_at(t);
                let normal = (p - self.center(ray.time)) / self.radius;
                let material = self.material;
                return Some(HitRecord::new(t, p, normal, material));
            }
        }

        None
    }
}
