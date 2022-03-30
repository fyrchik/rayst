use std::rc::Rc;

use crate::{
    aabb::AABB,
    aarect::{XYRect, XZRect, YZRect},
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::Material,
    ray::Ray,
    vec3::Point,
};

pub struct Box3D {
    pub min: Point,
    pub max: Point,
    pub sides: HittableList,
}

impl Box3D {
    pub fn new(p0: Point, p1: Point, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::default();

        sides.add(Rc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            material.clone(),
        )));
        sides.add(Rc::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            material.clone(),
        )));

        sides.add(Rc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            material.clone(),
        )));
        sides.add(Rc::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            material.clone(),
        )));

        sides.add(Rc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            material.clone(),
        )));
        sides.add(Rc::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            material.clone(),
        )));

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for Box3D {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
}
