use std::{ops::Range, rc::Rc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point, Vec3},
};

pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub time: Range<f64>,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point,
        center1: Point,
        time: Range<f64>,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time,
            radius,
            material,
        }
    }

    pub fn center_at(&self, t: f64) -> Point {
        self.center0
            + ((t - self.time.start) / (self.time.end - self.time.start))
                * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center_at(r.time);
        let oc = r.orig - center;
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
        let outward_normal = (p - center) / self.radius;
        let (u, v) = Sphere::get_uv(&outward_normal);

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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center_at(time0) - Vec3::new_eq(self.radius),
            self.center_at(time0) + Vec3::new_eq(self.radius),
        );
        let box1 = AABB::new(
            self.center_at(time1) - Vec3::new_eq(self.radius),
            self.center_at(time1) + Vec3::new_eq(self.radius),
        );
        Some(AABB::union(&box0, &box1))
    }
}
