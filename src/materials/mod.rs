use objects::HitRecord;
use ray::Ray;

pub mod dielectric;
pub mod lambertian;
pub mod light;
pub mod metal;

use self::dielectric::Dielectric;
use self::lambertian::Lambertian;
use self::light::DiffuseLight;
use self::metal::Metal;

use cgmath::dot;
use cgmath::vec3;
use cgmath::InnerSpace;
use cgmath::Vector3;
use rand::prelude::*;

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
    fn emitted(&self, _u: f64, _v: f64, _p: Vector3<f64>) -> Vector3<f64> {
        vec3::<f64>(0.0, 0.0, 0.0)
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray, rec),
            Material::Metal(ref inner) => inner.scatter(ray, rec),
            Material::Dielectric(ref inner) => inner.scatter(ray, rec),
            Material::DiffuseLight(ref inner) => inner.scatter(ray, rec),
        }
    }

    fn emitted(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        match *self {
            Material::Lambertian(ref inner) => inner.emitted(u, v, p),
            Material::Metal(ref inner) => inner.emitted(u, v, p),
            Material::Dielectric(ref inner) => inner.emitted(u, v, p),
            Material::DiffuseLight(ref inner) => inner.emitted(u, v, p),
        }
    }
}

fn point_in_unit_sphere() -> Vector3<f64> {
    Vector3::new(
        2.0 * random::<f64>() - 1.0,
        2.0 * random::<f64>() - 1.0,
        2.0 * random::<f64>() - 1.0,
    ).normalize()
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * dot(v, n) * n
}
