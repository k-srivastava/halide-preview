#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Self { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }

    pub fn universe() -> Self {
        Self { min: f64::NEG_INFINITY, max: f64::INFINITY }
    }

    pub fn new(min: f64, max: f64) -> Self { Self { min, max } }

    pub fn contains(&self, x: f64) -> bool { self.min <= x && x <= self.max }

    pub fn surrounds(&self, x: f64) -> bool { self.min < x && x < self.max }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        x
    }
}
