pub mod camera;
pub mod moving_sphere;
pub mod sphere;

use crate::aabb::AABB;
use crate::materials::Material;
use crate::ray::Ray;

use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Point3;
use cgmath::Vector3;
use rand::prelude::*;

use std::f64::consts::PI;
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Arc<Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        t: f64,
        p: Vector3<f64>,
        normal: Vector3<f64>,
        material: Arc<Material>,
        u: f64,
        v: f64,
    ) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
            u,
            v,
        }
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for sphere in &self.objects {
            if let Some(hit) = sphere.hits(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.size() < 1 {
            return None;
        }

        let mut bx = self.objects[0].bounding_box(t0, t1)?;

        for i in 1..self.size() {
            let tmp_box = self.objects[i].bounding_box(t0, t1)?;
            bx = bx.surrounding_box(&tmp_box);
        }

        Some(bx)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimedMovement {
    pub starttime: f64,
    pub start: Point3<f64>,
    pub endtime: f64,
    pub end: Point3<f64>,
}

impl TimedMovement {
    pub fn new(
        starttime: f64,
        start: Point3<f64>,
        endtime: f64,
        end: Point3<f64>,
    ) -> TimedMovement {
        TimedMovement {
            starttime,
            start,
            endtime,
            end,
        }
    }

    pub fn lerp(&self, time: f64) -> Vector3<f64> {
        self.start.to_vec()
            + ((time - self.starttime) / (self.endtime - time)) * (self.end - self.start)
    }
}

fn random_in_unit_disk() -> Vector3<f64> {
    let mut rng = thread_rng();
    let mut p =
        2.0 * vec3::<f64>(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - vec3::<f64>(1.0, 1.0, 0.0);
    while p.dot(p) >= 1.0 {
        p = 2.0 * vec3::<f64>(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - vec3::<f64>(1.0, 1.0, 0.0);
    }

    p
}

fn get_sphere_uv(p: Vector3<f64>) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;

    (u, v)
}
