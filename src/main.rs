use indicatif::ProgressIterator;
mod camera;
mod character;
mod color;
mod ray;
mod render_target;
use color::Color;

use crate::render_target::RenderTarget;

fn main() {
    // Image
    let image_width = 256;
    let aspect_ratio = 1f64;

    let image = RenderTarget::new(image_width, aspect_ratio);

    // Render
    print!("P3\n{} {}\n255\n", image.width(), image.height());
    for j in (0..image.height()).progress() {
        for i in 0..image_width {
            let r = i as f64 / (image.width() - 1) as f64;
            let g = j as f64 / (image.height() - 1) as f64;
            let b = 0f64;
            let color = Color::new(r, g, b);

            println!("{}", color);
        }
    }
}
