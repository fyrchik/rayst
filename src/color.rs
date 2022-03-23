use std::fmt;

use crate::vec3::Vec3;

#[repr(transparent)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3 { x: r, y: g, z: b })
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            (255.999 * self.0.x) as u32,
            (255.999 * self.0.y) as u32,
            (255.999 * self.0.z) as u32
        )
    }
}
