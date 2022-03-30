use std::rc::Rc;

use rand::Rng;

use crate::{
    aabb::AABB,
    color::Color,
    hittable::{HitRecord, Hittable},
    material::{Isotropic, Material},
    ray::Ray,
    texture::Texture,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase: Rc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary,
            phase: Rc::new(Isotropic::new(c)),
            neg_inv_density: -d.recip(),
        }
    }
    pub fn new_with_texture(boundary: Rc<dyn Hittable>, d: f64, texture: Rc<dyn Texture>) -> Self {
        Self {
            boundary,
            phase: Rc::new(Isotropic::new_with_texture(texture)),
            neg_inv_density: -d.recip(),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        let enable_debug = false;
        let debugging = enable_debug && rng.gen::<f64>() < 0.00001;

        match self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY) {
            None => None,
            Some(mut rec) => match self.boundary.hit(r, rec.t + 0.0001, f64::INFINITY) {
                None => None,
                Some(rec2) => {
                    if debugging {
                        eprintln!("t_min={}, t_max={}", rec.t, rec2.t);
                    }

                    let r1t = rec.t.max(t_min);
                    let r2t = rec2.t.min(t_max);
                    if r1t >= r2t {
                        return None;
                    }

                    rec.t = r1t.max(0.0);

                    let ray_length = r.dir.length();
                    let distance_inside_boundary = (r2t - rec.t) * ray_length;
                    let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();
                    if hit_distance > distance_inside_boundary {
                        return None;
                    }

                    rec.t += hit_distance / ray_length;
                    rec.p = r.at(rec.t);
                    if debugging {
                        eprintln!(
                            "hit_distance={}\nrec.t={}\nrec.p={}\n",
                            hit_distance, rec.t, rec.p
                        );
                    }

                    rec.normal = Vec3::x(1.0);
                    rec.front_face = true;
                    rec.material = self.phase.clone();

                    Some(rec)
                }
            },
        }
    }
}
