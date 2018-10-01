extern crate cgmath;
extern crate indicatif;
extern crate rand;
extern crate rayon;

extern crate raytrac;

use raytrac::*;

use io::write::write_img;
use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use materials::Material;
use materials::Scatterable;
use objects::camera::Camera;
use objects::sphere::Sphere;
use objects::{Hittable, HittableList};
use ray::Ray;

use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use std::f64;

fn main() {
    let filename = "output/sample.png";

    let nx: i16 = 200;
    let ny: i16 = 100;
    let ns: u64 = 100;

    let cam: Camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(nx) / f64::from(ny),
        0.1,
        10.0,
    );

    let world: HittableList = random_scene();

    let bar = ProgressBar::new(ny as u64);
    bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));

    let scene: Vec<Vec<Vector3<f64>>> = (0..ny)
        .into_par_iter()
        .map(|y_rev| {
            let y: f64 = f64::from(ny) - f64::from(y_rev) - 1.0;
            let row: Vec<Vector3<f64>> = (0..nx)
                .into_par_iter()
                .map(|x| {
                    let mut color_vector = Vector3::new(0.0, 0.0, 0.0);
                    for _s in 0..ns {
                        let u: f64 = (f64::from(x) + rand::random::<f64>()) / f64::from(nx);
                        let v: f64 = (y + rand::random::<f64>()) / f64::from(ny);
                        let r: Ray = cam.get_ray(u, v);
                        color_vector += lerp(&r, &world, 0);
                    }
                    color_vector /= ns as f64;
                    color_vector = color_vector.map(|x| x.sqrt()) * 255.99;
                    color_vector
                }).collect();
            bar.inc(1);
            row
        }).collect();

    bar.finish();

    write_img(&scene, filename);
}

fn lerp(ray: &Ray, world: &Hittable, depth: i32) -> Vector3<f64> {
    if let Some(hit) = world.hits(ray, 0.001, f64::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
                let color = lerp(&scattered, world, depth + 1);
                Vector3::new(
                    color.x * attenuation.x,
                    color.y * attenuation.y,
                    color.z * attenuation.z,
                )
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut list: HittableList = HittableList::new();
    list.add(Sphere::from(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Vector3::new(0.5, 0.5, 0.5))),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vector3::new(4.0, 0.2, 0.0)).to_vec().magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    list.add(Sphere::from(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(Vector3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    ));
                } else if choose_mat < 0.95 {
                    //metal
                    list.add(Sphere::from(
                        center,
                        0.2,
                        Material::Metal(Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        )),
                    ));
                } else {
                    // dielectric
                    list.add(Sphere::from(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::from(1.5)),
                    ));
                }
            }

            list.add(Sphere::from(
                Point3::new(0.0, 1.0, 0.0),
                1.0,
                Material::Dielectric(Dielectric::from(1.5)),
            ));
            list.add(Sphere::from(
                Point3::new(-4.0, 1.0, 0.0),
                1.0,
                Material::Lambertian(Lambertian::new(Vector3::new(0.4, 0.4, 0.1))),
            ));
            list.add(Sphere::from(
                Point3::new(4.0, 1.0, 0.0),
                1.0,
                Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
            ));
        }
    }

    list
}
