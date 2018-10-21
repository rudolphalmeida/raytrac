use cgmath::Vector3;

pub trait Textured {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64>;
}

pub enum Texture {
    ConstantTexture(constant_texture::ConstantTexture),
    CheckedTexture(checked_texture::CheckedTexture),
    NoiseTexture(noise_texture::NoiseTexture),
}

impl Textured for Texture {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        match *self {
            Texture::ConstantTexture(ref tex) => tex.value(u, v, p),
            Texture::CheckedTexture(ref tex) => tex.value(u, v, p),
            Texture::NoiseTexture(ref tex) => tex.value(u, v, p),
        }
    }
}

pub mod checked_texture;
pub mod constant_texture;
pub mod noise_texture;
