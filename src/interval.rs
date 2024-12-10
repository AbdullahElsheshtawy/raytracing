#[derive(Debug, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            max: f32::INFINITY,
            min: f32::NEG_INFINITY,
        }
    }
}
