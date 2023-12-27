use glam::DVec3;

use crate::color::ColorMixer;
use crate::color::LinearRgbColor;
use crate::ray::Ray;
use crate::render_spec::RenderSpec;
use crate::utils::Interval;
use crate::world::Hittable;
use image::{ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;

pub struct Camera {
    position: DVec3,
}

impl Camera {
    pub fn new(position: DVec3) -> Self {
        Self { position }
    }

    pub fn render<M: ColorMixer>(
        &self,
        render_spec: &impl RenderSpec,
        world: &impl Hittable,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let size = render_spec.image_size();
        let width = size.width as usize;
        let height = size.height as usize;
        let max_depth = 10;

        // Pre-allocating a flat vector to hold all pixel data
        let mut pixels = vec![Rgb([0, 0, 0]); width * height];

        let style = ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}/{duration_precise}] [{bar:40.cyan/blue}] {pos}/{len}={percent}%",
            )
            .unwrap().progress_chars("##-");

        pixels
            .par_chunks_mut(width)
            .progress_with_style(style)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let mut mixer = M::new();
                    let rays = render_spec.ray_for_pixel(x as u32, y as u32);
                    //dbg!(x, y);

                    for ray in rays {
                        //dbg!(ray.direction);
                        let color = Self::ray_color(&ray, world, max_depth);
                        mixer.add(&color);
                    }

                    let final_color = mixer.mix();
                    row[x] = final_color.into();
                }
            });

        ImageBuffer::from_raw(
            size.width,
            size.height,
            pixels.into_iter().flat_map(|rgb| rgb.0.to_vec()).collect(),
        )
        .unwrap()
    }

    fn ray_color<W: Hittable>(ray: &Ray, world: &W, depth: i32) -> LinearRgbColor {
        if depth <= 0 {
            // too many reflections, no light remaining
            return LinearRgbColor::from_hex(0x000000);
        }
        // add a small eps to fix shadow acne
        let eps = 0.001;
        if let Some(hit_rec) = world.hit(ray, &Interval::greater_than(eps)) {
            if let Some(scatter_rec) = hit_rec.mat.scatter(ray, &hit_rec) {
                return Self::ray_color(&scatter_rec.scattered, world, depth - 1)
                    .attenute(scatter_rec.attenuation_factor);
            }
            // error color
            return LinearRgbColor::from_hex(0x6000a0);
        }
        // miss, background color
        let dir = ray.direction.normalize();
        let t = 0.5 * (dir.y + 1f64);
        LinearRgbColor::lerp(
            &LinearRgbColor::new(1f64, 1f64, 1f64),
            &LinearRgbColor::new(0.5f64, 0.7f64, 1.0f64),
            t,
        )
    }
}
