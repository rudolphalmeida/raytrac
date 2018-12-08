use raytrac::materials::dielectric::Dielectric;
use raytrac::materials::lambertian::Lambertian;
use raytrac::materials::light::DiffuseLight;
use raytrac::materials::metal::Metal;
use raytrac::materials::Material;
use raytrac::objects::camera::Camera;
use raytrac::objects::sphere::Sphere;
use raytrac::objects::HittableList;
use raytrac::scene::Scene;

use cgmath::prelude::*;
use cgmath::Point3;
use cgmath::Vector3;
use rand::prelude::*;

use std::f64;
use std::sync::Arc;

fn main() {
    const WIDTH: u16 = 350;
    const HEIGHT: u16 = 200;
    const SAMPLES: u64 = 200;

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    // let dist_to_focus = (look_from - look_at).magnitude();
    let dist_to_focus = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(WIDTH) / f64::from(HEIGHT),
        0.1,
        dist_to_focus,
    );

    let mut world = random_scene();

    let scene = Scene::new(camera, WIDTH, HEIGHT, SAMPLES, &mut world, 0.0, 1.0);
    scene.render("output/sample.png")
}

fn random_scene() -> HittableList {
    let mut rng = thread_rng();

    let mut list: HittableList = HittableList::new();
    list.add(Box::new(Sphere::from(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Material::Lambertian(Lambertian::color(0.4, 0.2, 0.4))),
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
                        Arc::new(Material::Lambertian(Lambertian::color(
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
                        Arc::new(Material::Metal(Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        ))),
                    )));
                } else {
                    // dielectric
                    list.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        Arc::new(Material::Dielectric(Dielectric::from(1.5))),
                    )));
                }
            }
        }
    }

    list.add(Box::new(Sphere::from(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Material::Dielectric(Dielectric::from(1.5))),
    )));
    list.add(Box::new(Sphere::from(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Material::DiffuseLight(DiffuseLight::color(4.0, 4.0, 4.0))),
    )));
    list.add(Box::new(Sphere::from(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Material::Metal(Metal::new(
            Vector3::new(0.7, 0.6, 0.5),
            0.0,
        ))),
    )));

    list
}
