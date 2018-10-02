use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn from(origin: Point3<f64>, direction: Vector3<f64>, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn point_at(&self, t: f64) -> Vector3<f64> {
        self.origin.to_vec() + t * self.direction
    }
}
