use std::fmt;
use std::ops::{Add, Mul};

use crate::vec3::Vec3;

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
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {} {}",
            (255.999 * self.r) as u32,
            (255.999 * self.g) as u32,
            (255.999 * self.b) as u32
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
