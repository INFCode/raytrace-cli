use crate::render_target::RenderTarget;
use crate::utils::Interval;
use crate::{color::ColorMixer, ray::Ray, world::hittable::Hittable};
use indicatif::ProgressIterator;
use nalgebra::{vector, Point2, Vector3};
use rand::random;

use crate::color::{self, Color};

pub struct Viewport {
    width: f64,
    height: f64,
    focal_length: f64,
}

impl Viewport {
    pub fn new(ratio: f64, width: f64, focal_length: f64) -> Viewport {
        let height = width / ratio;
        Viewport {
            width,
            height,
            focal_length,
        }
    }

    pub fn u(&self) -> Vector3<f64> {
        vector![self.width, 0f64, 0f64]
    }

    pub fn v(&self) -> Vector3<f64> {
        vector![0f64, -self.height, 0f64]
    }

    pub fn right(&self) -> Vector3<f64> {
        self.u()
    }

    pub fn down(&self) -> Vector3<f64> {
        self.v()
    }

    pub fn vector_to(&self, relative_position: Point2<f64>) -> Vector3<f64> {
        self.u() * (relative_position.x - 0.5f64) + self.v() * (relative_position.y - 0.5f64)
            - vector![0f64, 0f64, self.focal_length]
    }
}

pub struct Camera<M: ColorMixer> {
    viewport: Viewport,
    position: Vector3<f64>,
    pub target: RenderTarget,
    pub sample_per_pixel: usize,
    mixer: M,
}

impl<M: ColorMixer> Camera<M> {
    pub fn new(
        viewport_width: f64,
        focal_length: f64,
        position: Vector3<f64>,
        target: RenderTarget,
        sample_per_pixel: usize,
        mixer: M,
    ) -> Self {
        let viewport = Viewport::new(target.real_ratio(), viewport_width, focal_length);
        //dbg!(viewport.u());
        //dbg!(viewport.v());
        Self {
            viewport,
            position,
            target,
            sample_per_pixel,
            mixer,
        }
    }

    pub fn ray_through(&self, relative_position: Point2<f64>) -> Ray {
        Ray::new(
            self.position.into(),
            self.viewport.vector_to(relative_position),
        )
    }

    pub fn render<W: Hittable>(&mut self, world: &W) {
        self.target.initialize();
        let max_depth = 50;
        for j in (0..self.target.height()).progress() {
            for i in 0..self.target.width() {
                for _s in 0..self.sample_per_pixel {
                    let random_offset = [random::<f64>() - 0.5, random::<f64>() - 0.5];
                    let relative_position = self.target.relative_position_of_pixel(
                        i as f64 + random_offset[0],
                        j as f64 + random_offset[1],
                    );
                    //dbg!(relative_position);
                    let ray = self.ray_through(relative_position);
                    let color = Camera::<M>::ray_color(&ray, world, max_depth);
                    self.mixer.add(&color);
                }
                self.target.write_pixel(self.mixer.mix());
            }
        }
    }

    fn ray_color<W: Hittable>(ray: &Ray, world: &W, depth: i32) -> Color {
        if depth <= 0 {
            // too many reflections, no light remaining
            return Color::from_hex(0x000000);
        }
        if let Some(hit_rec) = world.hit(ray, &Interval::non_neg()) {
            if let Some(scatter_rec) = hit_rec.mat.scatter(ray, &hit_rec) {
                return Camera::<M>::ray_color(&scatter_rec.scattered, world, depth - 1)
                    .attenute(&scatter_rec.attenuation_factor);
            }
            // error color
            return Color::from_hex(0x6000a0);
        }
        // miss, background color
        let dir = ray.direction.normalize();
        let t = 0.5 * (dir.y + 1f64);
        color::lerp(
            &Color::new(1f64, 1f64, 1f64),
            &Color::new(0.5f64, 0.7f64, 1.0f64),
            t,
        )
    }
}
