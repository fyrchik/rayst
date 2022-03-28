use std::f64::consts::PI;
use std::ops::Neg;
use std::rc::Rc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub(crate) fn get_uv(p: &Point) -> (f64, f64) {
        let theta = p.y.neg().acos();
        let phi = p.z.neg().atan2(p.x) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius.powi(2);

        let d = half_b.powi(2) - a * c;
        if d < 0.0 {
            return None;
        }

        let sqrtd = d.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (u, v) = Self::get_uv(&outward_normal);

        Some(HitRecord::new(
            root,
            p,
            r,
            u,
            v,
            self.material.clone(),
            outward_normal,
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new_eq(self.radius),
            self.center + Vec3::new_eq(self.radius),
        ))
    }
}
