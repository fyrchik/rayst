use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn as_slice_mut(&mut self) -> &mut [Rc<dyn Hittable>] {
        &mut self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit = t_max;
        let mut rec: Option<HitRecord> = None;

        for obj in self.objects.iter() {
            if let Some(temp_rec) = obj.hit(r, t_min, closest_hit) {
                closest_hit = temp_rec.t;
                rec = Some(temp_rec.clone());
            }
        }

        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut out_box: Option<AABB> = None;

        for obj in self.objects.iter() {
            if let Some(temp_box) = obj.bounding_box(time0, time1) {
                out_box = match out_box {
                    None => Some(temp_box),
                    Some(b) => Some(AABB::union(&b, &temp_box)),
                };
                continue;
            }
            return None;
        }

        out_box
    }
}
