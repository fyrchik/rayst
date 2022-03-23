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
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_hit = t_max;
        let mut temp_rec = HitRecord::default();

        for obj in self.objects.iter() {
            if obj.hit(r, t_min, closest_hit, &mut temp_rec) {
                hit_anything = true;
                closest_hit = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
