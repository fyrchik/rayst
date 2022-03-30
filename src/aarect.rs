use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct XYRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mat: Rc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mat,
        }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point::new(self.x0, self.y0, self.k - 0.0001),
            Point::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t_max < t {
            return None;
        }

        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || self.x1 < x || y < self.y0 || self.y1 < y {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        Some(HitRecord::new(
            t,
            r.at(t),
            r,
            u,
            v,
            self.mat.clone(),
            Vec3::z(1.0),
        ))
    }
}

pub struct XZRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat: Rc<dyn Material>,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for XZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point::new(self.x0, self.k - 0.0001, self.z0),
            Point::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t_max < t {
            return None;
        }

        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if x < self.x0 || self.x1 < x || z < self.z0 || self.z1 < z {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord::new(
            t,
            r.at(t),
            r,
            u,
            v,
            self.mat.clone(),
            Vec3::y(1.0),
        ))
    }
}

pub struct YZRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat: Rc<dyn Material>,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for YZRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point::new(self.k - 0.0001, self.y0, self.z0),
            Point::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t_max < t {
            return None;
        }

        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if y < self.y0 || self.y1 < y || z < self.z0 || self.z1 < z {
            return None;
        }

        let v = (y - self.y0) / (self.y1 - self.y0);
        let u = (z - self.z0) / (self.z1 - self.z0);

        Some(HitRecord::new(
            t,
            r.at(t),
            r,
            u,
            v,
            self.mat.clone(),
            Vec3::x(1.0),
        ))
    }
}
