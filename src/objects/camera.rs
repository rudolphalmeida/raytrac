use crate::ray::Ray;

use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Point3<f64>,
    pub lower_left_corner: Point3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3<f64>,
        lookat: Point3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        let lower_left_corner =
            lookfrom - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = 2.0 * focus_dist * u * half_width;
        let vertical = 2.0 * focus_dist * v * half_height;
        Camera {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, time: f64) -> Ray {
        let rd = super::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::from(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
