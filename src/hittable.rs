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

pub struct Translate {
    pub object: Rc<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.object
            .bounding_box(time0, time1)
            .map(|obj| AABB::new(obj.min + self.offset, obj.max + self.offset))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved = Ray::new(r.orig - self.offset, r.dir, r.time);
        self.object.hit(&moved, t_min, t_max).map(|mut rec| {
            rec.p += self.offset;
            rec.set_face_normal(&moved, rec.normal);
            rec
        })
    }
}

pub struct RotateY {
    pub object: Rc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let r = angle.to_radians();
        let sin_theta = r.sin();
        let cos_theta = r.cos();
        let bbox = object.bounding_box(0.0, 1.0);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: bbox.map(|inner| {
                let mut min = Vec3::new_eq(f64::INFINITY);
                let mut max = Vec3::new_eq(f64::NEG_INFINITY);

                min.y = inner.min.y;
                max.y = inner.max.y;
                for x in [inner.min.x, inner.max.x] {
                    for z in [inner.min.z, inner.max.z] {
                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        min.x = min.x.min(new_x);
                        min.z = min.z.min(new_z);

                        max.x = max.x.max(new_x);
                        max.z = max.z.max(new_z);
                    }
                }

                AABB::new(min, max)
            }),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.bbox.clone()
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ox = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        let oz = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;

        let dx = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        let dz = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;

        let rotated = Ray::new(
            Vec3::new(ox, r.orig.y, oz),
            Vec3::new(dx, r.dir.y, dz),
            r.time,
        );

        self.object.hit(&rotated, t_min, t_max).map(|mut rec| {
            let px = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            let pz = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
            let nx = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            let nz = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            rec.p.x = px;
            rec.p.z = pz;
            rec.set_face_normal(&rotated, Vec3::new(nx, rec.normal.y, nz));

            rec
        })
    }
}
