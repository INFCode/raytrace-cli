#![allow(dead_code)]
mod camera;
mod color;
mod light_source;
mod materials;
mod output;
mod ray;
mod utils;
mod world;

use crate::camera::Camera;
use crate::color::LinearMixer;
use crate::light_source::{ImageSize, PinHoleSpec};
use crate::materials::Material;
use crate::materials::{
    DielectricMaterial, LambertianMaterial, MetalMaterial, SimpleDiffuseMaterial,
};
use crate::output::{ImageTarget, RenderTarget};
use crate::world::{Hittable, Sphere};
use nalgebra::{vector, Point3, Vector3};

fn main() {
    // Image
    let image_width = 1200;
    let aspect_ratio = 4f64 / 3f64;

    let spp = 500;

    let simple = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
    let lambertian = Box::new(LambertianMaterial::new(vector![0.2, 0.8, 0.1])) as Box<dyn Material>;
    let metal = Box::new(MetalMaterial::new(Vector3::from_element(0.9), 0f64)) as Box<dyn Material>;
    let metal_fuzz = Box::new(MetalMaterial::new(vector![0.8, 0.6, 0.2], 0.6)) as Box<dyn Material>;
    let dielectric = Box::new(DielectricMaterial::new(1.5)) as Box<dyn Material>;

    let s1 = Sphere::new(Point3::new(0f64, 0f64, -1f64), 0.5, &lambertian);
    let s2 = Sphere::new(Point3::new(-1f64, 0f64, -1f64), 0.5, &dielectric);
    let s3 = Sphere::new(Point3::new(1f64, 0f64, -1f64), 0.5, &metal_fuzz);
    let s4 = Sphere::new(Point3::new(0f64, 1f64, -1f64), 0.5, &metal);
    let gnd = Sphere::new(Point3::new(0f64, -100.5, -1f64), 100f64, &simple);

    let world = vec![
        Box::new(s1) as Box<dyn Hittable>,
        Box::new(s2) as Box<dyn Hittable>,
        Box::new(s3) as Box<dyn Hittable>,
        Box::new(s4) as Box<dyn Hittable>,
        Box::new(gnd) as Box<dyn Hittable>,
    ];

    let image = ImageTarget::new(image_width, aspect_ratio);
    let spec = PinHoleSpec::new(
        spp,
        135f64,
        ImageSize {
            width: 600,
            height: 400,
        },
    );

    dbg!(image.width());
    dbg!(image.height());
    dbg!(image.actual_aspect_ratio());
    dbg!(image.theoretical_aspect_ratio());
    //let mut camera = Camera::new(2f64, 0.5f64, vector![0f64, 0f64, 0f64], image, spp, mixer);
    let camera = Camera::new(vector![0f64, 0f64, 0f64]);

    let buffer = camera.render::<LinearMixer, _, _>(&spec, &world);
    buffer.save("test.png").unwrap();
}
