#[derive(Clone, Copy)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            lower: min,
            upper: max,
        }
    }

    pub fn empty() -> Self {
        Self {
            lower: f64::INFINITY,
            upper: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Self {
        Self {
            lower: f64::NEG_INFINITY,
            upper: f64::INFINITY,
        }
    }

    pub fn non_neg() -> Self {
        Self {
            lower: 0f64,
            upper: f64::INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.lower <= x && x <= self.upper
    }

    pub fn surround(&self, x: f64) -> bool {
        self.lower < x && x < self.upper
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.min(self.upper).max(self.lower)
    }
}
