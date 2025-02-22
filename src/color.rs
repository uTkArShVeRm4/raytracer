use crate::{interval::Interval, utils::linear_to_gamma, vector::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn to_string(&self) -> String {
        let intensity = Interval::new(0.0, 1.0);
        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());

        let rbyte = (255.0 * intensity.clamp(r)) as u32;
        let gbyte = (255.0 * intensity.clamp(g)) as u32;
        let bbyte = (255.0 * intensity.clamp(b)) as u32;
        format!("{} {} {}\n", rbyte, gbyte, bbyte)
    }
}
