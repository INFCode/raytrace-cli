use crate::render_target::RenderTarget;
use nalgebra::{vector, Vector3};

pub struct Viewport {
    width: f64,
    height: f64,
    position: Vector3<f64>,
}

impl Viewport {
    pub fn new(ratio: f64, width: f64, position: Vector3<f64>) -> Viewport {
        let height = width / ratio;
        Viewport {
            width,
            height,
            position,
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

    pub fn vector_to(&self, x: f64, y: f64) -> Vector3<f64> {
        self.position + self.u() * (x - 0.5f64) + self.v() * (y - 0.5f64)
    }
}

pub struct Camera {
    viewport: Viewport,
    focal_length: f64,
    position: Vector3<f64>,
    target: RenderTarget,
}

impl Camera {
    pub fn new(
        viewport_width: f64,
        viewport_ratio: f64,
        focal_length: f64,
        position: Vector3<f64>,
        target: RenderTarget,
    ) -> Camera {
        Camera {
            viewport: Viewport::new(
                viewport_width,
                viewport_ratio,
                vector![0f64, focal_length, 0f64],
            ),
            focal_length,
            position,
            target,
        }
    }
}
