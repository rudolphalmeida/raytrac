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

use std::f64;
use std::sync::Arc;

fn main() {
    const WIDTH: u16 = 1920;
    const HEIGHT: u16 = 1080;
    const SAMPLES: u64 = 500;

    let look_from = Point3::new(0.0, 4.0, 5.0);
    let look_at = Point3::new(0.0, 0.0, -5.0);
    let dist_to_focus = (look_from - look_at).magnitude();

    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(WIDTH) / f64::from(HEIGHT),
        0.0,
        dist_to_focus,
    );

    let mut world = random_scene();

    let scene = Scene::new(camera, WIDTH, HEIGHT, SAMPLES, &mut world, 0.0, 1.0);
    scene.render("output/sample.png")
}

fn random_scene() -> HittableList {
    let mut hitable_list = HittableList::new();

    //red
    let _red = Arc::new(Material::Lambertian(Lambertian::color(1.0, 0.0, 0.0)));
    //green
    let _green = Arc::new(Material::Lambertian(Lambertian::color(0.0, 1.0, 0.0)));
    //blue
    let _blue = Arc::new(Material::Lambertian(Lambertian::color(0.0, 0.0, 1.0)));
    //white
    let _white = Arc::new(Material::Lambertian(Lambertian::color(1.0, 1.0, 1.0)));
    //red
    let _red_metal = Arc::new(Material::Metal(Metal::new(
        Vector3::new(1.0, 0.0, 0.0),
        0.1,
    )));
    //green
    let _green_metal = Arc::new(Material::Metal(Metal::new(
        Vector3::new(0.0, 1.0, 0.0),
        0.1,
    )));
    //blue
    let _blue_metal = Arc::new(Material::Metal(Metal::new(
        Vector3::new(0.0, 0.0, 1.0),
        0.1,
    )));
    //white
    let _white_metal = Arc::new(Material::Metal(Metal::new(
        Vector3::new(1.0, 1.0, 1.0),
        0.1,
    )));
    //black \m/
    let _black_metal = Arc::new(Material::Metal(Metal::new(
        Vector3::new(0.0, 0.0, 0.0),
        0.1,
    )));
    //black
    let _black = Arc::new(Material::Lambertian(Lambertian::color(0.0, 0.0, 0.0)));
    //gold
    let _gold = Arc::new(Material::Metal(Metal::new(
        Vector3::new(0.8, 0.6, 0.2),
        0.3,
    )));
    //silver
    let _silver = Arc::new(Material::Metal(Metal::new(
        Vector3::new(0.8, 0.8, 0.8),
        0.2,
    )));
    //glass
    let _glass = Arc::new(Material::Dielectric(Dielectric::from(1.52)));

    //jules farben
    let _color1 = Arc::new(Material::Lambertian(Lambertian::color(0.5176, 0.4392, 1.0)));
    let _color2 = Arc::new(Material::Lambertian(Lambertian::color(
        0.8039, 0.3607, 0.3607,
    )));
    let _color3 = Arc::new(Material::Lambertian(Lambertian::color(0.6, 0.1960, 0.8)));
    let _color4 = Arc::new(Material::Lambertian(Lambertian::color(
        0.8666, 0.6274, 0.8666,
    )));
    let _color5 = Arc::new(Material::Lambertian(Lambertian::color(0.6901, 0.8862, 1.0)));
    let _color6 = Arc::new(Material::Metal(Metal::new(
        Vector3::new(0.0, 1.0, 0.6039),
        0.39,
    )));
    let _color7 = Arc::new(Material::Metal(Metal::new(
        Vector3::new(0.6039, 1.0, 0.6039),
        0.67,
    )));
    let _floor = Arc::new(Material::Lambertian(Lambertian::color(
        0.2117, 0.2117, 0.2117,
    )));

    hitable_list.add(Box::new(Sphere::from(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        _floor.clone(),
    )));
    for y in 0..15 {
        for x in 0..15 {
            let num = y * 15 + x;
            hitable_list.add(Box::new(Sphere::from(
                Point3::new(f64::from(x) - 15.0 / 2.0, 0.0, f64::from(-(y + 1))),
                0.5,
                match num % 7 {
                    0 => _color1.clone(),
                    1 => _color2.clone(),
                    2 => _color3.clone(),
                    3 => _color4.clone(),
                    4 => _color5.clone(),
                    5 => _color6.clone(),
                    _ => _color7.clone(),
                },
            )));
        }
    }

    hitable_list
}
