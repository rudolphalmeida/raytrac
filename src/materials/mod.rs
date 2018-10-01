use objects::HitRecord;
use ray::Ray;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use self::dielectric::Dielectric;
use self::lambertian::Lambertian;
use self::metal::Metal;

use cgmath::dot;
use cgmath::vec3;
use cgmath::Vector3;
use rand::prelude::*;

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray, rec),
            Material::Metal(ref inner) => inner.scatter(ray, rec),
            Material::Dielectric(ref inner) => inner.scatter(ray, rec),
        }
    }
}

fn point_in_unit_sphere() -> Vector3<f64> {
    let mut rng = thread_rng();
    let mut p = vec3::<f64>(2.0, 2.0, 2.0);
    while dot(p, p) >= 1.0 {
        p = 2.0 * vec3::<f64>(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - vec3::<f64>(1.0, 1.0, 1.0);
    }

    p
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * dot(v, n) * n
}
