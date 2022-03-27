use std::{cmp::Ordering, rc::Rc};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
};

// BVH stays for Bounded Volume Hierarchy.
pub struct BVHNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub out_box: AABB,
}

impl BVHNode {
    pub fn from_hittable_list(mut hlist: HittableList, time0: f64, time1: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(&mut rng, hlist.as_slice_mut(), time0, time1)
    }

    pub fn new(rng: &mut ThreadRng, lst: &mut [Rc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let comparator = match rng.gen_range(0..=2) as u32 {
            0 => Self::compare::<0>,
            1 => Self::compare::<1>,
            _ => Self::compare::<2>,
        };

        let (left, right) = if lst.len() == 1 {
            (lst[0].clone(), lst[0].clone())
        } else if lst.len() == 2 {
            if comparator(&lst[0], &lst[1]).is_lt() {
                (lst[0].clone(), lst[1].clone())
            } else {
                (lst[1].clone(), lst[0].clone())
            }
        } else {
            lst.sort_by(comparator);
            let (l, r) = lst.split_at_mut(lst.len() / 2);
            (
                Rc::new(Self::new(rng, l, time0, time1)) as Rc<dyn Hittable>,
                Rc::new(Self::new(rng, r, time0, time1)) as Rc<dyn Hittable>,
            )
        };

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        Self {
            left,
            right,
            out_box: AABB::union(&box_left.unwrap(), &box_right.unwrap()),
        }
    }

    pub fn compare<const AXIS: usize>(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0).unwrap();
        let box_b = b.bounding_box(0.0, 0.0).unwrap();
        if box_a.min[AXIS] < box_b.min[AXIS] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.out_box.hit(r, t_min, t_max) {
            return None;
        }
        match self.left.hit(r, t_min, t_max) {
            None => self.right.hit(r, t_min, t_max),
            Some(hr_left) => match self.right.hit(r, t_min, hr_left.t) {
                None => Some(hr_left),
                opt_right => opt_right,
            },
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.out_box.clone())
    }
}
