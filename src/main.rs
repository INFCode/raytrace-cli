mod camera;
mod character;
mod color;
mod ray;
mod render_target;
mod utils;
mod world;
use crate::camera::Camera;
use crate::render_target::RenderTarget;
use crate::world::{hittable::Hittable, sphere::Sphere};
use nalgebra::{vector, Point3};

fn main() {
    // Image
    let image_width = 256;
    let aspect_ratio = 16f64 / 9f64;

    let world = vec![
        Box::new(Sphere::new(Point3::new(0f64, 0f64, -1f64), 0.5)) as Box<dyn Hittable>,
        Box::new(Sphere::new(Point3::new(0f64, -100.5, -1f64), 100f64)) as Box<dyn Hittable>,
    ];

    let image = RenderTarget::new(image_width, aspect_ratio);
    dbg!(image.width());
    dbg!(image.height());
    dbg!(image.real_ratio());
    dbg!(image.aspect_ratio());
    let camera = Camera::new(2f64, 1f64, vector![0f64, 0f64, 0f64], image);

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
}
