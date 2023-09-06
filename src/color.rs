use nalgebra::{vector, Vector3};
use std::default::Default;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Color {
    color: Vector3<f64>,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: vector![r, g, b],
        }
    }
    pub fn from_hex(hex: u32) -> Color {
        let mask = 0xff;
        let b = (hex & mask) as f64 / 256f64;
        let g = ((hex >> 8) & mask) as f64 / 256f64;
        let r = ((hex >> 16) & mask) as f64 / 256f64;
        Self::new(r, g, b)
    }

    pub fn from_vec(v: &Vector3<f64>) -> Color {
        Color { color: v.clone() }
    }
    pub fn r(&self) -> f64 {
        self.color[0]
    }
    pub fn g(&self) -> f64 {
        self.color[1]
    }
    pub fn b(&self) -> f64 {
        self.color[2]
    }

    pub fn attenute_mut(&mut self, scale: &Vector3<f64>) {
        self.color.component_mul_assign(scale);
    }

    pub fn attenute(&self, scale: &Vector3<f64>) -> Self {
        let mut copy = self.clone();
        copy.attenute_mut(scale);
        copy
    }

    pub fn lerp(c1: &Color, c2: &Color, t: f64) -> Color {
        let r = c1.r() * (1f64 - t) + c2.r() * t;
        let g = c1.g() * (1f64 - t) + c2.g() * t;
        let b = c1.b() * (1f64 - t) + c2.b() * t;
        Color::new(r, g, b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ri = (255.999 * f64::sqrt(self.r())).trunc() as i64;
        let gi = (255.999 * f64::sqrt(self.g())).trunc() as i64;
        let bi = (255.999 * f64::sqrt(self.b())).trunc() as i64;
        // gamma is set to 2
        write!(f, "{} {} {}", ri, gi, bi)
    }
}

pub trait ColorMixer {
    fn add(&mut self, c: &Color) -> &mut Self;
    fn mix(&mut self) -> Color;
}

pub struct LinearMixer {
    color: Vector3<f64>,
    total_color: usize,
}

impl LinearMixer {
    pub fn new() -> Self {
        Self {
            color: vector![0f64, 0f64, 0f64],
            total_color: 0,
        }
    }
}

impl ColorMixer for LinearMixer {
    fn add(&mut self, c: &Color) -> &mut Self {
        for i in 0..3 {
            self.color[i] += c.color[i];
        }
        self.total_color += 1;
        self
    }

    fn mix(&mut self) -> Color {
        for i in 0..3 {
            self.color[i] /= self.total_color as f64;
        }
        let result = Color { color: self.color };
        self.color = vector![0f64, 0f64, 0f64];
        self.total_color = 0;
        result
    }
}

pub struct RMSMixer {
    color: Vector3<f64>,
    total_color: usize,
}

impl RMSMixer {
    pub fn new() -> Self {
        Self {
            color: vector![0f64, 0f64, 0f64],
            total_color: 0,
        }
    }
}

impl ColorMixer for RMSMixer {
    fn add(&mut self, c: &Color) -> &mut Self {
        for i in 0..3 {
            self.color[i] += c.color[i] * c.color[i];
        }
        self.total_color += 1;
        self
    }

    fn mix(&mut self) -> Color {
        for i in 0..3 {
            self.color[i] = (self.color[i] / self.total_color as f64).sqrt();
        }
        let result = Color { color: self.color };
        self.color = vector![0f64, 0f64, 0f64];
        self.total_color = 0;
        result
    }
}

impl Default for Color {
    fn default() -> Self {
        // default to black
        Self::from_hex(0x0)
    }
}
