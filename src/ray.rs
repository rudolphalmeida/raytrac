use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn from(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> Vector3<f64> {
        self.origin.to_vec() + t * self.direction
    }
}
