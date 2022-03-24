use crate::vec3::{Point, Vec3};

#[derive(Default)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
