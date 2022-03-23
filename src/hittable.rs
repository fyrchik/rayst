use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

// Hittable represents class of objects which can be intersected by a ray.
pub trait Hittable {
    // hit must return true if r intersects an object at point t
    // such that t_min < t < t_max.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
