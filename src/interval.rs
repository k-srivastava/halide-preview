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

    pub fn from_interval_bounds(interval1: &Self, interval2: &Self) -> Self {
        Self { min: interval1.min.min(interval2.min), max: interval1.max.max(interval2.max) }
    }

    pub fn size(&self) -> f64 { self.max - self.min }

    pub fn contains(&self, x: f64) -> bool { self.min <= x && x <= self.max }

    pub fn surrounds(&self, x: f64) -> bool { self.min < x && x < self.max }

    pub fn expand(&self, delta: f64)  -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min; }
        if x > self.max { return self.max; }
        x
    }
}
