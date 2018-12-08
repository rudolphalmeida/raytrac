use crate::materials::Scatterable;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::textures::constant_texture::ConstantTexture;
use crate::textures::Texture;
use crate::textures::Textured;

use cgmath::Vector3;

pub struct DiffuseLight {
    emit: Texture,
}

impl DiffuseLight {
    pub fn new(emit: Texture) -> Self {
        DiffuseLight { emit }
    }

    pub fn color(r: f64, g: f64, b: f64) -> Self {
        DiffuseLight {
            emit: Texture::ConstantTexture(ConstantTexture::from(r, g, b)),
        }
    }

    pub fn from_vec3(color: Vector3<f64>) -> Self {
        DiffuseLight {
            emit: Texture::ConstantTexture(ConstantTexture::new(color)),
        }
    }
}

impl Scatterable for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        self.emit.value(u, v, p)
    }
}
