use std::fmt::Display;

pub struct Color {
    color: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { color: [r, g, b] }
    }
    pub fn from_hex(hex: u32) -> Color {
        let mask = 0xff;
        let b = (hex & mask) as f64 / 256f64;
        let g = ((hex >> 8) & mask) as f64 / 256f64;
        let r = ((hex >> 16) & mask) as f64 / 256f64;
        Self::new(r, g, b)
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
    pub fn ri(&self) -> i64 {
        (255.999 * self.r()).trunc() as i64
    }
    pub fn gi(&self) -> i64 {
        (255.999 * self.g()).trunc() as i64
    }
    pub fn bi(&self) -> i64 {
        (255.999 * self.b()).trunc() as i64
    }
}

pub fn lerp(c1: &Color, c2: &Color, t: f64) -> Color {
    let r = c1.r() * (1f64 - t) + c2.r() * t;
    let g = c1.g() * (1f64 - t) + c2.g() * t;
    let b = c1.b() * (1f64 - t) + c2.b() * t;
    Color::new(r, g, b)
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.ri(), self.gi(), self.bi())
    }
}

pub trait ColorMixer {
    fn add(&mut self, c: &Color) -> &mut Self;
    fn mix(&mut self) -> Color;
}

pub struct LinearMixer {
    color: [f64; 3],
    total_color: usize,
}

impl LinearMixer {
    pub fn new() -> Self {
        Self {
            color: [0f64, 0f64, 0f64],
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
        self.color = [0f64, 0f64, 0f64];
        self.total_color = 0;
        result
    }
}

pub struct RMSMixer {
    color: [f64; 3],
    total_color: usize,
}

impl RMSMixer {
    pub fn new() -> Self {
        Self {
            color: [0f64, 0f64, 0f64],
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
        self.color = [0f64, 0f64, 0f64];
        self.total_color = 0;
        result
    }
}
