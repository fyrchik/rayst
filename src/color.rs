use std::ops::{Add, AddAssign, Mul};

use crate::vec3::Vec3;

#[derive(Clone, Debug, Default)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn new_from_vec3(v: Vec3) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }

    pub fn adjust_and_format(&self, samples: u32) -> String {
        let scale = 1.0 / samples as f64;
        let r = self.r * scale;
        let g = self.g * scale;
        let b = self.b * scale;
        format!(
            "{} {} {}\n",
            (256.0 * r.clamp(0.0, 0.999)) as u32,
            (256.0 * g.clamp(0.0, 0.999)) as u32,
            (256.0 * b.clamp(0.0, 0.999)) as u32
        )
    }
}

impl Add for Color {
    type Output = Color;

    #[inline]
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    #[inline]
    fn mul(self, v: Color) -> Color {
        Color {
            r: self * v.r,
            g: self * v.g,
            b: self * v.b,
        }
    }
}
