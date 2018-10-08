extern crate cgmath;
extern crate indicatif;
extern crate rand;
extern crate rayon;

extern crate raytrac;

use raytrac::*;

use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;
use materials::Material;
use objects::camera::Camera;
use objects::sphere::Sphere;
use objects::HittableList;
use scene::Scene;

use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;
use rand::prelude::*;

use std::f64;

fn main() {
    const WIDTH: u16 = 1200;
    const HEIGHT: u16 = 800;
    const SAMPLES: u64 = 500;

    let camera = Camera::new(
        Point3::new(13.0, 0.5, 5.0),
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(WIDTH) / f64::from(HEIGHT),
        0.1,
        10.0,
    );

    let mut world = random_scene();

    let scene = Scene::new(camera, WIDTH, HEIGHT, SAMPLES, &mut world, 0.0, 1.0);
    scene.render("output/sample.png")
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut list: HittableList = HittableList::new();
    list.add(Box::new(Sphere::from(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Vector3::new(0.5, 0.5, 0.5))),
    )));

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
                    list.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(Vector3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    list.add(Box::new(Sphere::from(
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
                    )));
                } else {
                    // dielectric
                    list.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::from(1.5)),
                    )));
                }
            }

            list.add(Box::new(Sphere::from(
                Point3::new(0.0, 1.0, 0.0),
                1.0,
                Material::Dielectric(Dielectric::from(1.5)),
            )));
            list.add(Box::new(Sphere::from(
                Point3::new(-4.0, 1.0, 0.0),
                1.0,
                Material::Lambertian(Lambertian::new(Vector3::new(0.4, 0.4, 0.1))),
            )));
            list.add(Box::new(Sphere::from(
                Point3::new(4.0, 1.0, 0.0),
                1.0,
                Material::Metal(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
            )));
        }
    }

    list
}
