use crate::vector::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn to_string(&self) -> String {
        let r = (self.x() * 255.999).clamp(0.0, 255.0) as u32;
        let g = (self.y() * 255.999).clamp(0.0, 255.0) as u32;
        let b = (self.z() * 255.999).clamp(0.0, 255.0) as u32;
        format!("{} {} {}\n", r, g, b)
    }
}
