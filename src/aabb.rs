use crate::ray::Ray;

use cgmath::vec3;
use cgmath::Vector3;

use std::mem::swap;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl AABB {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> Self {
        AABB { min, max }
    }

    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let tmin = t0.max(tmin);
            let tmax = t1.min(tmax);

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(&self, other: &AABB) -> Self {
        let small = vec3(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );
        let big = vec3(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );

        AABB::new(small, big)
    }
}
