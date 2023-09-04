#![allow(dead_code)]
mod camera;
mod character;
mod color;
mod materials;
mod ray;
mod render_target;
mod utils;
mod world;

use crate::camera::Camera;
use crate::color::RMSMixer;
use crate::materials::Material;
use crate::materials::{LambertianMaterial, SimpleDiffuseMaterial};
use crate::render_target::RenderTarget;
use crate::world::{Hittable, Sphere};
use nalgebra::Vector3;
use nalgebra::{vector, Point3};

fn main() {
    // Image
    let image_width = 100;
    let aspect_ratio = 4f64 / 3f64;

    let spp = 5;

    let simple = Box::new(SimpleDiffuseMaterial::new()) as Box<dyn Material>;
    let lambertian =
        Box::new(LambertianMaterial::new(Vector3::from_element(0.6))) as Box<dyn Material>;

    {
        let s1 = Sphere::new(Point3::new(0f64, 0f64, -1f64), 0.5, &lambertian);
        let s2 = Sphere::new(Point3::new(0f64, -100.5, -1f64), 100f64, &simple);
        let world = vec![
            Box::new(s1) as Box<dyn Hittable>,
            Box::new(s2) as Box<dyn Hittable>,
        ];

        let image = RenderTarget::new(image_width, aspect_ratio);
        let mixer = RMSMixer::new();
        dbg!(image.width());
        dbg!(image.height());
        dbg!(image.real_ratio());
        dbg!(image.aspect_ratio());
        let mut camera = Camera::new(2f64, 1f64, vector![0f64, 0f64, 0f64], image, spp, mixer);

        // Render
        //for j in (0..image.height()).progress() {
        //    for i in 0..image.width() {
        //        let r = i as f64 / (image.width() - 1) as f64;
        //        let g = j as f64 / (image.height() - 1) as f64;
        //        let b = 0f64;
        //        let color = Color::new(r, g, b);

        //        println!("{}", color);
        //    }
        //}
        camera.render(&world);
        println!("{:?} {:?}", simple, lambertian);
    }
}
