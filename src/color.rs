use std::fmt::Display;

pub struct Color {
    color: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { color: [r, g, b] }
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.ri(), self.gi(), self.bi())
    }
}
