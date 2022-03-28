use crate::{
    aabb::AABB,
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub material: Rc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        t: f64,
        p: Point,
        r: &Ray,
        u: f64,
        v: f64,
        mat: Rc<dyn Material>,
        outward_normal: Vec3,
    ) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;

        Self {
            t,
            p,
            u,
            v,
            material: mat,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    // bounding_box return AABB containing the object.
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}
