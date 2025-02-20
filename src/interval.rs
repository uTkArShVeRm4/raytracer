use crate::constants::{INFINITY, NEG_INFINITY};
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn clamp(&self, x: f64) -> f64 {
        // Its hilarious how smart this is but also how unreadable it is
        x.max(self.min).min(self.max)
    }
}
