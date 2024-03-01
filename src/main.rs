#![allow(dead_code)]
mod camera;
mod color;
mod materials;
mod output;
mod ray;
mod render_spec;
#[cfg(test)]
mod test_utils;
mod utils;
mod world;

use crate::camera::Camera;
use crate::color::LinearMixer;
use crate::materials::{
    DielectricMaterial, LambertianMaterial, Material, MetalMaterial, SimpleDiffuseMaterial,
};
use crate::output::{ImageFormatsSaver, ImageSaver};
use crate::render_spec::{ImageSize, PinHoleSpec};
use crate::world::{InfinitePlane, Intersectable, Rectangle, Sphere, VecContainer};
use color::LinearRgbColor;
use glam::{DQuat, DVec3, EulerRot};
use world::LerpScene;

fn main() {
    let spp = 500;
    let fov = 135f64;

    let spec = PinHoleSpec::new(
        spp,
        fov,
        ImageSize {
            width: 300,
            height: 200,
        },
    );

    let camera = Camera::new(
        DVec3::Z * 0.3,
        DQuat::from_euler(glam::EulerRot::XYZ, 15f64.to_radians(), 0f64, 0f64),
    );

    // materials
    let simple = SimpleDiffuseMaterial::make_shared(SimpleDiffuseMaterial::new());
    let lambertian =
        LambertianMaterial::make_shared(LambertianMaterial::new(DVec3::new(0.2, 0.8, 0.1)));
    let metal = MetalMaterial::make_shared(MetalMaterial::new(DVec3::splat(0.9), 0f64));
    let metal_fuzz = MetalMaterial::make_shared(MetalMaterial::new(DVec3::new(0.8, 0.6, 0.2), 0.6));
    let dielectric = DielectricMaterial::make_shared(DielectricMaterial::new(1.5));

    // objects
    let s1 = Sphere::new(DVec3::new(0f64, 0f64, -1f64), 0.5, &lambertian);
    let s2 = Sphere::new(DVec3::new(-1f64, 0f64, -1f64), 0.5, &dielectric);
    let s3 = Sphere::new(DVec3::new(1f64, 0f64, -1f64), 0.5, &metal_fuzz);
    let s4 = Sphere::new(DVec3::new(0f64, 1f64, -1f64), 0.5, &metal);
    //let gnd = Sphere::new(DVec3::new(0f64, -100.5, -1f64), 100f64, &simple);
    let gnd = InfinitePlane::new(
        DVec3::new(0f64, -1.5, 0f64),
        DVec3::new(0f64, 1f64, 0f64),
        &simple,
    );

    let mirror = Rectangle::new(
        DVec3::new(-1f64, 1.5, -1f64),
        DQuat::from_euler(EulerRot::XYZ, 0f64, 0f64, -135f64.to_radians()),
        2.5f64,
        2.5f64,
        &simple,
    );

    // world
    let container = VecContainer::from_iter(vec![
        s1.into_box(),
        s2.into_box(),
        s3.into_box(),
        s4.into_box(),
        gnd.into_box(),
        mirror.into_box(),
    ]);

    let world = LerpScene::new(
        container,
        LinearRgbColor::new(1f64, 1f64, 1f64),
        LinearRgbColor::new(0.5f64, 0.7f64, 1.0f64),
    );

    let buffer = camera.render::<LinearMixer>(&spec, &world);
    let saver = ImageFormatsSaver::new();
    saver.save_to(&buffer, "test_saver.png");
}
