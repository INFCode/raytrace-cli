use glam::DQuat;
use glam::DVec3;

use crate::color::ColorMixer;
use crate::color::LinearRgbColor;
use crate::ray::Ray;
use crate::render_spec::RenderSpec;
use crate::utils::Interval;
use crate::world::Scene;
use image::{ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;

pub struct Camera {
    rotation: DQuat,
    position: DVec3,
    max_depth: u32,
}

impl Camera {
    pub fn new(position: DVec3, rotation: DQuat) -> Self {
        Self {
            rotation,
            position,
            max_depth: 10,
        }
    }

    pub fn render<M: ColorMixer>(
        &self,
        render_spec: &impl RenderSpec,
        world: &impl Scene,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let size = render_spec.image_size();
        let width = size.width as usize;
        let height = size.height as usize;

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
                    row[x] = self
                        .render_pixel(&mut M::new(), render_spec, world, x as u32, y as u32)
                        .into();
                }
            });

        ImageBuffer::from_raw(
            size.width,
            size.height,
            pixels.into_iter().flat_map(|rgb| rgb.0.to_vec()).collect(),
        )
        .unwrap()
    }

    fn render_pixel(
        &self,
        mixer: &mut impl ColorMixer,
        render_spec: &impl RenderSpec,
        world: &impl Scene,
        x: u32,
        y: u32,
    ) -> LinearRgbColor {
        let rays = render_spec.ray_for_pixel(x, y);

        for ray in rays {
            let rotated_ray = Ray {
                direction: self.rotation * ray.direction,
                origin: self.position + ray.origin,
            };
            let color = Self::ray_color(&rotated_ray, world, self.max_depth);
            mixer.add(&color);
        }

        mixer.mix()
    }

    fn ray_color<W: Scene>(ray: &Ray, world: &W, depth: u32) -> LinearRgbColor {
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
        world.miss(ray)
    }
}
