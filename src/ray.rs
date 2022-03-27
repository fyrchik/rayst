use crate::vec3::{Point, Vec3};

#[derive(Default)]
pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3, time: f64) -> Self {
        Self { orig, dir, time }
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
