use super::HitRecord;
use super::Hittable;
use super::TimedMovement;
use aabb::AABB;
use materials::Material;
use objects::sphere::Sphere;
use ray::Ray;

use cgmath::dot;
use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;

pub struct MovingSphere {
    pub movement: TimedMovement,
    pub radius: f64,
    pub material: Material,
}

impl MovingSphere {
    pub fn from(
        center0: Point3<f64>,
        center1: Point3<f64>,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Material,
    ) -> MovingSphere {
        MovingSphere {
            movement: TimedMovement::new(time0, center0, time1, center1),
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vector3<f64> {
        self.movement.lerp(time)
    }
}

impl Hittable for MovingSphere {
    fn hits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = (ray.origin - self.center(ray.time)).to_vec();
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let normal = (point - self.center(ray.time)) / self.radius;
                let material = &self.material;
                return Some(HitRecord::new(t, point, normal, material));
            }

            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let point = ray.point_at(t);
                let normal = (point - self.center(ray.time)) / self.radius;
                let material = &self.material;
                return Some(HitRecord::new(t, point, normal, material));
            }
        }

        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let start_pos = Sphere::from(self.movement.start, self.radius, Material::PitchBlack);
        let end_pos = Sphere::from(self.movement.end, self.radius, Material::PitchBlack);

        let box_start = start_pos.bounding_box(t0, t1).unwrap();
        let box_end = end_pos.bounding_box(t0, t1).unwrap();

        let bx = box_start.surrounding_box(&box_end);

        Some(bx)
    }
}
