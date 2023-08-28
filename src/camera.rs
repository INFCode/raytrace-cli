use crate::render_target::RenderTarget;
use crate::{ray::Ray, world::ray_color};
use indicatif::ProgressIterator;
use nalgebra::{vector, Point2, Vector3};

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

pub struct Camera {
    viewport: Viewport,
    focal_length: f64,
    position: Vector3<f64>,
    pub target: RenderTarget,
}

impl Camera {
    pub fn new(
        viewport_width: f64,
        focal_length: f64,
        position: Vector3<f64>,
        target: RenderTarget,
    ) -> Camera {
        let viewport = Viewport::new(target.real_ratio(), viewport_width, focal_length);
        //dbg!(viewport.u());
        //dbg!(viewport.v());
        Camera {
            viewport,
            focal_length,
            position,
            target,
        }
    }

    pub fn ray_through(&self, relative_position: Point2<f64>) -> Ray {
        Ray::new(
            self.position.into(),
            self.viewport.vector_to(relative_position),
        )
    }

    pub fn render(&self) {
        self.target.initialize();
        for j in (0..self.target.height()).progress() {
            for i in 0..self.target.width() {
                let relative_position = self.target.relative_position_of_pixel(i, j);
                //dbg!(relative_position);
                let ray = self.ray_through(relative_position);
                self.target.write_pixel(ray_color(&ray));
            }
        }
    }
}
