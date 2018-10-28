use bvh::BvhTree;
use io::write::write_img;
use materials::Scatterable;
use objects::camera::Camera;
use objects::Hittable;
use objects::HittableList;
use ray::Ray;

use cgmath::vec3;
use cgmath::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use std::f64;

pub struct Scene<'a> {
    pub camera: Camera,
    pub width: u16,
    pub height: u16,
    pub samples: u64,
    pub world: BvhTree<'a>,
    pub time0: f64,
    pub time1: f64,
}

impl<'a> Scene<'a> {
    pub fn new(
        camera: Camera,
        width: u16,
        height: u16,
        samples: u64,
        world: &'a mut HittableList,
        time0: f64,
        time1: f64,
    ) -> Scene<'a> {
        Scene {
            camera,
            width,
            height,
            samples,
            world: BvhTree::new(&mut world.objects[..], time0, time1),
            time0,
            time1,
        }
    }

    pub fn render(&self, filename: &str) {
        let progressbar = ProgressBar::new(u64::from(self.height));
        progressbar.set_style(ProgressStyle::default_bar().template(
            "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
        ));

        let scene: Vec<Vec<Vector3<f64>>> = (0..self.height)
            .into_par_iter()
            .map(|y_rev| {
                let y: f64 = f64::from(self.height) - f64::from(y_rev) - 1.0;
                let row: Vec<Vector3<f64>> = (0..self.width)
                    .into_par_iter()
                    .map(|x| {
                        let mut color_vector = Vector3::new(0.0, 0.0, 0.0);
                        for _s in 0..self.samples {
                            let u: f64 = (f64::from(x) + random::<f64>()) / f64::from(self.width);
                            let v: f64 = (y + random::<f64>()) / f64::from(self.height);
                            let time = self.time0 + random::<f64>() * (self.time1 - self.time0);
                            let r = self.camera.get_ray(u, v, time);
                            color_vector += lerp(&r, &self.world, 0);
                        }
                        color_vector /= self.samples as f64;
                        color_vector = color_vector.map(|x| x.sqrt()) * 255.99;
                        color_vector
                    }).collect();
                progressbar.inc(1);
                row
            }).collect();

        progressbar.finish();

        write_img(&scene, filename);
    }
}

fn lerp(ray: &Ray, world: &Hittable, depth: i32) -> Vector3<f64> {
    if let Some(hit) = world.hits(ray, 0.001, f64::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, hit.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
                let color = lerp(&scattered, world, depth + 1);
                Vector3::new(
                    color.x * attenuation.x + emitted.x,
                    color.y * attenuation.y + emitted.y,
                    color.z * attenuation.z + emitted.z,
                )
            } else {
                emitted
            }
        } else {
            emitted
        }
    } else {
        vec3::<f64>(0.0, 0.0, 0.0)
    }
}
