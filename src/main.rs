#![allow(dead_code)]
mod camera;
mod color;
mod materials;
mod output;
mod ray;
mod render_spec;
mod utils;
mod world;

use crate::camera::Camera;
use crate::color::LinearMixer;
use crate::materials::Material;
use crate::materials::{
    DielectricMaterial, LambertianMaterial, MetalMaterial, SimpleDiffuseMaterial,
};
use crate::render_spec::{ImageSize, PinHoleSpec};
use crate::world::{Hittable, Sphere};
use glam::DVec3;

fn main() {
    let spp = 500;
    let fov = 135f64;

    let spec = PinHoleSpec::new(
        spp,
        fov,
        ImageSize {
            width: 600,
            height: 400,
        },
    );

    let camera = Camera::new(DVec3::ZERO);

    // materials
    let simple = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
    let lambertian =
        Box::new(LambertianMaterial::new(DVec3::new(0.2, 0.8, 0.1))) as Box<dyn Material>;
    let metal = Box::new(MetalMaterial::new(DVec3::splat(0.9), 0f64)) as Box<dyn Material>;
    let metal_fuzz =
        Box::new(MetalMaterial::new(DVec3::new(0.8, 0.6, 0.2), 0.6)) as Box<dyn Material>;
    let dielectric = Box::new(DielectricMaterial::new(1.5)) as Box<dyn Material>;

    // objects
    let s1 = Sphere::new(DVec3::new(0f64, 0f64, -1f64), 0.5, &lambertian);
    let s2 = Sphere::new(DVec3::new(-1f64, 0f64, -1f64), 0.5, &dielectric);
    let s3 = Sphere::new(DVec3::new(1f64, 0f64, -1f64), 0.5, &metal_fuzz);
    let s4 = Sphere::new(DVec3::new(0f64, 1f64, -1f64), 0.5, &metal);
    let gnd = Sphere::new(DVec3::new(0f64, -100.5, -1f64), 100f64, &simple);

    // world
    let world = vec![
        Box::new(s1) as Box<dyn Hittable>,
        Box::new(s2) as Box<dyn Hittable>,
        Box::new(s3) as Box<dyn Hittable>,
        Box::new(s4) as Box<dyn Hittable>,
        Box::new(gnd) as Box<dyn Hittable>,
    ];

    let buffer = camera.render::<LinearMixer>(&spec, &world);
    buffer.save("test.png").unwrap();
}
