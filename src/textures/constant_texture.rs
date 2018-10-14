use cgmath::vec3;
use cgmath::Vector3;

use super::Textured;

pub struct ConstantTexture {
    pub color: Vector3<f64>,
}

impl ConstantTexture {
    pub fn new(color: Vector3<f64>) -> Self {
        ConstantTexture { color }
    }

    pub fn from(r: f64, g: f64, b: f64) -> Self {
        ConstantTexture {
            color: vec3::<f64>(r, g, b),
        }
    }
}

impl Textured for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: Vector3<f64>) -> Vector3<f64> {
        self.color
    }
}
