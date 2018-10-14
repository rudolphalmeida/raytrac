use super::Texture;
use super::Textured;

use cgmath::Vector3;

pub struct CheckedTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl CheckedTexture {
    pub fn new(odd: Texture, even: Texture) -> Self {
        CheckedTexture {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }
}

impl Textured for CheckedTexture {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
