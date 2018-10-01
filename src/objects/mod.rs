pub mod camera;
pub mod sphere;

use self::sphere::Sphere;
use materials::Material;
use ray::Ray;

use cgmath::prelude::*;
use cgmath::vec3;
use cgmath::Vector3;
use rand::prelude::*;

pub trait Hittable: Send + Sync {
    fn hits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
}

impl HitRecord {
    pub fn new(t: f64, p: Vector3<f64>, normal: Vector3<f64>, material: Material) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub struct HittableList {
    spheres: Vec<Box<Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            spheres: Vec::new(),
        }
    }

    pub fn add(&mut self, sphere: Sphere) {
        self.spheres.push(Box::new(sphere));
    }

    pub fn size(&self) -> usize {
        self.spheres.len()
    }
}

impl Hittable for HittableList {
    fn hits(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for sphere in &self.spheres {
            if let Some(hit) = sphere.hits(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
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
