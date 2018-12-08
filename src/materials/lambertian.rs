use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::textures::constant_texture::ConstantTexture;
use crate::textures::Texture;
use crate::textures::Textured;

use super::{point_in_unit_sphere, Scatterable};

use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;

pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    pub fn new(albedo: Texture) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn color(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian {
            albedo: Texture::ConstantTexture(ConstantTexture::from(r, g, b)),
        }
    }

    pub fn from_vec3(color: Vector3<f64>) -> Self {
        Lambertian {
            albedo: Texture::ConstantTexture(ConstantTexture::new(color)),
        }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let target = rec.p + rec.normal + point_in_unit_sphere();
        let scattered = Ray::from(Point3::from_vec(rec.p), target - rec.p, ray.time);
        let attenuation = self.albedo.value(0.0, 0.0, rec.p);

        Some((scattered, attenuation))
    }
}
