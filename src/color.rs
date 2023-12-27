use glam::DVec3;
use image::Rgb;
use std::default::Default;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct LinearRgbColor {
    color: DVec3,
}

impl LinearRgbColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: DVec3::new(r, g, b),
        }
    }
    pub fn from_hex(hex: u32) -> LinearRgbColor {
        let mask = 0xff;
        let b = (hex & mask) as f64 / 256f64;
        let g = ((hex >> 8) & mask) as f64 / 256f64;
        let r = ((hex >> 16) & mask) as f64 / 256f64;
        Self::new(r, g, b)
    }

    pub fn from_vec(v: &DVec3) -> LinearRgbColor {
        LinearRgbColor { color: v.clone() }
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

    pub fn attenute_mut(&mut self, scale: DVec3) {
        self.color *= scale;
    }

    pub fn attenute(&self, scale: DVec3) -> Self {
        let mut copy = self.clone();
        copy.attenute_mut(scale);
        copy
    }

    pub fn lerp(c1: &LinearRgbColor, c2: &LinearRgbColor, t: f64) -> LinearRgbColor {
        let r = c1.r() * (1f64 - t) + c2.r() * t;
        let g = c1.g() * (1f64 - t) + c2.g() * t;
        let b = c1.b() * (1f64 - t) + c2.b() * t;
        LinearRgbColor::new(r, g, b)
    }
}

impl Display for LinearRgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ri = (255.999 * f64::sqrt(self.r())).trunc() as i64;
        let gi = (255.999 * f64::sqrt(self.g())).trunc() as i64;
        let bi = (255.999 * f64::sqrt(self.b())).trunc() as i64;
        // gamma is set to 2
        write!(f, "{} {} {}", ri, gi, bi)
    }
}

pub trait ColorMixer {
    fn new() -> Self;
    fn add(&mut self, c: &LinearRgbColor) -> &mut Self;
    fn mix(&mut self) -> LinearRgbColor;
}

pub struct LinearMixer {
    color: DVec3,
    total_color: usize,
}

impl ColorMixer for LinearMixer {
    fn new() -> Self {
        Self {
            color: DVec3::ZERO,
            total_color: 0,
        }
    }
    fn add(&mut self, c: &LinearRgbColor) -> &mut Self {
        for i in 0..3 {
            self.color[i] += c.color[i];
        }
        self.total_color += 1;
        self
    }

    fn mix(&mut self) -> LinearRgbColor {
        for i in 0..3 {
            self.color[i] /= self.total_color as f64;
        }
        let result = LinearRgbColor { color: self.color };
        self.color = DVec3::ZERO;
        self.total_color = 0;
        result
    }
}

pub struct RMSMixer {
    color: DVec3,
    total_color: usize,
}

impl ColorMixer for RMSMixer {
    fn new() -> Self {
        Self {
            color: DVec3::ZERO,
            total_color: 0,
        }
    }

    fn add(&mut self, c: &LinearRgbColor) -> &mut Self {
        for i in 0..3 {
            self.color[i] += c.color[i] * c.color[i];
        }
        self.total_color += 1;
        self
    }

    fn mix(&mut self) -> LinearRgbColor {
        for i in 0..3 {
            self.color[i] = (self.color[i] / self.total_color as f64).sqrt();
        }
        let result = LinearRgbColor { color: self.color };
        self.color = DVec3::ZERO;
        self.total_color = 0;
        result
    }
}

impl Default for LinearRgbColor {
    fn default() -> Self {
        // default to black
        Self::from_hex(0x0)
    }
}

impl Into<image::Rgb<u8>> for LinearRgbColor {
    fn into(self) -> image::Rgb<u8> {
        // image::Rgb assumes sRGB color, use gamma = 2 here.
        Rgb([
            (f64::sqrt(self.r()) * 255f64).trunc() as u8,
            (f64::sqrt(self.g()) * 255f64).trunc() as u8,
            (f64::sqrt(self.b()) * 255f64).trunc() as u8,
        ])
    }
}
