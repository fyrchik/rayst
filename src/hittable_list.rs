use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};

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
}
